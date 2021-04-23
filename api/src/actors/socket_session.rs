use actix::{Actor, ActorContext, Addr, AsyncContext, Handler, Running, StreamHandler};
use actix_web_actors::ws;

use crate::messages::{Connected, SendClientMessage, UpdateUserSubscriptions};

use super::user_store::UserStore;

/// Actor for handling Web