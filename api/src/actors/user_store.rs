use std::collections::HashMap;

use actix::{Actor, Addr, Context, Handler};

use crate::{
    messages::{Connected, SendClientMessag