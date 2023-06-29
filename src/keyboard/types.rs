use bitflags::bitflags;

#[derive(Debug, Clone)]
pub struct KeyPress {
    pub code: KeyCode,
    pub action: KeyAction,
    // pub modifier: KeyModifiers
}

#[derive(Debug, Clone)]
pub enum KeyAction {
    Press,
    Release,
    Repeat
}



impl From<crossterm::event::KeyCode> for KeyCode {
    fn from(value: crossterm::event::KeyCode) -> Self {
        match value {
            crossterm::event::KeyCode::Backspace => KeyCode::Backspace,
            crossterm::event::KeyCode::Enter => KeyCode::Enter,
            crossterm::event::KeyCode::Left => KeyCode::Left,
            crossterm::event::KeyCode::Right => KeyCode::Right,
            crossterm::event::KeyCode::Up => KeyCode::Up,
            crossterm::event::KeyCode::Down => KeyCode::Down,
            crossterm::event::KeyCode::Home => KeyCode::Home,
            crossterm::event::KeyCode::End => KeyCode::End,
            crossterm::event::KeyCode::PageUp => KeyCode::PageUp,
            crossterm::event::KeyCode::PageDown => KeyCode::PageDown,
            crossterm::event::KeyCode::Tab => KeyCode::Tab,
            crossterm::event::KeyCode::BackTab => KeyCode::BackTab,
            crossterm::event::KeyCode::Delete => KeyCode::Delete,
            crossterm::event::KeyCode::Insert => KeyCode::Insert,
            crossterm::event::KeyCode::F(c) => KeyCode::F(c),
            crossterm::event::KeyCode::Char(c) => KeyCode::Char(c),
            crossterm::event::KeyCode::Null => KeyCode::Null,
            crossterm::event::KeyCode::Esc => KeyCode::Esc,
            crossterm::event::KeyCode::CapsLock => KeyCode::CapsLock,
            crossterm::event::KeyCode::ScrollLock => KeyCode::ScrollLock,
            crossterm::event::KeyCode::NumLock => KeyCode::NumLock,
            crossterm::event::KeyCode::PrintScreen => KeyCode::PrintScreen,
            crossterm::event::KeyCode::Pause => KeyCode::Pause,
            crossterm::event::KeyCode::Menu => KeyCode::Menu,
            crossterm::event::KeyCode::KeypadBegin => KeyCode::KeypadBegin,
            crossterm::event::KeyCode::Media(c) => KeyCode::Media(c.into()),
            crossterm::event::KeyCode::Modifier(c) => KeyCode::Modifier(c.into()),
        }
    }
}

impl From<crossterm::event::MediaKeyCode> for MediaKeyCode {
    fn from(value: crossterm::event::MediaKeyCode) -> Self {
        match value {
            crossterm::event::MediaKeyCode::Play => MediaKeyCode::Play,
            crossterm::event::MediaKeyCode::Pause => MediaKeyCode::Pause,
            crossterm::event::MediaKeyCode::PlayPause => MediaKeyCode::PlayPause,
            crossterm::event::MediaKeyCode::Reverse => MediaKeyCode::Reverse,
            crossterm::event::MediaKeyCode::Stop => MediaKeyCode::Stop,
            crossterm::event::MediaKeyCode::FastForward => MediaKeyCode::FastForward,
            crossterm::event::MediaKeyCode::Rewind => MediaKeyCode::Rewind,
            crossterm::event::MediaKeyCode::TrackNext => MediaKeyCode::TrackNext,
            crossterm::event::MediaKeyCode::TrackPrevious => MediaKeyCode::TrackPrevious,
            crossterm::event::MediaKeyCode::Record => MediaKeyCode::Record,
            crossterm::event::MediaKeyCode::LowerVolume => MediaKeyCode::LowerVolume,
            crossterm::event::MediaKeyCode::RaiseVolume => MediaKeyCode::RaiseVolume,
            crossterm::event::MediaKeyCode::MuteVolume => MediaKeyCode::MuteVolume,
        }
    }
}

