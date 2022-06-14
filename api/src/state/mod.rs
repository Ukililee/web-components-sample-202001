
use actix_web::web::Data;
use std::sync::{Arc, RwLock};
use stock::StockData;

pub(crate) type StockDataSink = Arc<RwLock<StockData>>;

#[derive(Debug)]
pub(crate) struct AppState {