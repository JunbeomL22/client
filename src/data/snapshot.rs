use crate::{BookQuantity, TimeStamp};
use crate::data::level::LevelSnapshot;
use crate::InstId;
use serde::Serialize;

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct QuoteSnapshot {
    pub id: InstId,
    //
    pub datatime: TimeStamp,
    pub systemtime: TimeStamp,
    //
    pub ask_quote_data: Vec<LevelSnapshot>,
    pub bid_quote_data: Vec<LevelSnapshot>,
    pub quote_level_cut: usize, // this value indicates how many levels of order data are actually used. This can be less than the length of ask_order_data and bid_order_data
    //
    pub all_lp_holdings: Option<BookQuantity>,
}

impl QuoteSnapshot {
    pub fn sample(level: usize) -> Self {
        let mut ask_quote_data = Vec::with_capacity(level);
        let mut bid_quote_data = Vec::with_capacity(level);
        for _ in 0..level {
            ask_quote_data.push(LevelSnapshot::default());
            bid_quote_data.push(LevelSnapshot::default());
        }
        Self {
            id: InstId::default(),
            datatime: TimeStamp::default(),
            systemtime: TimeStamp::default(),
            ask_quote_data,
            bid_quote_data,
            quote_level_cut: level,
            all_lp_holdings: None,
        }
    }
}