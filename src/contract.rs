use cosmwasm_std::{Binary, Deps, DepsMut, Empty, Env, Event, MessageInfo, Response, StdResult, to_json_binary, Uint64};
use crate::msg::{ExecuteMsg, QueryMsg};
use crate::state::COUNTER;

pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: Empty,
) -> StdResult<Response> {
    COUNTER.save(deps.storage, &Uint64::zero())?;
    let event = Event::new("Initiated contract lol xd").add_attribute("xd", "lol");
    Ok(Response::new().add_event(event).add_attribute("Initial value", COUNTER.load(deps.storage)?))

}

pub fn execute(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> StdResult<Response> {
    match msg {
        ExecuteMsg::Increment {  } => exec::increment(deps),
        ExecuteMsg::Decrement {  } => exec::decrement(deps),
    }
}

mod exec {
    use cosmwasm_std::{DepsMut, Event, Response, StdResult, Uint64};
    use crate::state::COUNTER;

    pub fn increment(deps: DepsMut) -> StdResult<Response> {
        let mut curr_counter = COUNTER.load(deps.storage).unwrap();
        curr_counter = curr_counter + Uint64::one();
        COUNTER.save(deps.storage, &curr_counter).expect("Could not be increased.");
        let event = Event::new("Incremented counter by one.").add_attribute("counter", curr_counter);
        Ok(Response::new().add_event(event).add_attribute("action", "decrement"))
    }

    pub fn decrement(deps: DepsMut) -> StdResult<Response> {
        let mut curr_counter = COUNTER.load(deps.storage).unwrap();
        curr_counter = curr_counter - Uint64::one();
        COUNTER.save(deps.storage, &curr_counter).expect("Could not be decreased");
        let event = Event::new("Decremented counter by one.").add_attribute("counter", curr_counter);
        //curr_counter = Uint64::zero(); // To check this in normal file, non cosmwasm
        Ok(Response::new().add_event(event).add_attribute("action", "decrement"))
    }
}

pub fn query(
    deps: Deps,
    _env: Env,
    msg: QueryMsg
) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetCounter { } => to_json_binary(&query::get_counter(deps).unwrap())
    }
}


mod query {
    use cosmwasm_std::{Deps, StdResult,};
    use crate::msg::QueryCounter;
    use crate::state::COUNTER;

    pub fn get_counter(deps: Deps) -> StdResult<QueryCounter> {
        return Ok(QueryCounter { counter: COUNTER.load(deps.storage)? });
    }
}