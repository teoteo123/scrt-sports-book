use cosmwasm_std::{Addr, Uint128};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct InstantiateMsg {  }

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    OpenRound {
        id: String,
        game: String,
        odds: Uint128,
    },
    CloseRound { winner: String },
    Deposit {  },
    PlaceBet { amount: Uint128 },
    Withdraw {
        amount: Uint128,
    },
    WithdrawFees {  }
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    CurrentRound {  },
    BettingOpen {  },
    Balance { address: Addr },
}
