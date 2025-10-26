//! Devora - A modern developer tool for C++ projects
//!
//! Inspired by Vite, Devora provides instant project scaffolding, fast incremental builds,
//! and live reload capabilities for C++ projects using Meson + Ninja.

pub mod cli;
pub mod config;
pub mod dependencies;
pub mod logger;
pub mod result;

// Core modules
pub mod build;
pub mod create;
pub mod dev;
pub mod lint;
pub mod template;
pub mod test;
pub mod utils;