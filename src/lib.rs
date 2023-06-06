#![allow(clippy::too_many_arguments)]

extern crate core;

pub mod api;
pub mod client;
pub mod errors;
pub(crate) mod macros;
pub(crate) mod request;
pub mod server_config;

pub mod entities;
pub mod methods;
pub mod utils;

#[cfg(test)]
mod tests {
    #[test]
    fn run() {}
}
