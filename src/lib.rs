pub mod orders{
    tonic::include_proto!("orders");
}

pub use orders::{
    order_book_server::{OrderBook, OrderBookServer},
    CancelOrderRequest, CancelOrderResponse, ModifyOrderRequest, ModifyOrderResponse,
    NewOrderRequest, NewOrderResponse, BookRequest, BookResponse,BookDepth,PriceLevelDepth,
    order_book_client::{OrderBookClient}
};

impl From<clob_engine::order_book::types::BookDepth> for BookDepth {
     fn from(value: clob_engine::order_book::types::BookDepth) -> Self {
        let bid_depth = value.bid_depth.into_iter()
        .map(|price_level_depth| PriceLevelDepth {
            price : price_level_depth.price_level,
            quantity : price_level_depth.quantity
        }).collect();

        let ask_depth = value.ask_depth.into_iter()
        .map(|price_level_depth| PriceLevelDepth {
            price : price_level_depth.price_level,
            quantity : price_level_depth.quantity
        }).collect();
        Self { bid_depth, ask_depth }
    }
}