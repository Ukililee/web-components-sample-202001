use crate::{messages::StockUpdated, state::StockDataSink};
use actix::{
    clock::{interval_at, Instant},
    Actor, Addr, Context,
};
use futures::StreamExt;
use std::time::Duration;

use super::user_store::UserStore;

const TICK_INTERVAL: u64 = 1;

/// Stock Engine
/// engine that generates ticks and informs UserStore of Stock Updates
/// this engine is the only place from where we are updating the AppState's stock data
