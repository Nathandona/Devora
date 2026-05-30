//! The language status board shown by `devora list`.
//!
//! This is the single source of truth for the CLI's language roadmap and it
//! mirrors the website's language table. `stable` languages have a working
//! plugin embedded in the binary. (`paused` and `wishlist` states also exist
//! for future entries, but every shipped language is currently stable.)

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
        state: "stable",
        note: "1 template",
    },
    LangStatus {
        name: "go",
        state: "stable",
        note: "1 template",
    },
    LangStatus {
        name: "python",
        state: "stable",
        note: "1 template",
    },
    LangStatus {
        name: "c#",
        state: "stable",
        note: "1 template",
    },
];
