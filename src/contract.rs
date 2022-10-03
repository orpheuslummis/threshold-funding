use cosmwasm_schema::cw_serde;
#[cfg(not(feature = "library"))]
use cosmwasm_std::{
    entry_point, to_binary, BankMsg, Binary, Coin, CosmosMsg, Deps, DepsMut, Env, MessageInfo,
    Order, Response, StdResult, Timestamp, Uint128,
};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{CONTRIBUTIONS, DEADLINE, RECEIVER, THRESHOLD_COIN};

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
    RECEIVER.save(
        deps.storage,
        &msg.receiver.unwrap_or_else(|| info.sender.to_string()),
    )?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender)
        .add_attribute("deadline-seconds", msg.deadline.seconds().to_string())
        .add_attribute("threshold-amount", msg.coin_threshold.amount.to_string())
        .add_attribute("threshold-denom", msg.coin_threshold.denom.to_string()))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::ContributionMsg {} => execute::contribution(deps, env, info, msg),
        ExecuteMsg::RefundMsg {} => execute::refund(deps, env, info, msg),
        ExecuteMsg::ResolveMsg {} => execute::resolve(deps, env, info, msg),
    }
}

pub mod execute {
    use super::*;

    /// TBD spec
    pub fn contribution(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        _: ExecuteMsg,
    ) -> Result<Response, ContractError> {
        let threshold_coin = THRESHOLD_COIN.load(deps.storage)?;
        let deadline = DEADLINE.load(deps.storage)?;

        if env.block.time > deadline {
            return Err(ContractError::DeadlinePassed {});
        }

        let user = deps.api.addr_validate(info.sender.as_ref())?;

        // TBD check if denom is correct
        if info.funds.len() != 1 || info.funds[0].denom != threshold_coin.denom {
            return Err(ContractError::CustomError { val: String::new() });
        }

        let amount = info.funds[0].amount;

        CONTRIBUTIONS.update(deps.storage, &user, |old| -> StdResult<_> {
            match old {
                Some(old) => Ok(old + amount),
                None => Ok(amount),
            }
        })?;

        Ok(Response::new()
            .add_attribute("method", "contribution")
            .add_attribute("contributor", info.sender)
            .add_attribute("amount", amount.to_string()))
    }

    // refund a single user's contribution
    // only valid before deadline
    pub fn refund(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        _: ExecuteMsg,
    ) -> Result<Response, ContractError> {
        let deadline = DEADLINE.load(deps.storage)?;
        let threshold_coin = THRESHOLD_COIN.load(deps.storage)?;

        if env.block.time > deadline {
            return Err(ContractError::DeadlinePassed {});
        }

        let user = deps.api.addr_validate(info.sender.as_ref())?;

        let amount = CONTRIBUTIONS
            .may_load(deps.storage, &user)?
            .unwrap_or_default();

        if amount.is_zero() {
            return Err(ContractError::CustomError { val: String::new() });
        }

        CONTRIBUTIONS.remove(deps.storage, &user);

        Ok(Response::new()
            .add_attribute("method", "refund")
            .add_attribute("contributor", info.sender)
            .add_attribute("amount", amount.to_string())
            .add_message(CosmosMsg::Bank(BankMsg::Send {
                to_address: user.into_string(),
                amount: vec![Coin {
                    denom: threshold_coin.denom,
                    amount,
                }],
            })))
    }

    /// if threshold isn't reached, refund all contributions
    /// if threshold is reached, send all funds to the receiver
    /// can be called anytime
    pub fn resolve(
        deps: DepsMut,
        _: Env,
        _: MessageInfo,
        _: ExecuteMsg,
    ) -> Result<Response, ContractError> {
        let threshold_coin = THRESHOLD_COIN.load(deps.storage)?;
        let receiver = RECEIVER.load(deps.storage)?;

        let total_contributions = CONTRIBUTIONS
            .range(deps.storage, None, None, Order::Ascending)
            .map(|item| item.unwrap().1)
            .fold(Uint128::zero(), |acc, x| acc + x);

        if total_contributions < threshold_coin.amount {
            // refund all contributions
            let mut res = Response::new()
                .add_attribute("method", "resolve")
                .add_attribute("status", "refund");

            for item in CONTRIBUTIONS
                .range(deps.storage, None, None, Order::Ascending)
                .map(|item| item.unwrap())
            {
                let (addr, amount) = item;
                res = res.add_message(CosmosMsg::Bank(BankMsg::Send {
                    to_address: addr.to_string(),
                    amount: vec![Coin {
                        denom: threshold_coin.denom.clone(),
                        amount,
                    }],
                }));
            }

            Ok(res)
        } else {
            // send all funds to the receiver
            Ok(Response::new()
                .add_attribute("method", "resolve")
                .add_attribute("status", "send")
                .add_message(CosmosMsg::Bank(BankMsg::Send {
                    to_address: receiver,
                    amount: vec![Coin {
                        denom: threshold_coin.denom,
                        amount: total_contributions,
                    }],
                })))
        }
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetUserContribution { addr } => {
            to_binary(&query::usercontribution(deps, env, addr)?)
        }
        QueryMsg::GetTotalContribution {} => to_binary(&query::totalcontribution(deps, env)?),
        QueryMsg::GetDeadline {} => to_binary(&query::deadline(deps, env)?),
    }
}

pub mod query {
    use super::*;

    // TBD why option and not just the unboxed type?

    #[cw_serde]
    pub struct ContributionResponse {
        pub amount: Option<Uint128>,
    }

    #[cw_serde]
    pub struct DeadlineResponse {
        pub timestamp: Option<Timestamp>,
    }

    pub fn usercontribution(
        deps: Deps,
        _env: Env,
        addr: String,
    ) -> StdResult<ContributionResponse> {
        let user = deps.api.addr_validate(&addr)?;
        let contribution = CONTRIBUTIONS.may_load(deps.storage, &user)?;
        Ok(ContributionResponse {
            amount: contribution,
        })
    }

    pub fn totalcontribution(deps: Deps, _env: Env) -> StdResult<ContributionResponse> {
        let contributions = CONTRIBUTIONS
            .range(deps.storage, None, None, Order::Ascending)
            .map(|item| item.map(|(_, v)| v))
            .fold(Uint128::zero(), |acc, item| acc + item.unwrap());
        Ok(ContributionResponse {
            amount: Some(contributions),
        })
    }

    pub fn deadline(deps: Deps, _env: Env) -> StdResult<DeadlineResponse> {
        let timestamp = DEADLINE.may_load(deps.storage)?;
        Ok(DeadlineResponse { timestamp })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};

    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies();

        let msg_no_receiver = InstantiateMsg {
            coin_threshold: Coin {
                denom: "OSMO".to_string(),
                amount: Uint128::from(10_000_000u128),
            },
            deadline: Timestamp::from_seconds(10),
            receiver: None,
        };
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

    #[test]
    fn query_usercontribution() {
        let mut deps = mock_dependencies();

        let msg_no_receiver = InstantiateMsg {
            coin_threshold: Coin {
                denom: "OSMO".to_string(),
                amount: Uint128::from(10_000_000u128),
            },
            deadline: Timestamp::from_seconds(10),
            receiver: None,
        };
        let info = mock_info("creator", &[]);

        instantiate(deps.as_mut(), mock_env(), info, msg_no_receiver).unwrap();

        // TODO
    }

    // TODO test query::totalcontribution

    // TODO test query:deadline

    // TODO test execute::contribute

    // TODO test execute::refund

    // TODO test execute::resolve

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
