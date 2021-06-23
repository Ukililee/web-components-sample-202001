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
/// engine that generates tic