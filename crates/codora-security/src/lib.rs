//! Codora - Security
//!
//! # Overview
//!
//! # TODO
#![forbid(unsafe_code)]
// Silence the noise in development!
#![cfg_attr(debug_assertions, allow(dead_code, unused_variables, unused_imports))]
// Docs and linting rules
#![cfg_attr(docsrs, feature(doc_auto_cfg, doc_cfg))]
#![cfg_attr(test, allow(clippy::float_cmp))]
#![cfg_attr(not(test), deny(clippy::print_stdout, clippy::dbg_macro))]
// - Lint for missing docs
#![cfg_attr(not(debug_assertions), deny(missing_docs))]

mod authentication;
mod authorization;
mod domain;
pub mod identity;

pub use {
    // Re-export for nicer importation!
    authentication::*,
    authorization::*,
};

// Extern crate
#[macro_use]
extern crate tracing;

#[macro_use]
extern crate codora_foundation;

// Use this page for export and documentation only but we gonna include some tests

#[cfg(test)]
mod tests {
    use super::*;
}