impl From<crossterm::event::ModifierKeyCode> for ModifierKeyCode {
    fn from(value: crossterm::event::ModifierKeyCode) -> Self {
        match value {
            crossterm::event::ModifierKeyCode::LeftShift => ModifierKeyCode::LeftShift,
            crossterm::event::ModifierKeyCode::LeftControl => ModifierKeyCode::LeftControl,
            crossterm::event::ModifierKeyCode::LeftAlt => ModifierKeyCode::LeftAlt,
            crossterm::event::ModifierKeyCode::LeftSuper => ModifierKeyCode::LeftSuper,
            crossterm::event::ModifierKeyCode::LeftHyper => ModifierKeyCode::LeftHyper,
            crossterm::event::ModifierKeyCode::LeftMeta => ModifierKeyCode::LeftMeta,
            crossterm::event::ModifierKeyCode::RightShift => ModifierKeyCode::RightShift,
            crossterm::event::ModifierKeyCode::RightControl => ModifierKeyCode::RightControl,
            crossterm::event::ModifierKeyCode::RightAlt => ModifierKeyCode::RightAlt,
            crossterm::event::ModifierKeyCode::RightSuper => ModifierKeyCode::RightSuper,
            crossterm::event::ModifierKeyCode::RightHyper => ModifierKeyCode::RightHyper,
            crossterm::event::ModifierKeyCode::RightMeta => ModifierKeyCode::RightMeta,
            crossterm::event::ModifierKeyCode::IsoLevel3Shift => ModifierKeyCode::IsoLevel3Shift,
            crossterm::event::ModifierKeyCode::IsoLevel5Shift => ModifierKeyCode::IsoLevel5Shift,
        }
    }
}

bitflags! {
    /// Represents key modifiers (shift, control, alt, etc.).
    ///
    /// **Note:** `SUPER`, `HYPER`, and `META` can only be read if
    /// [`KeyboardEnhancementFlags::DISAMBIGUATE_ESCAPE_CODES`] has been enabled with
    /// [`PushKeyboardEnhancementFlags`].
    #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
    #[derive(Debug)]
    pub struct KeyModifiers: u8 {
        const SHIFT = 0b0000_0001;
        const CONTROL = 0b0000_0010;
        const ALT = 0b0000_0100;
        const SUPER = 0b0000_1000;
        const HYPER = 0b0001_0000;
        const META = 0b0010_0000;
        const NONE = 0b0000_0000;
    }
}

// impl From<u8> for  KeyModifiers {
//     fn from(value: u8) -> Self {
//         let mut ret: KeyModifiers = 0.into();
//         match value {
//             _ if value & 0b0000_0001 == 0b0000_0001 => ret &= KeyModifiers::SHIFT,
//             _ if value & 0b0000_0010 == 0b0000_0010 => ret &= KeyModifiers::CONTROL,
//             _ if value & 0b0000_0100 == 0b0000_0100 => ret &= KeyModifiers::ALT,
//             _ if value & 0b0000_1000 == 0b0000_1000 => ret &= KeyModifiers::SUPER,
//             _ if value & 0b0001_0000 == 0b0001_0000 => ret &= KeyModifiers::HYPER,
//             _ if value & 0b0010_0000 == 0b0010_0000 => ret &= KeyModifiers::META,
//             _ => unreachable!("bitflag convert unimplemented")
//         };

//         ret
//     }
// }

impl From<crossterm::event::KeyEventKind> for KeyAction {
    fn from(value: crossterm::event::KeyEventKind) -> Self {
        match value {
            crossterm::event::KeyEventKind::Press => KeyAction::Press,
            crossterm::event::KeyEventKind::Repeat => KeyAction::Repeat,
            crossterm::event::KeyEventKind::Release => KeyAction::Release,
        }
    }
}

