#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::{Code, Key, KeyState, Location, Modifiers};

/// Keyboard events are issued for all pressed and released keys.
#[derive(Clone, Debug, Default, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct KeyboardEvent {
    /// Whether the key is pressed or released.
    pub state: KeyState,
    /// Logical key value.
    pub key: Key,
    /// Physical key position.
    pub code: Code,
    /// Location for keys with multiple instances on common keyboards.
    pub location: Location,
    /// Flags for pressed modifier keys.
    pub modifiers: Modifiers,
    /// True if the key is currently auto-repeated.
    pub repeat: bool,
    /// Events with this flag should be ignored in a text editor
    /// and instead [composition events](crate::CompositionEvent) should be used.
    pub is_composing: bool,
}

impl KeyboardEvent {
    /// Convenience constructor which takes `key` and `code`, sets `state` to
    /// [`KeyState::Down`], and sets everything else to default values.
    pub fn key_down(key: impl Into<Key>, code: Code) -> Self {
        KeyboardEvent {
            state: KeyState::Down,
            key: key.into(),
            code,
            ..Default::default()
        }
    }

    /// Convenience constructor which takes `key` and `code`, sets `state` to
    /// [`KeyState::Up`], and sets everything else to default values.
    pub fn key_up(key: impl Into<Key>, code: Code) -> Self {
        KeyboardEvent {
            state: KeyState::Up,
            key: key.into(),
            code,
            ..Default::default()
        }
    }
}
