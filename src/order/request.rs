use crate::{OrderCore, InstId, TimeStamp, BookQuantity, OrderId};
use crate::order::enums::OrderStatus;
use serde::{Serialize, Deserialize};

/// The 'Request' means it is my order
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OrderRequest {
    pub order_core: OrderCore,
    pub instid: InstId,
    pub systemtime: TimeStamp,
    pub status: OrderStatus,
    pub filled: Option<BookQuantity>,
}

impl OrderRequest {
    #[inline]
    pub fn get_id(&self) -> Option<OrderId> {
        self.order_core.order_id()
    }

    #[inline]
    pub fn new(
        instid: InstId,
        order_core: OrderCore, // OrderId is in OrderCore if needed
        systemtime: TimeStamp,
    ) -> Self {
        Self {
            order_core,
            status: OrderStatus::PendingNew,
            systemtime,
            filled: None,
            instid,

        }
    }

    #[inline]
    pub fn accepted(&mut self) {
        self.status = OrderStatus::Accepted;
    }

    #[inline]
    pub fn trade(&mut self, amount: BookQuantity) -> OrderStatus {
        let quantity = self.order_core.quantity().expect("Order quantity not found in trade");
        let fill_amount = self.filled.unwrap_or(0) + amount;
        if fill_amount >= quantity {
            self.status = OrderStatus::FullyFilled;
        } else {
            self.status = OrderStatus::PartiallyFilled;
        }

        self.filled = Some(fill_amount);
        self.status
    }
}