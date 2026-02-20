pub mod orders{
    tonic::include_proto!("orders");
}

pub use orders::{
    order_book_server::{OrderBook, OrderBookServer},
    CancelOrderRequest, CancelOrderResponse, ModifyOrderRequest, ModifyOrderResponse,
    NewOrderRequest, NewOrderResponse, BookRequest, BookResponse,BookDepth,PriceLevelDepth,
    order_book_client::{OrderBookClient}
};