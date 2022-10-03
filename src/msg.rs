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
    ContributionMsg {},
    /// Trigger the resolution of the market. (TBD not sure about 'market' terminology).
    ResolveMsg {},
    RefundMsg {},
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(GetContribution)]
    GetUserContribution { addr: String },
    #[returns(GetContributionTotal)]
    GetTotalContribution {},
    #[returns(GetDeadline)]
    GetDeadline {},
}

// TBD Get* structs here, and further structs in query ?

#[cw_serde]
pub struct GetContribution {
    pub coin: Coin,
}

#[cw_serde]
pub struct GetContributionTotal {
    pub cointotal: Coin,
}

#[cw_serde]
pub struct GetDeadline {
    pub timestamp: Timestamp,
}

#[cw_serde]
pub struct GetReceiver {
    pub receiver: Addr, // TBD or should it be String?
}
