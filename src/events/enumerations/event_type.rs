// rustdoc imports
#[allow(unused_imports)]
use crate::{SDL_AddEventWatch, SDL_RegisterEvents};

/// The types of events that can be delivered.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum SDL_EventType {
    /// Unused (do not remove)
    First = 0,

    /* Application events */
    /// User-requested quit
    Quit = 0x100,

    /* These application events have special meaning on iOS and Android */
    /// The application is being terminated by the OS. This event must be handled in a callback set
    /// with [`SDL_AddEventWatch`].
    ///
    /// Called on iOS in `applicationWillTerminate`
    ///
    /// Called on Android in `onDestroy`
    Terminating,

    /// The application is low on memory, free memory if possible. This event must be handled in a
    /// callback set with [`SDL_AddEventWatch`].
    ///
    /// Called on iOS in `applicationDidReceiveMemoryWarning`
    ///
    /// Called on Android in `onTrimMemory`
    LowMemory,

    /// The application is about to enter the background. This event must be handled in a callback
    /// set with [`SDL_AddEventWatch`].
    ///
    /// Called on iOS in `applicationWillResignActive`
    ///
    /// Called on Android in `onPause`
    WillEnterBackground,

    /// The application did enter the background and may not get CPU for some time. This event must
    /// be handled in a callback set with [`SDL_AddEventWatch`].
    ///
    /// Called on iOS in `applicationDidEnterBackground`
    ///
    /// Called on Android in `onPause`
    DidEnterBackground,

    /// The application is about to enter the foreground. This event must be handled in a callback
    /// set with [`SDL_AddEventWatch`].
    ///
    /// Called on iOS in `applicationWillEnterForeground`
    ///
    /// Called on Android in `onResume`
    WillEnterForeground,

    /// The application is now interactive. This event must be handled in a callback set with
    /// [`SDL_AddEventWatch`].
    ///
    /// Called on iOS in `applicationDidBecomeActive`
    ///
    /// Called on Android in `onResume`
    DidEnterForeground,

    /// The user's locale preferences have changed.
    LocaleChanged,

    /// The system theme changed
    SystemThemeChanged,

    /* Display events */

    /* 0x150 was `SDL_DISPLAYEVENT`, reserve the number for sdl2-compat */
    /// Display orientation has changed to data1
    DisplayOrientation = 0x151,

    /// Display has been added to the system
    DisplayAdded,

    /// Display has been removed from the system
    DisplayRemoved,

    /// Display has changed position
    DisplayMoved,

    /// Display has changed desktop mode
    DisplayDesktopModeChanged,

    /// Display has changed current mode
    DisplayCurrentModeChanged,

    /// Display has changed content scale
    DisplayContentScaleChanged,

    /// Display has changed usable bounds
    DisplayUsableBoundsChanged,

    /* Window events */

    /* 0x200 was `SDL_WINDOWEVENT`, reserve the number for sdl2-compat */

    /* 0x201 was `SDL_SYSWMEVENT`, reserve the number for sdl2-compat */
    /// Window has been shown
    WindowShown = 0x202,

    /// Window has been hidden
    WindowHidden,

    /// Window has been exposed and should be redrawn, and can be redrawn directly from event
    /// watchers for this event.
    ///
    /// `data1` is 1 for live-resize expose events, 0 otherwise.
    WindowExposed,

    /// Window has been moved to `data1`, `data2`
    WindowMoved,

    /// Window has been resized to `data1xdata2`
    WindowResized,

    /// The pixel size of the window has changed to data1xdata2
    WindowPixelSizeChanged,

    /// The pixel size of a Metal view associated with the window has changed
    WindowMetalViewResized,

    /// Window has been minimized
    WindowMinimized,

    /// Window has been maximized
    WindowMaximized,

    /// Window has been restored to normal size and position
    WindowRestored,

    /// Window has gained mouse focus
    WindowMouseEnter,

    /// Window has lost mouse focus
    WindowMouseLeave,

    /// Window has gained keyboard focus
    WindowFocusGained,

    /// Window has lost keyboard focus
    WindowFocusLost,

    /// The window manager requests that the window be closed
    WindowCloseRequested,

    /// Window had a hit test that wasn't [`SDL_HITTEST_NORMAL`]
    WindowHitTest,

    /// The ICC profile of the window's display has changed
    WindowIccProfChanged,

    /// Window has been moved to display data1
    WindowDisplayChanged,

    /// Window display scale has been changed
    WindowDisplayScaleChanged,

    /// The window safe area has been changed
    WindowSafeAreaChanged,

    /// The window has been occluded
    WindowOcculuded,

    /// The window has entered fullscreen mode
    WindowEnterFullscreen,

    /// The window has left fullscreen mode
    WindowLeaveFullscreen,

    /// The window with the associated ID is being or has been destroyed. If this message is being
    /// handled in an event watcher, the window handle is still valid and can still be used to
    /// retrieve any properties associated with the window. Otherwise, the handle has already been
    /// destroyed and all resources associated with it are invalid
    WindowDestroyed,

    /// Window HDR properties have changed
    WindowHdrStateChanged,

    /* Keyboard events */
    /// Key pressed
    KeyDown = 0x300,

    /// Key released
    KeyUp,

    /// Keyboard text editing (composition)
    TextEditing,

    /// Keyboard text input
    TextInput,

    /// Keymap changed due to a system event such as an input language or keyboard layout change.
    KeymapChanged,

    /// A new keyboard has been inserted into the system
    KeyboardAdded,

    /// A keyboard has been removed
    KeyboardRemoved,

    /// Keyboard text editing candidates
    TextEditingCandidates,

    /// The on-screen keyboard has been shown
    ScreenKeyboardShown,

    /// The on-screen keyboard has been hidden
    ScreenKeyboardHidden,

    /* Mouse events */
    /// Mouse moved
    MouseMotion = 0x400,

    /// Mouse button pressed
    MouseButtonDown,

    /// Mouse button released
    MouseButtonUp,

    /// Mouse wheel motion
    MouseWheel,

    /// A new mouse has been inserted into the system
    MouseAdded,

    /// A mouse has been removed
    MouseRemoved,

    /* Joystick events */
    /// Joystick axis motion
    JoystickAxisMotion = 0x600,

    /// Joystick trackball motion
    JoystickBallMotion,

    /// Joystick hat position change
    JoystickHatMotion,

    /// Joystick button pressed
    JoystickButtonDown,

    /// Joystick button released
    JoystickButtonUp,

    /// A new joystick has been inserted into the system
    JoystickAdded,

    /// An opened joystick has been removed
    JoystickRemoved,

    /// Joystick battery level change
    JoystickBatteryUpdated,

    /// Joystick update is complete
    JoystickUpdateComplete,

    /* Gamepad events */
    /// Gamepad axis motion
    GamepadAxisMotion = 0x650,

    /// Gamepad button pressed
    GamepadButtonDown,

    /// Gamepad button released
    GamepadButtonUp,

    /// A new gamepad has been inserted into the system
    GamepadAdded,

    /// A gamepad has been removed
    GamepadRemoved,

    /// The gamepad mapping was updated
    GamepadRemapped,

    /// Gamepad touchpad was touched
    GamepadTouchpadDown,

    /// Gamepad touchpad finger was moved
    GamepadTouchpadMotion,

    /// Gamepad touchpad finger was lifted
    GamepadTouchpadUp,

    /// Gamepad sensor was updated
    GamepadSensorUpdate,

    /// Gamepad update is complete
    GamepadUpdateComplete,

    /// Gamepad Steam handle has changed
    GamepadSteamHandleUpdated,

    /* Touch events */
    #[allow(missing_docs)]
    FingerDown = 0x700,

    #[allow(missing_docs)]
    FingerUp,

    #[allow(missing_docs)]
    FingerMotion,

    #[allow(missing_docs)]
    FingerCanceled,

    /* Pinch events */
    /// Pinch gesture started
    PinchBegin = 0x710,

    /// Pinch gesture updated
    PinchUpdate,

    /// Pinch gesture ended
    PinchEnd,

    /* 0x800, 0x801, and 0x802 were the Gesture events from SDL2. Do not reuse these values!
       sdl2-compat needs them! */

    /* Clipboard events */
    /// The clipboard changed
    ClipboardUpdate = 0x900,

    /* Drag and drop events */
    /// The system requests a file open
    DropFile = 0x1000,

    /// text/plain drag-and-drop event
    DropText,

    /// A new set of drops is beginning (NULL filename)
    DropBegin,

    /// Current set of drops is now complete (NULL filename)
    DropComplete,

    /// Position while moving over the window
    DropPosition,

    /* Audio hotplug events */
    /// A new audio device is available
    AudioDeviceAdded = 0x1100,

    /// An audio device has been removed.
    AudioDeviceRemoved,

    /// An audio device's format has been changed by the system.
    AudioDeviceFormatChanged,

    /* Sensor events */
    /// A sensor was updated
    SensorUpdate = 0x1200,

    /* Pressure-sensitive pen events */
    /// Pressure-sensitive pen has become available
    PenProximityIn = 0x1300,

    /// Pressure-sensitive pen has become unavailable
    PenProximityOut,

    /// Pressure-sensitive pen touched drawing surface
    PenDown,

    /// Pressure-sensitive pen stopped touching drawing surface
    PenUp,

    /// Pressure-sensitive pen button pressed
    PenButtonDown,

    /// Pressure-sensitive pen button released
    PenButtonUp,

    /// Pressure-sensitive pen is moving on the tablet
    PenMotion,

    /// Pressure-sensitive pen angle/pressure/etc changed
    PenAxis,

    /* Camera hotplug events */
    /// A new camera device is available
    CameraDeviceAdded = 0x1400,

    /// A camera device has been removed.
    CameraDeviceRemoved,

    /// A camera device has been approved for use by the user.
    CameraDeviceApproved,

    /// A camera device has been denied for use by the user.
    CameraDeviceDenied,

    /* Render events */
    /// The render targets have been reset and their contents need to be updated
    RenderTargetsReset = 0x2000,

    /// The device has been reset and all textures need to be recreated
    RenderDeviceReset,

    /// The device has been lost and can't be recovered.
    RenderDeviceLost,

    /// Reserved events for private platforms
    Private0 = 0x4000,

    /// Reserved events for private platforms
    Private1,

    /// Reserved events for private platforms
    Private2,

    /// Reserved events for private platforms
    Private3,

    /* Internal events */
    /// Signals the end of an event poll cycle
    PollSentinel = 0x7F00,

    /// Events [`SDL_EventType::User`] through [`SDL_EventType::Last`] are for your use, and should
    /// be allocated with [`SDL_RegisterEvents`]
    User = 0x8000,

    /// This last event is only for bounding internal arrays
    Last = 0xFFFF,

    /// This just makes sure the enum is the size of [`u32`]
    EnumPadding = 0x7FFFFFFF,
}
