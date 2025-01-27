pub mod udp_client;
pub mod tcp_client;
pub mod unique_id;
pub mod order;
pub mod data;

use static_id::StaticId;

pub type UnixNano = u64;
pub type BookQuantity = u64;
pub type BookPrice = i64;
pub type OrderId = u64;
pub type TimeStamp = UnixNano;
pub type InstId = StaticId;

pub use order::core::OrderCore;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
