use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Coin, Timestamp};

#[cw_serde]
pub struct InstantiateMsg {
    pub coin_threshold: Coin,
    pub deadline: Timestamp,
    pub receiver: Option<String>, // contract creator if None
}

#[cw_serde]
pub enum ExecuteMsg {
    /// User's contribution.
    ContributionMsg {
        coin: Coin,
    },
    /// Trigger the resolution of the market. (TBD not sure about 'market' terminology).
    ResolveMsg {},
    RefundMsg {},
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(ContributionResponse)]
    GetUserContribution { addr: String },
    #[returns(ContributionTotalResponse)]
    GetTotalContribution {},
    #[returns(DeadlineResponse)]
    GetDeadline {},
}

#[cw_serde]
pub struct ContributionResponse {
    pub coin: Coin,
}

#[cw_serde]
pub struct ContributionTotalResponse {
    pub cointotal: Coin,
}

#[cw_serde]
pub struct DeadlineResponse {
    pub timestamp: Timestamp,
}

#[cw_serde]
pub struct ReceiverResponse {
    pub receiver: Addr, // TBD or should it be String?
}
