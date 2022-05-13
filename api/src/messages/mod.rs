use actix::{Addr, Message};

use crate::actors::socket_session::SocketSession;

#[derive(Message)]
#[rtype(result = "()")]
pub(crate) struct StockUpdated;

#[derive(Message)]
#[rtype(result = "()")]
pub(crate) struct Connected {
    pub addr: Addr<SocketSession>,
    pub 