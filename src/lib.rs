// thiserror 2.x generates code that triggers false-positive unused_assignments warnings
#![allow(unused_assignments)]

pub mod api;
pub mod auth;
pub mod cache;
pub mod cli;
pub mod config;
pub mod error;
pub mod models;
pub mod output;
pub mod rate_limit;
