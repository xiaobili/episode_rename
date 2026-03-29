//! Rename popup components.
//!
//! This module provides all rename-related dialog components extracted from
//! the original render.rs per D-06 (subdirectory pattern for related components).
//!
//! ## Components
//! - `mode`: Rename mode selection popup
//! - `manual`: Manual rename dialog
//! - `unified`: Unified naming dialog
//! - `regex`: Regex find/replace dialog
//! - `single`: Single file rename dialog
//! - `folder`: Single folder rename dialog
//!
//! ## Architecture
//! Per D-03, each component uses state-slice signatures for testability.

pub mod folder;
pub mod manual;
pub mod mode;
pub mod regex;
pub mod single;
pub mod unified;
