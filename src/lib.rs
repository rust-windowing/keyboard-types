//! Contains types to define keyboard related events.
//!
//! The naming and conventions follow the UI Events specification
//! but this crate should be useful for anyone implementing keyboard
//! input in a cross-platform way.

#![warn(clippy::doc_markdown)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![no_std]

extern crate alloc;
#[cfg(feature = "std")]
extern crate std;

pub use crate::code::{Code, UnrecognizedCodeError};
pub use crate::composition::{CompositionEvent, CompositionState};
pub use crate::key::{Key, UnrecognizedKeyError};
pub use crate::key_state::KeyState;
pub use crate::keyboard_event::KeyboardEvent;
pub use crate::location::Location;
pub use crate::modifiers::Modifiers;
pub use crate::named_key::{NamedKey, UnrecognizedNamedKeyError};
pub use crate::shortcuts::ShortcutMatcher;

mod code;
mod composition;
mod key;
mod key_state;
mod keyboard_event;
mod location;
mod modifiers;
mod named_key;
mod shortcuts;
#[cfg(feature = "webdriver")]
pub mod webdriver;

impl Default for NamedKey {
    fn default() -> Self {
        Self::Unidentified
    }
}

impl Default for Code {
    fn default() -> Code {
        Code::Unidentified
    }
}

/// Return the first codepoint of a string.
///
/// # Panics
/// Panics if the string is empty.
fn first_char(s: &str) -> char {
    s.chars().next().expect("empty string")
}
