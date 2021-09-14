use std::collections::HashMap;

use actix::{Actor, Addr, Context, Handler};

use crate::{
    messages::{Connected, SendClientMessage, StockUpdated, UpdateUserSubscriptions},
    state::StockDataSink,
};

use super::socket_session::SocketSession;

const USER_CREDITS: u32 = 1024;

/// UserStore
/// where we store newly created Users and their info
/// the only place from where we update Users
pub(crate) struct UserStore {
    pub users: HashMap<usize, User>,
    pub stock_data_sink: StockDataSink,
}

impl Actor for UserStore {
    type Context = Context<Self>;
}

impl Handler<StockUpdated> for UserStore {
    type Result = ();

    /// on stock updates - iterate over all users and send them their subscribed prices
    /// also performs crediting the users
    fn handle(&mut self, _msg: StockUpdated, _ctx: &mut Self::Context) -> Self::Result {
        let stock_data = self.stock_data_sink.read().unwrap();

        for (_, user) in &mut self.users {
            let subs = user.subscriptions.len() as u32;

            if subs > 0 && user.credits > 0 && user.credits >= subs {
                let response = user
                    .subscriptions
                    .iter()
                    .filter(|stock| stock_data.get_last_price(stock).is_some())
                    .map(|stock| {
                        format!("{}: {}", stock, stock_data.get_l