use std::collections::HashMap;

use actix::{Actor, Addr, Context, Handler};

use crate::{
    messages::{Connected, SendClientMessage, StockUpdated, UpdateUserSubscriptions},
    state::StockDataSink,
};

use super::soc