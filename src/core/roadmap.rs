//! The language status board shown by `devora list`.
//!
//! This is the single source of truth for the CLI's language roadmap and it
//! mirrors the website's language table. `stable` languages have a working
//! plugin embedded in the binary; `paused` and `wishlist` entries are
//! aspirational and have no (or frozen) plugins.

use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct LangStatus {
    pub name: &'static str,
    pub state: &'static str,
    pub note: &'static str,
}

pub static ROADMAP: &[LangStatus] = &[
    LangStatus {
        name: "rust",
        state: "stable",
        note: "1 template",
    },
    LangStatus {
        name: "c++",
        state: "paused",
        note: "templates being rethought",
    },
    LangStatus {
        name: "go",
        state: "wishlist",
        note: "open to contributions",
    },
    LangStatus {
        name: "python",
        state: "wishlist",
        note: "open to contributions",
    },
    LangStatus {
        name: "typescript",
        state: "wishlist",
        note: "open to contributions",
    },
    LangStatus {
        name: "zig",
        state: "wishlist",
        note: "open to contributions",
    },
];
