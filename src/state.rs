use cosmwasm_std::{Uint64};
use cw_storage_plus::Item;

pub const COUNTER: Item<Uint64> = Item::new("counter");
