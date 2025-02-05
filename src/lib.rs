#![warn(clippy::pedantic, clippy::nursery)]
#![allow(

    // Both lints actually hurt code readability
    clippy::if_not_else,
    clippy::option_if_let_else,

    // Meh
    clippy::cast_sign_loss,
    clippy::cast_possible_truncation,
    clippy::cast_precision_loss,
    clippy::cast_possible_wrap,

    // The Bot API docs trigger this...
    clippy::too_long_first_doc_paragraph,
    clippy::doc_markdown,

    // In request param structs and their constructors, we have no control over it 
    clippy::struct_excessive_bools,
    clippy::fn_params_excessive_bools,

    // TODO: document that errors are raised by the Bot API?
    clippy::missing_errors_doc,

    // UsableMessageEffects enum with large ids
    clippy::unreadable_literal,
    clippy::enum_clike_unportable_variant,

    // Large enums are boxed when contained inside of structs 
    clippy::large_enum_variant
)]

extern crate core;

pub mod api;
pub mod client;
pub mod errors;
pub(crate) mod macros;
pub mod request;
pub mod server_config;

pub mod entities;
pub mod methods;
pub mod utils;
