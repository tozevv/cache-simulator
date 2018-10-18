extern crate chrono;

use chrono::prelude::*;
use chrono::DateTime;
use chrono::Utc;

pub struct Request {
    pub date: DateTime<Utc>,
    pub path: String,
    pub query: String,
    pub status: u16,
    pub size: u32
}