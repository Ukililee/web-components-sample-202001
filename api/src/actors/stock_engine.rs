use crate::{messages::StockUpdated, state::StockDataSink};
use actix::{
    clock::{interval_at, Instant},
    Actor, Addr, Context,
};
use futures::StreamExt;
us