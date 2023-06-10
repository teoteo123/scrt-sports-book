use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use secret_toolkit::storage::{Item, Keymap};

use cosmwasm_std::{Addr, Uint128};
pub static CONFIG_KEY: &[u8] = b"config";
pub const PREFIX_BALANCES: &[u8] = b"balances";

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct State {
    pub admin: Addr,
    pub payout_address: Addr,
    pub current_round: Round,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct Round {
    pub id: String,
    pub game: String,
    pub odds: Uint128,
}

impl Default for Round {
    fn default() -> Self {
        Round {
            id: "0".to_string(),
            game: "N/a".to_string(),
            odds: Uint128::from(500000u128)
        }
    }
}

pub const USER_BALANCE: Item<Uint128> = Item::new(b"user balance");
pub const CURRENT_ROUND: Item<Round> = Item::new(b"current round");
pub static BETS: Keymap<String, Uint128> = Keymap::new(b"bets");
pub const DEPOSIT_DENOM: Item<String> = Item::new(b"deposit denom");
pub const ADMIN: Item<Addr> = Item::new(b"admin");
pub const FEE_POOL_BALANCE: Item<Uint128> = Item::new(b"fee pool balance");
pub const BETTING_OPEN: Item<bool> = Item::new(b"betting open");