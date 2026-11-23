#[cfg(test)]
extern crate approx;

#[cfg(feature = "dev")]
mod dev;

#[cfg(feature = "tsv")]
pub mod tsv;

pub mod analytics;
pub mod config;
pub mod error;
pub mod game;
pub mod player;
pub mod tournament;
pub mod utils;
