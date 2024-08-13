use cosmwasm_std::Uint64;
use cosmwasm_schema::cw_serde;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
//Equivalent to cw_serde


#[cw_serde]
pub enum ExecuteMsg {
    Increment { },
    Decrement { }
}

#[cw_serde]
pub enum QueryMsg {
    GetCounter { }
}

#[cw_serde]
pub struct QueryCounter {
    pub(crate) counter: Uint64
}