mod connect4;
mod board;
mod cli;
mod otto;
mod ottobot;

use crate::board::*;
use wasm_bindgen::prelude::*;
use crate::otto::TootOttoBoard;
use std::sync::{Mutex,Arc};
use lazy_static::lazy_static;
use wasm_bindgen::JsValue;
use serde::Serialize;

