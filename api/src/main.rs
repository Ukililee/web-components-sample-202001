
use std::collections::HashMap;

use actix::{Actor, Addr};
use actix_web::{
    web::{self, Data, HttpResponse},
    App, Error, HttpRequest, HttpServer,
};
mod actors;
mod messages;