// rustdoc imports
#[allow(unused_imports)]
use crate::{SDL_KeyboardEvent, SDL_KMOD_MODE};

/// The SDL keyboard scancode representation.
///
/// # Remarks
/// An SDL scancode is the physical representation of a key on the keyboard, independent of
/// language and keyboard mapping.
///
/// Values of this type are used to represent keyboard keys, among other places in the scancode
/// field of the [`SDL_KeyboardEvent`] structure.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
#[allow(missing_docs)]
pub enum SDL_Scancode {
    Unknown = 0,

    /**
     *  \name Usage page 0x07
     *
     *  These values are from usage page 0x07 (USB keyboard page).
     */
    /* @{ */
    A = 4,
    B = 5,
    C = 6,
    D = 7,
    E = 8,
    F = 9,
    G = 10,
    H = 11,
    I = 12,
    J = 13,
    K = 14,
    L = 15,
    M = 16,
    N = 17,
    O = 18,
    P = 19,
    Q = 20,
    R = 21,
    S = 22,
    T = 23,
    U = 24,
    V = 25,
    W = 26,
    X = 27,
    Y = 28,
    Z = 29,

    _1 = 30,
    _2 = 31,
    _3 = 32,
    _4 = 33,
    _5 = 34,
    _6 = 35,
    _7 = 36,
    _8 = 37,
    _9 = 38,
    _0 = 39,

    Return = 40,
    Escape = 41,
    Backspace = 42,
    Tab = 43,
    Space = 44,

    Minus = 45,
    Equals = 46,
    LeftBracket = 47,
    RightBracket = 48,
    /// Located at the lower left of the return key on ISO keyboards and at the right end of the
    /// QWERTY row on ANSI keyboards. Produces REVERSE SOLIDUS (backslash) and VERTICAL LINE in a
    /// US layout, REVERSE SOLIDUS and VERTICAL LINE in a UK Mac layout, NUMBER SIGN and TILDE in a
    /// UK Windows layout, DOLLAR SIGN and POUND SIGN in a Swiss German layout, NUMBER SIGN and
    /// APOSTROPHE in a German layout, GRAVE ACCENT and POUND SIGN in a French Mac layout, and
    /// ASTERISK and MICRO SIGN in a French Windows layout.
    Backslash = 49,

    /// ISO USB keyboards actually use this code instead of 49 for the same key, but all OSes I've
    /// seen treat the two codes identically. So, as an implementor, unless your keyboard generates
    /// both of those codes and your OS treats them differently, you should generate BACKSLASH
    /// instead of this code. As a user, you should not rely on this code because SDL will never
    /// generate it with most (all?) keyboards.
    NonUsHash = 50,

    Semicolon = 51,
    Apostrophe = 52,

    /// Located in the top left corner (on both ANSI and ISO keyboards). Produces GRAVE ACCENT and
    /// TILDE in a US Windows layout and in US and UK Mac layouts on ANSI keyboards, GRAVE ACCENT
    /// and NOT SIGN in a UK Windows layout, SECTION SIGN and PLUS-MINUS SIGN in US and UK Mac
    /// layouts on ISO keyboards, SECTION SIGN and DEGREE SIGN in a Swiss German layout (Mac: only
    /// on ISO keyboards), CIRCUMFLEX ACCENT and DEGREE SIGN in a German layout (Mac: only on ISO
    /// keyboards), SUPERSCRIPT TWO and TILDE in a French Windows layout, COMMERCIAL AT and NUMBER
    /// SIGN in a French Mac layout on ISO keyboards, and LESS-THAN SIGN and GREATER-THAN SIGN in a
    /// Swiss German, German, or French Mac layout on ANSI keyboards.
    Grave = 53,
    Comma = 54,
    Period = 55,
    Slash = 56,

    CapsLock = 57,

    F1 = 58,
    F2 = 59,
    F3 = 60,
    F4 = 61,
    F5 = 62,
    F6 = 63,
    F7 = 64,
    F8 = 65,
    F9 = 66,
    F10 = 67,
    F11 = 68,
    F12 = 69,

    PrintScreen = 70,
    ScrollLock = 71,
    Pause = 72,

