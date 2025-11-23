// Strict on correctness
#![deny(
    clippy::await_holding_lock,
    clippy::expect_used,
    clippy::panic,
    clippy::panic_in_result_fn,
    clippy::unwrap_used,
    clippy::unused_async,
    clippy::dbg_macro
)]
// Security / robustness
#![deny(
    clippy::assertions_on_constants,
    clippy::indexing_slicing,
    clippy::or_fun_call,
    clippy::unnecessary_safety_comment
)]
// Performance
#![warn(
    clippy::inefficient_to_string,
    clippy::large_enum_variant,
    clippy::large_stack_arrays,
    clippy::map_clone,
    clippy::naive_bytecount,
    clippy::needless_collect,
    clippy::single_match_else,
    clippy::stable_sort_primitive,
    clippy::unnecessary_to_owned
)]
// Readability & style (warnings, not build-breaking)
#![warn(
    clippy::bool_to_int_with_if,
    clippy::borrow_interior_mutable_const,
    clippy::collapsible_if,
    clippy::enum_glob_use,
    clippy::field_reassign_with_default,
    clippy::from_over_into,
    clippy::if_then_some_else_none,
    clippy::implicit_clone,
    clippy::manual_ok_or,
    clippy::match_bool,
    clippy::needless_borrow,
    clippy::needless_pass_by_value,
    clippy::option_if_let_else,
    clippy::redundant_clone,
    clippy::redundant_else,
    clippy::struct_excessive_bools,
    clippy::too_many_arguments,
    clippy::useless_let_if_seq
)]
// Lints that are often noisy; keep allowed unless you want to be very strict
#![allow(
    clippy::module_name_repetitions,  // often fine in Rust
    clippy::missing_errors_doc,       // depends on your doc policy
    clippy::missing_panics_doc,
    clippy::similar_names,            // can be noisy
    clippy::upper_case_acronyms       // style preference
)]

pub mod config;
pub mod database;
pub mod middleware;
pub mod routes;
pub mod websocket;