/// Represents a key.
#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy, Hash)]
pub enum KeyCode {
    /// Backspace key.
    Backspace,
    /// Enter key.
    Enter,
    /// Left arrow key.
    Left,
    /// Right arrow key.
    Right,
    /// Up arrow key.
    Up,
    /// Down arrow key.
    Down,
    /// Home key.
    Home,
    /// End key.
    End,
    /// Page up key.
    PageUp,
    /// Page down key.
    PageDown,
    /// Tab key.
    Tab,
    /// Shift + Tab key.
    BackTab,
    /// Delete key.
    Delete,
    /// Insert key.
    Insert,
    /// F key.
    ///
    /// `KeyCode::F(1)` represents F1 key, etc.
    F(u8),
    /// A character.
    ///
    /// `KeyCode::Char('c')` represents `c` character, etc.
    Char(char),
    /// Null.
    Null,
    /// Escape key.
    Esc,
    /// Caps Lock key.
    ///
    /// **Note:** this key can only be read if
    /// [`KeyboardEnhancementFlags::DISAMBIGUATE_ESCAPE_CODES`] has been enabled with
    /// [`PushKeyboardEnhancementFlags`].
    CapsLock,
    /// Scroll Lock key.
    ///
    /// **Note:** this key can only be read if
    /// [`KeyboardEnhancementFlags::DISAMBIGUATE_ESCAPE_CODES`] has been enabled with
    /// [`PushKeyboardEnhancementFlags`].
    ScrollLock,
    /// Num Lock key.
    ///
    /// **Note:** this key can only be read if
    /// [`KeyboardEnhancementFlags::DISAMBIGUATE_ESCAPE_CODES`] has been enabled with
    /// [`PushKeyboardEnhancementFlags`].
    NumLock,
    /// Print Screen key.
    ///
    /// **Note:** this key can only be read if
    /// [`KeyboardEnhancementFlags::DISAMBIGUATE_ESCAPE_CODES`] has been enabled with
    /// [`PushKeyboardEnhancementFlags`].
    PrintScreen,
    /// Pause key.
    ///
    /// **Note:** this key can only be read if
    /// [`KeyboardEnhancementFlags::DISAMBIGUATE_ESCAPE_CODES`] has been enabled with
    /// [`PushKeyboardEnhancementFlags`].
    Pause,
    /// Menu key.
    ///
    /// **Note:** this key can only be read if
    /// [`KeyboardEnhancementFlags::DISAMBIGUATE_ESCAPE_CODES`] has been enabled with
    /// [`PushKeyboardEnhancementFlags`].
    Menu,
    /// The "Begin" key (often mapped to the 5 key when Num Lock is turned on).
    ///
    /// **Note:** this key can only be read if
    /// [`KeyboardEnhancementFlags::DISAMBIGUATE_ESCAPE_CODES`] has been enabled with
    /// [`PushKeyboardEnhancementFlags`].
    KeypadBegin,
    /// A media key.
    ///
    /// **Note:** these keys can only be read if
    /// [`KeyboardEnhancementFlags::DISAMBIGUATE_ESCAPE_CODES`] has been enabled with
    /// [`PushKeyboardEnhancementFlags`].
    Media(MediaKeyCode),
    /// A modifier key.
    ///
    /// **Note:** these keys can only be read if **both**
    /// [`KeyboardEnhancementFlags::DISAMBIGUATE_ESCAPE_CODES`] and
    /// [`KeyboardEnhancementFlags::REPORT_ALL_KEYS_AS_ESCAPE_CODES`] have been enabled with
    /// [`PushKeyboardEnhancementFlags`].
    Modifier(ModifierKeyCode),
}

/// Represents a media key (as part of [`KeyCode::Media`]).
#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy, Hash)]
pub enum MediaKeyCode {
    /// Play media key.
    Play,
    /// Pause media key.
    Pause,
    /// Play/Pause media key.
    PlayPause,
    /// Reverse media key.
    Reverse,
    /// Stop media key.
    Stop,
    /// Fast-forward media key.
    FastForward,
    /// Rewind media key.
    Rewind,
    /// Next-track media key.
    TrackNext,
    /// Previous-track media key.
    TrackPrevious,
    /// Record media key.
    Record,
    /// Lower-volume media key.
    LowerVolume,
    /// Raise-volume media key.
    RaiseVolume,
    /// Mute media key.
    MuteVolume,
}

/// Represents a modifier key (as part of [`KeyCode::Modifier`]).
#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy, Hash)]
pub enum ModifierKeyCode {
    /// Left Shift key.
    LeftShift,
    /// Left Control key.
    LeftControl,
    /// Left Alt key.
    LeftAlt,
    /// Left Super key.
    LeftSuper,
    /// Left Hyper key.
    LeftHyper,
    /// Left Meta key.
    LeftMeta,
    /// Right Shift key.
    RightShift,
    /// Right Control key.
    RightControl,
    /// Right Alt key.
    RightAlt,
    /// Right Super key.
    RightSuper,
    /// Right Hyper key.
    RightHyper,
    /// Right Meta key.
    RightMeta,
    /// Iso Level3 Shift key.
    IsoLevel3Shift,
    /// Iso Level5 Shift key.
    IsoLevel5Shift,
}

