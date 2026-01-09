//! Structured integration tests for the Traversable derive macro.
//!
//! These tests verify the tree structure output for each category of types
//! in brk_computer/src/internal, ensuring the traversable attributes produce
//! the expected flat/merged structures.

mod common;

mod group_types;
mod lazy_aggregation;
mod derived_date;
mod computed_types;
