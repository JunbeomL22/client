use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default, Hash, Eq, PartialOrd, Ord, Copy)]
pub enum OrderSide {
    #[default]
    Bid,
    Ask,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
pub enum OrderStatus {
    PendingNew,
    Accepted,
    PartiallyFilled,
    FullyFilled,
    Canceled,
    Rejected,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum OrderType {
    Limit,
    Market,
    Cancel,
    Modify,
    RemoveOther,
    Null,
}