    /// insert on PC, help on some Mac keyboards (but does send code 73, not 117)
    Insert = 73,
    Home = 74,
    PageUp = 75,
    Delete = 76,
    End = 77,
    PageDown = 78,
    Right = 79,
    Left = 80,
    Down = 81,
    Up = 82,

    /// num lock on PC, clear on Mac keyboards
    NumLockClear = 83,
    KeyPadDivide = 84,
    KeyPadMultiply = 85,
    KeyPadMinus = 86,
    KeyPadPlus = 87,
    KeyPadEnter = 88,
    KeyPad1 = 89,
    KeyPad2 = 90,
    KeyPad3 = 91,
    KeyPad4 = 92,
    KeyPad5 = 93,
    KeyPad6 = 94,
    KeyPad7 = 95,
    KeyPad8 = 96,
    KeyPad9 = 97,
    KeyPad0 = 98,
    KeyPadPeriod = 99,

    /// This is the additional key that ISO keyboards have over ANSI ones, located between left
    /// shift and Z. Produces GRAVE ACCENT and TILDE in a US or UK Mac layout, REVERSE SOLIDUS
    /// (backslash) and VERTICAL LINE in a US or UK Windows layout, and LESS-THAN SIGN and
    /// GREATER-THAN SIGN in a Swiss German, German, or French layout.
    NonUsBackslash = 100,

    /// windows contextual menu, compose
    Application = 101,

    /// The USB document says this is a status flag, not a physical key - but some Mac keyboards do
    /// have a power key.
    Power = 102,

    KeyPadEquals = 103,
    F13 = 104,
    F14 = 105,
    F15 = 106,
    F16 = 107,
    F17 = 108,
    F18 = 109,
    F19 = 110,
    F20 = 111,
    F21 = 112,
    F22 = 113,
    F23 = 114,
    F24 = 115,
    Execute = 116,

    /// AL Integrated Help Center
    Help = 117,

    /// Menu (show menu)
    Menu = 118,
    Select = 119,

    /// AC Stop
    Stop = 120,

    /// AC Redo/Repeat
    Again = 121,

    /// AC Undo
    Undo = 122,

    /// AC Cut
    Cut = 123,

    /// AC Copy
    Copy = 124,

    /// AC Paste
    Paste = 125,

    /// AC Find
    Find = 126,
    Mute = 127,
    VolumeUp = 128,
    VolumeDown = 129,

    KeyPadComma = 133,
    KeyPadEqualsAs400 = 134,

    /// used on Asian keyboards, see footnotes in USB doc
    International1 = 135,
    International2 = 136,

    /// Yen
    International3 = 137,
    International4 = 138,
    International5 = 139,
    International6 = 140,
    International7 = 141,
    International8 = 142,
    International9 = 143,

    /// Hangul/English toggle
    Lang1 = 144,

    /// Hanja conversion
    Lang2 = 145,

    /// Katakana
    Lang3 = 146,

    /// Hiragana
    Lang4 = 147,

    /// Zenkaku/Hankaku
    Lang5 = 148,

    /// reserved
    Lang6 = 149,

    /// reserved
    Lang7 = 150,

    /// reserved
    Lang8 = 151,

    /// reserved
    Lang9 = 152,

    /// Erase-Eaze
    AlterAse = 153,
    SysReq = 154,

    /// AC Cancel
    Cancel = 155,
    Clear = 156,
    Prior = 157,
    Return2 = 158,
    Separator = 159,
    Out = 160,
    Oper = 161,
    ClearAgain = 162,
    CrSel = 163,
    ExSel = 164,

