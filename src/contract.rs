use cosmwasm_std::{entry_point, to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Coin, Uint128, BankMsg, Addr};
use crate::err::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::payment::must_pay;
use crate::state::{ADMIN, BETS, BETTING_OPEN, CURRENT_ROUND, DEPOSIT_DENOM, FEE_POOL_BALANCE, Round, USER_BALANCE};

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _msg: InstantiateMsg,
) -> StdResult<Response> {

    // initialize state
    CURRENT_ROUND.save(deps.storage, &Round::default())?;
    ADMIN.save(deps.storage, &info.sender)?;
    DEPOSIT_DENOM.save(deps.storage, &"uscrt".to_string())?;
    FEE_POOL_BALANCE.save(deps.storage, &Uint128::from(0u128))?;
    BETTING_OPEN.save(deps.storage, &false)?;

    deps.api
        .debug(format!("Contract was initialized by {}", info.sender).as_str());

    Ok(Response::default())
}

#[entry_point]
pub fn execute(deps: DepsMut, _env: Env, info: MessageInfo, msg: ExecuteMsg) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::PlaceBet { amount } => place_bet(deps, info, amount),
        ExecuteMsg::OpenRound { id, game, odds } => open_round(deps, info, id, game, odds),
        ExecuteMsg::CloseRound { winner } => close_round(deps, info, winner),
        ExecuteMsg::Deposit {  } => deposit(deps, info),
        ExecuteMsg::Withdraw { amount } => withdraw(deps, info, amount),
        ExecuteMsg::WithdrawFees {  } => withdraw_fees(deps)
    }
}

// makes sure user has balance necessary to bet specified amount and pay a 2% fee then updates their balance accordingly
fn place_bet(deps: DepsMut, info: MessageInfo, amount: Uint128) -> Result<Response, ContractError> {
    let user_balance = USER_BALANCE
        .add_suffix(info.sender.as_bytes());
    let balance_exists = user_balance.load(deps.storage);
    match balance_exists {
        Ok(balance) => {
            // 2% fee
            if balance < amount + amount / Uint128::from(50u128) {
                return Err(ContractError::InsufficientBalance { amount, balance })
            }
            user_balance.update(deps.storage, |prev| Ok(prev - amount - amount / Uint128::from(50u128)))?;
            FEE_POOL_BALANCE.update(deps.storage, |prev| Ok(prev + amount / Uint128::from(50u128)))?;
            let current_round = CURRENT_ROUND.load(deps.storage)?;
            BETS
                .add_suffix(info.sender.as_bytes())
                .insert(deps.storage, &current_round.id, &amount)?;
            Ok(Response::default())
        },
        Err(_) => Err(ContractError::Deposit {msg: "No balance".to_string()})
    }
}

// opens a new round of betting, updates current round
fn open_round(deps: DepsMut, _info: MessageInfo, id: String, game: String, odds: Uint128) -> Result<Response, ContractError> {
    // todo: update payouts for previous round bets
    CURRENT_ROUND.save(deps.storage, &Round {id, game, odds})?;

    Ok(Response::default())
}

// closes the current round and sets CURRENT_ROUND to default
fn close_round(deps: DepsMut, info: MessageInfo, _winner: String) -> Result<Response, ContractError> {
    //check for admin privileges
    let admin = ADMIN.load(deps.storage)?;
    if info.sender != admin {
        return Err(ContractError::Unauthorized { sender: info.sender });
    }

    //re-write round to default
    let default_round = Round::default();

    CURRENT_ROUND.save(deps.storage, &default_round)?;
    Ok(Response::default())
}

// accepts only uscrt and updates user's balance based on the mount deposited
fn deposit(deps: DepsMut, info: MessageInfo) -> Result<Response, ContractError> {
    let user_balance = USER_BALANCE.add_suffix(info.sender.as_bytes());
    match user_balance.load(deps.storage) {
        Err(_) => {
            user_balance.save(deps.storage, &Uint128::from(0u128))?;
        }
        _ => {}
    };
    let deposit_amount = must_pay(&info, "uscrt")?;
    user_balance.update(deps.storage, |bal| Ok(bal + deposit_amount))?;
    Ok(Response::default())
}

// withdraw specified amount in uscrt from sender's balance
fn withdraw(deps: DepsMut, info: MessageInfo, amount: Uint128) -> Result<Response, ContractError> {
    let user_balance = USER_BALANCE
        .add_suffix(info.sender.as_bytes());
    let balance_exists = user_balance
        .load(deps.storage);
    match balance_exists {
        Ok(balance) => {
            if balance < amount {
                return Err(ContractError::InsufficientBalance { amount, balance });
            };
            user_balance.update(deps.storage, |prev| Ok(prev - amount))?;
            let withdrawal_message = BankMsg::Send { to_address: info.sender.into(), amount: vec![Coin { denom: "uscrt".into(), amount: balance }] };
            Ok(Response::new()
                .add_message(withdrawal_message)
                .add_attribute("action", "withdraw")
                .add_attribute("amount", amount.to_string())
            )
        },
        Err(_) => Err(ContractError::Deposit {msg: "No balance".to_string()})
    }
}

// allows only the admin to withdraw all of the money from the fee pool
fn withdraw_fees(deps: DepsMut) -> Result<Response, ContractError> {
    let fee_balance = FEE_POOL_BALANCE.load(deps.storage)?;
    let message = BankMsg::Send { to_address: ADMIN.load(deps.storage)?.into(), amount: vec![Coin { denom: "uscrt".into(), amount: fee_balance }] };
    Ok(
        Response::new()
        .add_message(message)
        .add_attribute("action", "fee collection")
        .add_attribute("amount", fee_balance)
    )
}


#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::CurrentRound { .. } => query_round(deps),
        QueryMsg::BettingOpen { .. } => betting_open(deps),
        QueryMsg::Balance { address } => query_balance(deps, address)
    }
}

// queries
fn query_round(deps: Deps) -> StdResult<Binary> {
    let result = CURRENT_ROUND.load(deps.storage)?;
    to_binary(&result)
}

fn betting_open(deps: Deps) -> StdResult<Binary> {
    let betting_open = BETTING_OPEN.load(deps.storage)?;
    to_binary(&betting_open)
}

fn query_balance(deps: Deps, address: Addr) -> StdResult<Binary> {
    let user_balance = USER_BALANCE
        .add_suffix(address.as_bytes())
        .load(deps.storage)?;
    to_binary(&user_balance)
}
