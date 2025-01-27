use crate::{BookPrice, BookQuantity, OrderId};
use crate::order::enums::OrderSide;
//
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CoreType {
    LimitOrder,
    MarketOrder,
    CancelOrder,
    ModifyOrder,
    RemoveOtherOrder,
    NullOrder,
}

impl std::fmt::Display for CoreType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            CoreType::LimitOrder => write!(f, "LimitOrder"),
            CoreType::MarketOrder => write!(f, "MarketOrder"),
            CoreType::CancelOrder => write!(f, "CancelOrder"),
            CoreType::ModifyOrder => write!(f, "ModifyOrder"),
            CoreType::RemoveOtherOrder => write!(f, "RemoveOtherOrder"),
            CoreType::NullOrder => write!(f, "NullOrder"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct LimitOrder {
    pub price: BookPrice,
    pub quantity: BookQuantity,
    pub order_side: OrderSide, // should I keep this? book also has its side
    pub order_id: OrderId,
}

impl LimitOrder {
    #[inline]
    pub fn new(
        price: BookPrice,
        quantity: BookQuantity,
        order_side: OrderSide,
        order_id: OrderId,
    ) -> Self {
        Self {
            price,
            quantity,
            order_side,
            order_id,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct MarketOrder {
    pub quantity: BookQuantity,
    pub order_side: OrderSide,
    pub order_id: OrderId,
}

impl MarketOrder {
    #[inline]
    pub fn new(quantity: BookQuantity, order_side: OrderSide, order_id: OrderId) -> Self {
        Self {
            quantity,
            order_side,
            order_id,
        }
    }
}

/// Normally, it is used for my order
#[derive(Debug, Clone, Serialize, Deserialize, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct CancelOrder {
    pub order_id: OrderId,
}

impl CancelOrder {
    #[inline]
    pub fn new(order_id: OrderId) -> Self {
        Self { order_id }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ModifyOrder {
    pub order_id: OrderId,
    pub price: BookPrice,
    pub quantity: BookQuantity,
}

impl ModifyOrder {
    #[inline]
    pub fn new(order_id: OrderId, price: BookPrice, quantity: BookQuantity) -> Self {
        Self {
            order_id,
            price,
            quantity,
        }
    }
}

/// If you want to remove order in a different ratio, you can use ModifyOrder
#[derive(Debug, Clone, Serialize, Deserialize, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct RemoveOtherOrder {
    pub price: BookPrice,
    pub quantity: BookQuantity,
    pub order_side: OrderSide,
}

impl RemoveOtherOrder {
    #[inline]
    pub fn new(price: BookPrice, quantity: BookQuantity, order_side: OrderSide) -> Self {
        Self {
            price,
            quantity,
            order_side,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct NullOrder {}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum OrderCore {
    NullOrder(NullOrder),
    LimitOrder(LimitOrder),
    RemoveOtherOrder(RemoveOtherOrder), 
    MarketOrder(MarketOrder),
    CancelOrder(CancelOrder), 
    ModifyOrder(ModifyOrder),
}

impl Default for OrderCore {
    fn default() -> Self {
        OrderCore::NullOrder(NullOrder {})
    }
}

impl OrderCore {
    #[inline]
    pub fn core_type(&self) -> CoreType {
        match self {
            OrderCore::LimitOrder(_) => CoreType::LimitOrder,
            OrderCore::MarketOrder(_) => CoreType::MarketOrder,
            OrderCore::CancelOrder(_) => CoreType::CancelOrder,
            OrderCore::ModifyOrder(_) => CoreType::ModifyOrder,
            OrderCore::RemoveOtherOrder(_) => CoreType::RemoveOtherOrder,
            OrderCore::NullOrder(_) => CoreType::NullOrder,
        }
    }

    #[inline]
    pub fn set_quantity(&mut self, quantity: BookQuantity) {
        match self {
            OrderCore::LimitOrder(order) => order.quantity = quantity,
            OrderCore::MarketOrder(order) => order.quantity = quantity,
            OrderCore::ModifyOrder(order) => order.quantity = quantity,
            _ => {},
        }
    }

    #[inline]
    pub fn set_price(&mut self, price: BookPrice) {
        match self {
            OrderCore::LimitOrder(order) => order.price = price,
            OrderCore::ModifyOrder(order) => order.price = price,
            _ => {},
        }
    }
    
    #[inline]
    pub fn quantity(&self) -> Option<BookQuantity> {
        match self {
            OrderCore::LimitOrder(order) => Some(order.quantity),
            OrderCore::MarketOrder(order) => Some(order.quantity),
            OrderCore::ModifyOrder(order) => Some(order.quantity),
            _ => None,
        }
    }

    #[inline]
    pub fn order_side(&self) -> Option<OrderSide> {
        match self {
            OrderCore::LimitOrder(order) => Some(order.order_side),
            OrderCore::MarketOrder(order) => Some(order.order_side),
            _ => None,
        }
    }

    #[inline]
    pub fn order_id(&self) -> Option<OrderId> {
        match self {
            OrderCore::LimitOrder(order) => Some(order.order_id),
            OrderCore::MarketOrder(order) => Some(order.order_id),
            OrderCore::CancelOrder(order) => Some(order.order_id),
            OrderCore::ModifyOrder(order) => Some(order.order_id),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_book_order() {
        use super::*;
        use crate::{BookPrice, BookQuantity, OrderId};
        use crate::order::enums::OrderSide;

        let price: BookPrice = 100;
        let quantity: BookQuantity = 100;
        let order_side: OrderSide = OrderSide::Bid;
        let order_id: OrderId = 1;

        let book_order = LimitOrder::new(price, quantity, order_side, order_id);

        assert_eq!(book_order.price, price);
        assert_eq!(book_order.quantity, quantity);
        assert_eq!(book_order.order_side, order_side);
        assert_eq!(book_order.order_id, order_id);
    }
}