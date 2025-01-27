use crate::{BookPrice, BookQuantity, OrderCount, BookYield};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default, PartialEq)]
pub struct LevelSnapshot {
    pub order_count: Option<OrderCount>,
    pub book_price: BookPrice,
    pub book_quantity: BookQuantity,
    pub book_yield: Option<BookYield>,
    pub lp_quantity: Option<BookQuantity>,
}