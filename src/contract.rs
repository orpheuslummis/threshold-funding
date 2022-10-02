#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{THRESHOLD_COIN, DEADLINE, RECEIVER};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:threshold-funding";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    
    THRESHOLD_COIN.save(deps.storage, &msg.coin_threshold)?;
    DEADLINE.save(deps.storage, &msg.deadline)?;
    RECEIVER.save(deps.storage, &msg.receiver.unwrap_or(info.sender.to_string()))?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender)
        .add_attribute("deadline-seconds", msg.deadline.seconds().to_string())
        .add_attribute("threshold-amount", msg.coin_threshold.amount.to_string())
        .add_attribute("threshold-denom", msg.coin_threshold.denom.to_string()))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Increment {} => Err(ContractError::CustomError { val: "".to_string() }),
        ExecuteMsg::Reset { count } => Err(ContractError::CustomError { val: "".to_string() }),
    }
}

pub mod execute {
    // use super::*;
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {
    // match msg {
    //     QueryMsg::GetCount {} => to_binary(&query::count(deps)?),
    // }
    Err(cosmwasm_std::StdError::GenericErr { msg: "".to_string() })
}

pub mod query {
    // use super::*;
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{Coin, Timestamp, Uint128};

    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies();

        let msg_no_receiver = InstantiateMsg { coin_threshold: Coin { denom: "OSMO".to_string(), amount: Uint128::from(10_000_000u128) }, deadline: Timestamp::from_seconds(10), receiver: None};
        let info = mock_info("creator", &[]);

        // we can just call .unwrap() to assert this was a success
        let res = instantiate(deps.as_mut(), mock_env(), info, msg_no_receiver).unwrap();
        assert_eq!(0, res.messages.len());
        assert_eq!(5, res.attributes.len());

        let init_threshold_coin = THRESHOLD_COIN.load(&deps.storage).unwrap();
        assert_eq!("OSMO", init_threshold_coin.denom);
        assert_eq!(Uint128::from(10_000_000u128), init_threshold_coin.amount);

        let init_deadline = DEADLINE.load(&deps.storage).unwrap();
        assert_eq!(Timestamp::from_seconds(10), init_deadline);

        let init_receiver_none = RECEIVER.load(&deps.storage).unwrap();
        assert_eq!("creator", init_receiver_none);

        // it worked, let's query the state
        // let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
        // let value: GetCountResponse = from_binary(&res).unwrap();
        // assert_eq!(17, value.count);
    }

    // #[test]
    // fn increment() {
    //     let mut deps = mock_dependencies();

    //     // let msg = InstantiateMsg { count: 17 };
    //     let info = mock_info("creator", &coins(2, "token"));
    //     let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

    //     // beneficiary can release it
    //     let info = mock_info("anyone", &coins(2, "token"));
    //     let msg = ExecuteMsg::Increment {};
    //     let _res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

    //     // should increase counter by 1
    //     let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
    //     let value: GetCountResponse = from_binary(&res).unwrap();
    //     assert_eq!(18, value.count);
    // }

    // #[test]
    // fn reset() {
    //     let mut deps = mock_dependencies();

    //     let msg = InstantiateMsg { count: 17 };
    //     let info = mock_info("creator", &coins(2, "token"));
    //     let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

    //     // beneficiary can release it
    //     let unauth_info = mock_info("anyone", &coins(2, "token"));
    //     let msg = ExecuteMsg::Reset { count: 5 };
    //     let res = execute(deps.as_mut(), mock_env(), unauth_info, msg);
    //     match res {
    //         Err(ContractError::Unauthorized {}) => {}
    //         _ => panic!("Must return unauthorized error"),
    //     }

    //     // only the original creator can reset the counter
    //     let auth_info = mock_info("creator", &coins(2, "token"));
    //     let msg = ExecuteMsg::Reset { count: 5 };
    //     let _res = execute(deps.as_mut(), mock_env(), auth_info, msg).unwrap();

    //     // should now be 5
    //     let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
    //     let value: GetCountResponse = from_binary(&res).unwrap();
    //     assert_eq!(5, value.count);
    // }
}
