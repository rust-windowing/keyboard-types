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

    /// Get the [`NamedKey`] for a an XKB keysym.
    pub fn from_xkb_keysym(keysym: u32) -> Option<Key> {
        match keysym {
            0x20 => Some(Key::Character(" ".into())),
            0xFD06 => Some(Key::Named(NamedKey::EraseEof)),
            0xFD0E => Some(Key::Named(NamedKey::Attn)),
            0xFD16 => Some(Key::Named(NamedKey::Play)),
            0xFD1B => Some(Key::Named(NamedKey::ExSel)),
            0xFD1C => Some(Key::Named(NamedKey::CrSel)),
            0xFD1D => Some(Key::Named(NamedKey::PrintScreen)),
            0xFD1E => Some(Key::Named(NamedKey::Enter)),
            0xFE03 => Some(Key::Named(NamedKey::AltGraph)),
            0xFE04 => Some(Key::Named(NamedKey::AltGraph)),
            0xFE05 => Some(Key::Named(NamedKey::AltGraph)),
            0xFE08 => Some(Key::Named(NamedKey::GroupNext)),
            0xFE0A => Some(Key::Named(NamedKey::GroupPrevious)),
            0xFE0C => Some(Key::Named(NamedKey::GroupFirst)),
            0xFE0E => Some(Key::Named(NamedKey::GroupLast)),
            0xFE20 => Some(Key::Named(NamedKey::Tab)),
            0xFE34 => Some(Key::Named(NamedKey::Enter)),
            0xFF08 => Some(Key::Named(NamedKey::Backspace)),
            0xFF09 => Some(Key::Named(NamedKey::Tab)),
            0xFF0B => Some(Key::Named(NamedKey::Clear)),
            0xFF0D => Some(Key::Named(NamedKey::Enter)),
            0xFF13 => Some(Key::Named(NamedKey::Pause)),
            0xFF14 => Some(Key::Named(NamedKey::ScrollLock)),
            0xFF15 => Some(Key::Named(NamedKey::PrintScreen)),
            0xFF1B => Some(Key::Named(NamedKey::Escape)),
            0xFF20 => Some(Key::Named(NamedKey::Compose)),
            0xFF21 => Some(Key::Named(NamedKey::KanjiMode)),
            0xFF22 => Some(Key::Named(NamedKey::NonConvert)),
            0xFF23 => Some(Key::Named(NamedKey::Convert)),
            0xFF24 => Some(Key::Named(NamedKey::Romaji)),
            0xFF25 => Some(Key::Named(NamedKey::Hiragana)),
            0xFF27 => Some(Key::Named(NamedKey::HiraganaKatakana)),
            0xFF28 => Some(Key::Named(NamedKey::Zenkaku)),
            0xFF29 => Some(Key::Named(NamedKey::Hankaku)),
            0xFF2A => Some(Key::Named(NamedKey::ZenkakuHankaku)),
            0xFF2D => Some(Key::Named(NamedKey::KanaMode)),
            0xFF2E => Some(Key::Named(NamedKey::KanaMode)),
            0xFF2F => Some(Key::Named(NamedKey::Alphanumeric)),
            0xFF30 => Some(Key::Named(NamedKey::Alphanumeric)),
            0xFF37 => Some(Key::Named(NamedKey::CodeInput)),
            0xFF3C => Some(Key::Named(NamedKey::SingleCandidate)),
            0xFF3D => Some(Key::Named(NamedKey::AllCandidates)),
            0xFF3E => Some(Key::Named(NamedKey::PreviousCandidate)),
            0xFF50 => Some(Key::Named(NamedKey::Home)),
            0xFF51 => Some(Key::Named(NamedKey::ArrowLeft)),
            0xFF52 => Some(Key::Named(NamedKey::ArrowUp)),
            0xFF53 => Some(Key::Named(NamedKey::ArrowRight)),
            0xFF54 => Some(Key::Named(NamedKey::ArrowDown)),
            0xFF55 => Some(Key::Named(NamedKey::PageUp)),
            0xFF56 => Some(Key::Named(NamedKey::PageDown)),
            0xFF57 => Some(Key::Named(NamedKey::End)),
            0xFF60 => Some(Key::Named(NamedKey::Select)),
            0xFF61 => Some(Key::Named(NamedKey::PrintScreen)),
            0xFF62 => Some(Key::Named(NamedKey::Execute)),
            0xFF63 => Some(Key::Named(NamedKey::Insert)),
            0xFF65 => Some(Key::Named(NamedKey::Undo)),
            0xFF66 => Some(Key::Named(NamedKey::Redo)),
            0xFF67 => Some(Key::Named(NamedKey::ContextMenu)),
            0xFF68 => Some(Key::Named(NamedKey::Find)),
            0xFF69 => Some(Key::Named(NamedKey::Cancel)),
            0xFF6A => Some(Key::Named(NamedKey::Help)),
            0xFF6B => Some(Key::Named(NamedKey::Pause)),
            0xFF7E => Some(Key::Named(NamedKey::ModeChange)),
            0xFF7F => Some(Key::Named(NamedKey::NumLock)),
            0xFF89 => Some(Key::Named(NamedKey::Tab)),
            0xFF8D => Some(Key::Named(NamedKey::Enter)),
            0xFF91 => Some(Key::Named(NamedKey::F1)),
            0xFF92 => Some(Key::Named(NamedKey::F2)),
            0xFF93 => Some(Key::Named(NamedKey::F3)),
            0xFF94 => Some(Key::Named(NamedKey::F4)),
            0xFF95 => Some(Key::Named(NamedKey::Home)),
            0xFF96 => Some(Key::Named(NamedKey::ArrowLeft)),
            0xFF97 => Some(Key::Named(NamedKey::ArrowUp)),
            0xFF98 => Some(Key::Named(NamedKey::ArrowRight)),
            0xFF99 => Some(Key::Named(NamedKey::ArrowDown)),
            0xFF9A => Some(Key::Named(NamedKey::PageUp)),
            0xFF9B => Some(Key::Named(NamedKey::PageDown)),
            0xFF9C => Some(Key::Named(NamedKey::End)),
            0xFF9E => Some(Key::Named(NamedKey::Insert)),
            0xFF9F => Some(Key::Named(NamedKey::Delete)),
            0xFFBE => Some(Key::Named(NamedKey::F1)),
            0xFFBF => Some(Key::Named(NamedKey::F2)),
            0xFFC0 => Some(Key::Named(NamedKey::F3)),
            0xFFC1 => Some(Key::Named(NamedKey::F4)),
            0xFFC2 => Some(Key::Named(NamedKey::F5)),
            0xFFC3 => Some(Key::Named(NamedKey::F6)),
            0xFFC4 => Some(Key::Named(NamedKey::F7)),
            0xFFC5 => Some(Key::Named(NamedKey::F8)),
            0xFFC6 => Some(Key::Named(NamedKey::F9)),
            0xFFC7 => Some(Key::Named(NamedKey::F10)),
            0xFFC8 => Some(Key::Named(NamedKey::F11)),
            0xFFC9 => Some(Key::Named(NamedKey::F12)),
            0xFFCA => Some(Key::Named(NamedKey::F13)),
            0xFFCB => Some(Key::Named(NamedKey::F14)),
            0xFFCC => Some(Key::Named(NamedKey::F15)),
            0xFFCD => Some(Key::Named(NamedKey::F16)),
            0xFFCE => Some(Key::Named(NamedKey::F17)),
            0xFFCF => Some(Key::Named(NamedKey::F18)),
            0xFFD0 => Some(Key::Named(NamedKey::F19)),
            0xFFD1 => Some(Key::Named(NamedKey::F20)),
            0xFFD2 => Some(Key::Named(NamedKey::F21)),
            0xFFD3 => Some(Key::Named(NamedKey::F22)),
            0xFFD4 => Some(Key::Named(NamedKey::F23)),
            0xFFD5 => Some(Key::Named(NamedKey::F24)),
            0xFFD6 => Some(Key::Named(NamedKey::F25)),
            0xFFD7 => Some(Key::Named(NamedKey::F26)),
            0xFFD8 => Some(Key::Named(NamedKey::F27)),
            0xFFD9 => Some(Key::Named(NamedKey::F28)),
            0xFFDA => Some(Key::Named(NamedKey::F29)),
            0xFFDB => Some(Key::Named(NamedKey::F30)),
            0xFFDC => Some(Key::Named(NamedKey::F31)),
            0xFFDD => Some(Key::Named(NamedKey::F32)),
            0xFFDE => Some(Key::Named(NamedKey::F33)),
            0xFFDF => Some(Key::Named(NamedKey::F34)),
            0xFFE0 => Some(Key::Named(NamedKey::F35)),
            0xFFE1 => Some(Key::Named(NamedKey::Shift)),
            0xFFE2 => Some(Key::Named(NamedKey::Shift)),
            0xFFE3 => Some(Key::Named(NamedKey::Control)),
            0xFFE4 => Some(Key::Named(NamedKey::Control)),
            0xFFE5 => Some(Key::Named(NamedKey::CapsLock)),
            0xFFE9 => Some(Key::Named(NamedKey::Alt)),
            0xFFEA => Some(Key::Named(NamedKey::Alt)),
            0xFFEB => Some(Key::Named(NamedKey::Meta)),
            0xFFEC => Some(Key::Named(NamedKey::Meta)),
            #[allow(deprecated)]
            0xFFED => Some(Key::Named(NamedKey::Hyper)),
            #[allow(deprecated)]
            0xFFEE => Some(Key::Named(NamedKey::Hyper)),
            0xFFFF => Some(Key::Named(NamedKey::Delete)),
            0x1005FF72 => Some(Key::Named(NamedKey::Copy)),
            0x1005FF73 => Some(Key::Named(NamedKey::Open)),
            0x1005FF74 => Some(Key::Named(NamedKey::Paste)),
            0x1005FF75 => Some(Key::Named(NamedKey::Cut)),
            0x1005FF77 => Some(Key::Named(NamedKey::AudioVolumeDown)),
            0x1005FF78 => Some(Key::Named(NamedKey::AudioVolumeMute)),
            0x1005FF79 => Some(Key::Named(NamedKey::AudioVolumeUp)),
            0x1005FF7B => Some(Key::Named(NamedKey::BrightnessDown)),
            0x1005FF7C => Some(Key::Named(NamedKey::BrightnessUp)),
            0x1008FE22 => Some(Key::Named(NamedKey::VideoModeNext)),
            0x1008FF02 => Some(Key::Named(NamedKey::BrightnessUp)),
            0x1008FF03 => Some(Key::Named(NamedKey::BrightnessDown)),
            0x1008FF10 => Some(Key::Named(NamedKey::Standby)),
            0x1008FF11 => Some(Key::Named(NamedKey::AudioVolumeDown)),
            0x1008FF12 => Some(Key::Named(NamedKey::AudioVolumeMute)),
            0x1008FF13 => Some(Key::Named(NamedKey::AudioVolumeUp)),
            0x1008FF14 => Some(Key::Named(NamedKey::MediaPlay)),
            0x1008FF15 => Some(Key::Named(NamedKey::MediaStop)),
            0x1008FF16 => Some(Key::Named(NamedKey::MediaTrackPrevious)),
            0x1008FF17 => Some(Key::Named(NamedKey::MediaTrackNext)),
            0x1008FF18 => Some(Key::Named(NamedKey::BrowserHome)),
            0x1008FF19 => Some(Key::Named(NamedKey::LaunchMail)),
            0x1008FF1B => Some(Key::Named(NamedKey::BrowserSearch)),
            0x1008FF1C => Some(Key::Named(NamedKey::MediaRecord)),
            0x1008FF1D => Some(Key::Named(NamedKey::LaunchApplication2)),
            0x1008FF20 => Some(Key::Named(NamedKey::LaunchCalendar)),
            0x1008FF21 => Some(Key::Named(NamedKey::Power)),
            0x1008FF26 => Some(Key::Named(NamedKey::BrowserBack)),
            0x1008FF27 => Some(Key::Named(NamedKey::BrowserForward)),
            0x1008FF29 => Some(Key::Named(NamedKey::BrowserRefresh)),
            0x1008FF2A => Some(Key::Named(NamedKey::Power)),
            0x1008FF2B => Some(Key::Named(NamedKey::WakeUp)),
            0x1008FF2C => Some(Key::Named(NamedKey::Eject)),
            0x1008FF2D => Some(Key::Named(NamedKey::LaunchScreenSaver)),
            0x1008FF2E => Some(Key::Named(NamedKey::LaunchWebBrowser)),
            0x1008FF2F => Some(Key::Named(NamedKey::Standby)),
            0x1008FF30 => Some(Key::Named(NamedKey::BrowserFavorites)),
            0x1008FF31 => Some(Key::Named(NamedKey::MediaPause)),
            0x1008FF33 => Some(Key::Named(NamedKey::LaunchApplication1)),
            0x1008FF3E => Some(Key::Named(NamedKey::MediaRewind)),
            0x1008FF54 => Some(Key::Named(NamedKey::LaunchApplication2)),
            0x1008FF56 => Some(Key::Named(NamedKey::Close)),
            0x1008FF57 => Some(Key::Named(NamedKey::Copy)),
            0x1008FF58 => Some(Key::Named(NamedKey::Cut)),
            0x1008FF5C => Some(Key::Named(NamedKey::LaunchSpreadsheet)),
            0x1008FF61 => Some(Key::Named(NamedKey::LogOff)),
            0x1008FF67 => Some(Key::Named(NamedKey::BrowserFavorites)),
            0x1008FF68 => Some(Key::Named(NamedKey::New)),
            0x1008FF6B => Some(Key::Named(NamedKey::Open)),
            0x1008FF6D => Some(Key::Named(NamedKey::Paste)),
            0x1008FF6E => Some(Key::Named(NamedKey::LaunchPhone)),
            0x1008FF72 => Some(Key::Named(NamedKey::MailReply)),
            0x1008FF73 => Some(Key::Named(NamedKey::BrowserRefresh)),
            0x1008FF77 => Some(Key::Named(NamedKey::Save)),
            0x1008FF7B => Some(Key::Named(NamedKey::MailSend)),
            0x1008FF7C => Some(Key::Named(NamedKey::SpellCheck)),
            0x1008FF7D => Some(Key::Named(NamedKey::SplitScreenToggle)),
            0x1008FF87 => Some(Key::Named(NamedKey::LaunchMediaPlayer)),
            0x1008FF89 => Some(Key::Named(NamedKey::LaunchWordProcessor)),
            0x1008FF8B => Some(Key::Named(NamedKey::ZoomIn)),
            0x1008FF8C => Some(Key::Named(NamedKey::ZoomOut)),
            0x1008FF8F => Some(Key::Named(NamedKey::LaunchWebCam)),
            0x1008FF90 => Some(Key::Named(NamedKey::MailForward)),
            0x1008FF92 => Some(Key::Named(NamedKey::LaunchMusicPlayer)),
            0x1008FF97 => Some(Key::Named(NamedKey::MediaFastForward)),
            0x1008FF99 => Some(Key::Named(NamedKey::RandomToggle)),
            0x1008FF9A => Some(Key::Named(NamedKey::Subtitle)),
            0x1008FF9B => Some(Key::Named(NamedKey::MediaAudioTrack)),
            0x1008FFA7 => Some(Key::Named(NamedKey::Standby)),
            0x1008FFA8 => Some(Key::Named(NamedKey::Hibernate)),
            _ => None,
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
