use actix::{Addr, Message};

use crate::actors::socket_session::SocketSession;

#[derive(Message)]
#[rtype(result = "