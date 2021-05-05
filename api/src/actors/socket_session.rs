use actix::{Actor, ActorContext, Addr, AsyncContext, Handler, Running, StreamHandler};
use actix_web_actors::ws;

use crate::messages::{Connected, SendClientMessage, UpdateUserSubscriptions};

use super::user_store::UserStore;

/// Actor for handling Websocket Connection,
/// created on each User connection
/// holds the clone ref of UserStore Actor
pub(crate) struct SocketSession {
    pub addr: Addr<UserStore>,
    pub user_id: usize,
}

impl Actor for SocketSession {
    type Context = ws::WebsocketContext<Self>;

    /// on new connection established,
    /// send