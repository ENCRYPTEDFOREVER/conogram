#![warn(clippy::pedantic, clippy::nursery)]
#![allow(
    clippy::too_many_arguments,

    // Pedantic
    clippy::too_many_lines,
    clippy::if_not_else,
    clippy::module_name_repetitions,
    clippy::cast_sign_loss,
    clippy::cast_possible_truncation,
    clippy::no_effect_underscore_binding,
    clippy::unreadable_literal,
    clippy::cast_possible_wrap,
    clippy::similar_names,
    clippy::struct_excessive_bools,
    clippy::cast_precision_loss,
    clippy::wildcard_imports,
    clippy::must_use_candidate,
    clippy::doc_markdown,
    clippy::fn_params_excessive_bools,
    clippy::missing_const_for_fn,

    // TODO
    clippy::missing_errors_doc,
    clippy::derive_partial_eq_without_eq,

    // Nursery
    clippy::future_not_send,
    clippy::option_if_let_else
)]

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