    KeyPad00 = 176,
    KeyPad000 = 177,
    ThousandsSeparator = 178,
    DecimalSeparator = 179,
    CurrencyUnit = 180,
    CurrencySubUnit = 181,
    KeyPadLeftParen = 182,
    KeyPadRightParen = 183,
    KeyPadLeftBrace = 184,
    KeyPadRightBrace = 185,
    KeyPadTab = 186,
    KeyPadBackspace = 187,
    KeyPadA = 188,
    KeyPadB = 189,
    KeyPadC = 190,
    KeyPadD = 191,
    KeyPadE = 192,
    KeyPadF = 193,
    KeyPadXor = 194,
    KeyPadPower = 195,
    KeyPadPercent = 196,
    KeyPadLess = 197,
    KeyPadGreater = 198,
    KeyPadAmpersand = 199,
    KeyPadDblAmpersan = 200,
    KeyPadVerticalBar = 201,
    KeyPadDblVerticalBar = 202,
    KeyPadColon = 203,
    KeyPadHash = 204,
    KeyPadSpace = 205,
    KeyPadAt = 206,
    KeyPadExclaim = 207,
    KeyPadMemStore = 208,
    KeyPadMemRecall = 209,
    KeyPadMemClear = 210,
    KeyPadMemAdd = 211,
    KeyPadMemSubtract = 212,
    KeyPadMemMultiply = 213,
    KeyPadMemDivide = 214,
    KeyPadPlusMinus = 215,
    KeyPadClear = 216,
    KeyPadClearEntry = 217,
    KeyPadBinary = 218,
    KeyPadOctal = 219,
    KeyPadDecimal = 220,
    KeyPadHexadecimal = 221,

    LCtrl = 224,
    LShift = 225,

    /// alt, option
    LAlt = 226,

    /// windows, command (apple), meta
    LGui = 227,
    RCtrl = 228,
    RShift = 229,

    /// alt gr, option
    RAlt = 230,

    /// windows, command (apple), meta
    RGui = 231,

    /// I'm not sure if this is really not covered by any of the above, but since there's a special
    /// [`SDL_KMOD_MODE`] for it I'm adding it here
    Mode = 257,

    /* @} *//* Usage page 0x07 */
    /**
     *  \name Usage page 0x0C
     *
     *  These values are mapped from usage page 0x0C (USB consumer page).
     *
     *  There are way more keys in the spec than we can represent in the
     *  current scancode range, so pick the ones that commonly come up in
     *  real world usage.
     */

    /// Sleep
    Sleep = 258,

    /// Wake       
    Wake = 259,

    /// Channel Increment
    ChannelIncrement = 260,

    /// Channel Decrement
    ChannelDecrement = 261,

    /// Play
    MediaPlay = 262,

    /// Pause
    MediaPause = 263,

    /// Record
    MediaRecord = 264,

    /// Fast Forward
    MediaFastForward = 265,

    /// Rewind
    MediaRewind = 266,

    /// Next Track
    MediaNextTrack = 267,

    /// Previous Track
    MediaPreviousTrack = 268,

    /// Stop
    MediaStop = 269,

    /// Eject
    MediaEject = 270,

    /// Play / Pause
    MediaPlayPause = 271,

    /// Media Select
    MediaSelect = 272,

    /// AC New
    AcNew = 273,

    /// AC Open
    AcOpen = 274,

    /// AC Close
    AcClose = 275,

    /// AC Exit
    AcExit = 276,

    /// AC Save
    AcSave = 277,

    /// AC Print
    AcPrint = 278,

    /// AC Properties
    AcProperties = 279,

    /// AC Search
    AcSearch = 280,

    /// AC Home
    AcHome = 281,

    /// AC Back
    AcBack = 282,

    /// AC Forward
    AcForward = 283,

    /// AC Stop
    AcStop = 284,

    /// AC Refresh
    AcRefresh = 285,

    /// AC Bookmarks
    AcBookmarks = 286,

    /* @} *//* Usage page 0x0C */
    /**
     *  \name Mobile keys
     *
     *  These are values that are often used on mobile phones.
     */
    /* @{ */

    /// Usually situated below the display on phones and used as a multi-function feature key for
    /// selecting a software defined function shown on the bottom left of the display.
    SoftLeft = 287,

    /// Usually situated below the display on phones and used as a multi-function feature key for
    /// selecting a software defined function shown on the bottom right of the display.
    SoftRight = 288,

    /// Used for accepting phone calls.
    Call = 289,

    /// Used for rejecting phone calls.
    EndCall = 290,

    /* @} *//* Mobile keys */

    /* Add any other keys here. */
    /// 400-500 reserved for dynamic keycodes
    Reserved = 400,

    /// not a key, just marks the number of scancodes for array bounds
    Count = 512,
}
