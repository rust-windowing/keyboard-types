use alloc::string::{String, ToString};
use core::fmt;
use core::str::FromStr;

use crate::{first_char, NamedKey};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// The value received from the keypress.
#[derive(Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Key {
    /// A key string that corresponds to the character typed by the user,
    /// taking into account the user’s current locale setting, modifier state,
    /// and any system-level keyboard mapping overrides that are in effect.
    Character(String),
    Named(NamedKey),
}

impl fmt::Display for Key {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Character(s) => f.write_str(s),
            Self::Named(k) => k.fmt(f),
        }
    }
}

/// Parse from string error, returned when string does not match to any [`Key`] variant.
#[derive(Clone, Debug)]
pub struct UnrecognizedKeyError;

impl FromStr for Key {
    type Err = UnrecognizedKeyError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if is_key_string(s) {
            Ok(Self::Character(s.to_string()))
        } else {
            Ok(Self::Named(
                NamedKey::from_str(s).map_err(|_| UnrecognizedKeyError)?,
            ))
        }
    }
}

impl From<NamedKey> for Key {
    fn from(value: NamedKey) -> Self {
        Self::Named(value)
    }
}

impl Key {
    /// Determine a *charCode* value for a key with a character value.
    ///
    /// For all other keys the value is zero.
    /// The *charCode* is an implementation specific legacy property of DOM keyboard events.
    ///
    /// Specification: <https://w3c.github.io/uievents/#dom-keyboardevent-charcode>
    pub fn legacy_charcode(&self) -> u32 {
        // Spec: event.charCode = event.key.charCodeAt(0)
        // otherwise 0
        match self {
            Key::Character(ref c) => c.chars().next().unwrap_or('\0') as u32,
            Key::Named(_) => 0,
        }
    }

    /// Determine a *keyCode* value for a key.
    ///
    /// The *keyCode* is an implementation specific legacy property of DOM keyboard events.
    ///
    /// Specification: <https://w3c.github.io/uievents/#dom-keyboardevent-keycode>
    pub fn legacy_keycode(&self) -> u32 {
        match self {
            // See: https://w3c.github.io/uievents/#fixed-virtual-key-codes
            Key::Named(NamedKey::Backspace) => 8,
            Key::Named(NamedKey::Tab) => 9,
            Key::Named(NamedKey::Enter) => 13,
            Key::Named(NamedKey::Shift) => 16,
            Key::Named(NamedKey::Control) => 17,
            Key::Named(NamedKey::Alt) => 18,
            Key::Named(NamedKey::CapsLock) => 20,
            Key::Named(NamedKey::Escape) => 27,
            Key::Named(NamedKey::PageUp) => 33,
            Key::Named(NamedKey::PageDown) => 34,
            Key::Named(NamedKey::End) => 35,
            Key::Named(NamedKey::Home) => 36,
            Key::Named(NamedKey::ArrowLeft) => 37,
            Key::Named(NamedKey::ArrowUp) => 38,
            Key::Named(NamedKey::ArrowRight) => 39,
            Key::Named(NamedKey::ArrowDown) => 40,
            Key::Named(NamedKey::Delete) => 46,
            Key::Character(ref c) if c.len() == 1 => match first_char(c) {
                ' ' => 32,
                x @ '0'..='9' => x as u32,
                x @ 'a'..='z' => x.to_ascii_uppercase() as u32,
                x @ 'A'..='Z' => x as u32,
                // See: https://w3c.github.io/uievents/#optionally-fixed-virtual-key-codes
                ';' | ':' => 186,
                '=' | '+' => 187,
                ',' | '<' => 188,
                '-' | '_' => 189,
                '.' | '>' => 190,
                '/' | '?' => 191,
                '`' | '~' => 192,
                '[' | '{' => 219,
                '\\' | '|' => 220,
                ']' | '}' => 221,
                '\'' | '\"' => 222,
                _ => 0,
            },
            _ => 0,
        }
    }
}

impl Default for Key {
    fn default() -> Self {
        Self::Named(NamedKey::default())
    }
}

/// Check if string can be used as a `Key::Character` _keystring_.
///
/// This check is simple and is meant to prevents common mistakes like mistyped keynames
/// (e.g. `Ennter`) from being recognized as characters.
fn is_key_string(s: &str) -> bool {
    s.chars().all(|c| !c.is_control()) && s.chars().skip(1).all(|c| !c.is_ascii())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_is_key_string() {
        assert!(is_key_string("A"));
        assert!(!is_key_string("AA"));
        assert!(!is_key_string("	"));
    }

    #[test]
    fn into() {
        assert_eq!(Key::Named(NamedKey::Enter), NamedKey::Enter.into());
    }
}
