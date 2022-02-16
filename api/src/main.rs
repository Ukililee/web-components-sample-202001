
use std::collections::HashMap;

use actix::{Actor, Addr};
use actix_web::{
    web::{self, Data, HttpResponse},
    App, Error, HttpRequest, HttpServer,
};
mod actors;
mod messages;
mod state;
use actix_web_actors::ws;
use actors::{socket_session::SocketSession, stock_engine::StockEngine, user_store::UserStore};
use serde::{Deserialize, Serialize};
use state::AppState;
use stock::StockSummary;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let address = "127.0.0.1:3000";
    let app_state = state::AppState::new();

    let user_store: Addr<UserStore> = UserStore {
        users: HashMap::new(),