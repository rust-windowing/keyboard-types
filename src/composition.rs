#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use alloc::string::String;

/// Describes the state of a composition session.
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum CompositionState {
    /// The [compositionstart] event.
    ///
    /// See also [the MDN documentation][mdn].
    ///
    /// [compositionstart]: https://w3c.github.io/uievents/#event-type-compositionstart
    /// [mdn]: https://developer.mozilla.org/en-US/docs/Web/API/Element/compositionstart_event
    Start,
    /// The [compositionupdate] event.
    ///
    /// See also [the MDN documentation][mdn].
    ///
    /// [compositionupdate]: https://w3c.github.io/uievents/#event-type-compositionupdate
    /// [mdn]: https://developer.mozilla.org/en-US/docs/Web/API/Element/compositionupdate_event
    Update,
    /// The [compositionend] event.
    ///
    /// In a text editor, in this state the data should be added to the input.
    ///
    /// See also [the MDN documentation][mdn].
    ///
    /// [compositionend]: https://w3c.github.io/uievents/#event-type-compositionend
    /// [mdn]: https://developer.mozilla.org/en-US/docs/Web/API/Element/compositionend_event
    End,
}

impl CompositionState {
    /// The [type] name of the corresponding composition event.
    ///
    /// This is either `"compositionstart"`, `"compositionupdate"` or `"compositionend"`.
    ///
    /// [type]: https://w3c.github.io/uievents/#events-composition-types
    pub const fn event_type(self) -> &'static str {
        match self {
            Self::Start => "compositionstart",
            Self::Update => "compositionupdate",
            Self::End => "compositionend",
        }
    }
}

/// Event to expose input methods to program logic.
///
/// Provides information about entered sequences from
/// dead key combinations and IMEs.
///
/// A composition session is always started by a [`CompositionState::Start`]
/// event followed by zero or more [`CompositionState::Update`] events
/// and terminated by a single [`CompositionState::End`] event.
#[derive(Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CompositionEvent {
    /// Describes the event kind.
    pub state: CompositionState,
    /// Current composition data. May be empty.
    pub data: String,
}
