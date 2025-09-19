#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Describes the state a key is in.
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum KeyState {
    /// The key is pressed down.
    ///
    /// Often emitted in a [keydown] event, see also [the MDN documentation][mdn] on that.
    ///
    /// [keydown]: https://w3c.github.io/uievents/#event-type-keydown
    /// [mdn]: https://developer.mozilla.org/en-US/docs/Web/API/Element/keydown_event
    Down,
    /// The key is not pressed / was just released.
    ///
    /// Often emitted in a [keyup] event, see also [the MDN documentation][mdn] on that.
    ///
    /// [keyup]: https://w3c.github.io/uievents/#event-type-keyup
    /// [mdn]: https://developer.mozilla.org/en-US/docs/Web/API/Element/keyup_event
    Up,
}

impl Default for KeyState {
    fn default() -> KeyState {
        KeyState::Down
    }
}

impl KeyState {
    /// The [type] name of the corresponding key event.
    ///
    /// This is either `"keydown"` or `"keyup"`.
    ///
    /// [type]: https://w3c.github.io/uievents/#events-keyboard-types
    pub const fn event_type(self) -> &'static str {
        match self {
            Self::Down => "keydown",
            Self::Up => "keyup",
        }
    }

    /// True if the key is pressed down.
    pub const fn is_down(self) -> bool {
        matches!(self, Self::Down)
    }

    /// True if the key is released.
    pub const fn is_up(self) -> bool {
        matches!(self, Self::Up)
    }
}
