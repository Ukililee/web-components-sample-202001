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
    /// send the id and the Addr of this socket to UserStore
    fn started(&mut self, ctx: &mut Self::Context) {
        let addr = ctx.address();
        self.addr.do_send(Connected {
            addr: addr.clone(),
            user_id: self.user_id,
        });
    }

    fn stopping(&mut self, _: &mut Self::Context) -> Running {
        Running::Stop
    }
}

impl Handler<SendClientMessage> for SocketSession {
    type Result = ();

    /// Receive messages from UserStore and forward them to the Client
    fn handle(&mut self, msg: SendClientMessage, ctx: &mut Self::Context) {
        ctx.text(msg.message);
    }
}

/// WebSocket message handler
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for SocketSession {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        let msg = match msg {
            Err(_) => {
                ctx.stop();
                return;
            }
            Ok(msg) => msg,
        };

        match msg {
            ws::Message::Text(te