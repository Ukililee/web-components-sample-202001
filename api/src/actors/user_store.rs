use std::collections::HashMap;

use actix::{Actor, Addr, Context, Handler};

use crate::{
    messages::{Connected, SendClientMessage, StockUpdated, UpdateUserSubscriptions},
    state::StockDataSink,
};

use super::socket_session::SocketSession;

const USER_CREDITS: u32 = 1024;

/// U