use crate::{
    SDL_AudioDeviceEvent, SDL_CameraDeviceEvent, SDL_ClipboardEvent, SDL_CommonEvent,
    SDL_DisplayEvent, SDL_DropEvent, SDL_GamepadAxisEvent, SDL_GamepadButtonEvent,
    SDL_GamepadDeviceEvent, SDL_GamepadSensorEvent, SDL_GamepadTouchpadEvent, SDL_JoyAxisEvent,
    SDL_JoyBallEvent, SDL_JoyBatteryEvent, SDL_JoyButtonEvent, SDL_JoyDeviceEvent, SDL_JoyHatEvent,
    SDL_KeyboardDeviceEvent, SDL_KeyboardEvent, SDL_MouseButtonEvent, SDL_MouseDeviceEvent,
    SDL_MouseMotionEvent, SDL_MouseWheelEvent, SDL_PenAxisEvent, SDL_PenButtonEvent,
    SDL_PenMotionEvent, SDL_PenProximityEvent, SDL_PenTouchEvent, SDL_PinchFingerEvent,
    SDL_QuitEvent, SDL_RenderEvent, SDL_SensorEvent, SDL_TextEditingCandidatesEvent,
    SDL_TextEditingEvent, SDL_TextInputEvent, SDL_TouchFingerEvent, SDL_UserEvent, SDL_WindowEvent,
};

// rustdoc imports
#[allow(unused_imports)]
use crate::SDL_EventType;

/// The structure for all events in SDL.
///
/// # Remarks
/// The [`SDL_Event`] structure is the core of all event handling in SDL. [`SDL_Event`] is a union
/// of all event structures used in SDL.
///
/// # Using events
/// The [`SDL_Event`] structure has two uses:
///  - Reading events from the event queue
///  - Placing events on the event queue
///
/// # Reading events from the event queue
/// Reading events from the event queue is done with either [`SDL_PollEvent`] or
/// [`SDL_PeepEvents`]. We'll use [`SDL_PollEvent`] and step through an example. First off, we
/// create an empty [`SDL_Event`] structure.
/// ```no_run
/// let test_event = SDL_Event::default();
/// ```
///
/// [`SDL_PollEvent`] removes the next event from the event queue. If there are no events on the
/// queue it returns 0, otherwise it returns 1. We use a while loop to process each event in turn.
/// ```no_run
/// while SDL_PollEvent(&test_event) {
/// ```
///
/// The [`SDL_PollEvent`] function takes a pointer to an [`SDL_Event`] structure that is to be
/// filled with event information. We know that if [`SDL_PollEvent`] removes an event from the
/// queue then the event information will be placed in our `test_event` structure, but we also know
/// that the type of event will be placed in the type member of `test_event`. So to handle each
/// event type separately we use a match statement.
/// ```no_run
///     match unsafe { test_event.r#type } {
/// ```
///
/// We need to know what kind of events we're looking for and the event types of those events. So
/// let's assume we want to detect where the user is moving the mouse pointer within our
/// application. We look through our event types and notice that [`SDL_EventType::MouseMotion`] is,
/// more than likely, the event we're looking for. Looking at the table below tells us that
/// [`SDL_EventType::MouseMotion`] events are handled within the [`SDL_MouseMotionEvent`] structure
/// which is the `motion` member of [`SDL_Event`]. We can check for the
/// [`SDL_EventType::MouseMotion`] event type within our match statement like so:
/// ```no_run
///         SDL_EventType::MouseMotion as u32 => {
/// ```
///
/// All we need to do now is read the information out of the `motion` member of `test_event`.
/// ```no_run
///             SDL_Log(c"We got a motion event.".as_ptr());
///             SDL_Log(
///                 c"Current mouse position is: (%f, %f)".as_ptr(),
///                 unsafe { test_event.motion.x },
///                 unsafe { test_event.motion.y }
///             );
///         }
///         _ => SDL_Log(c"Unhandled Event!".as_ptr()),
///     }
/// }
///
/// SDL_Log(c"Event queue empty.".as_ptr());
///```
///
/// # Placing events on the event queue
/// It is also possible to push events onto the event queue and so use it as a two-way
/// communication path. Both [`SDL_PushEvent`] and [`SDL_PeepEvents`] allow you to place events
/// onto the event queue. This is usually used to place a [`SDL_EventType::User`] on the event
/// queue, however you could use it to post fake input events if you wished. Creating your own
/// events is a simple matter of choosing the event type you want, setting the type member and
/// filling the appropriate member structure with information.
/// ```no_run
/// let user_event = SDL_Event::default();
/// SDL_zero(user_event);  /* SDL will copy this entire struct! Initialize to keep memory checkers
///                           happy. */
/// user_event.r#type = SDL_EventType::User as u32;
/// user_event.user.code = 2;
/// user_event.user.data1 = std::ptr::null_mut();
/// user_event.user.data2 = std::ptr::null_mut();
/// SDL_PushEvent(&user_event);
/// ```
///
/// Note that [`SDL_EventType::User`] is just a starting point for event numbers that SDL won't
/// claim for itself, and you should use [`SDL_RegisterEvents`] to claim a unique number that other
/// parts of your app won't also claim.
#[repr(C)]
#[derive(Clone, Copy)]
#[allow(non_camel_case_types, rustdoc::invalid_rust_codeblocks)]
pub union SDL_Event {
    /// Event type, shared with all events, [`u32`] to cover user events which are not in the
    /// [`SDL_EventType`] enumeration
    pub r#type: u32,

    /// Common event data
    pub common: SDL_CommonEvent,

    /// Display event data
    pub display: SDL_DisplayEvent,

    /// Window event data
    pub window: SDL_WindowEvent,

    /// Keyboard device change event data
    pub kdevice: SDL_KeyboardDeviceEvent,

    /// Keyboard event data
    pub key: SDL_KeyboardEvent,

    /// Text editing event data
    pub edit: SDL_TextEditingEvent,

    /// Text editing candidates event data
    pub edit_candidates: SDL_TextEditingCandidatesEvent,

    /// Text input event data
    pub text: SDL_TextInputEvent,

    /// Mouse device change event data
    pub mdevice: SDL_MouseDeviceEvent,

    /// Mouse motion event data
    pub motion: SDL_MouseMotionEvent,

    /// Mouse button event data
    pub button: SDL_MouseButtonEvent,

    /// Mouse wheel event data
    pub wheel: SDL_MouseWheelEvent,

    /// Joystick device change event data
    pub jdevice: SDL_JoyDeviceEvent,

    /// Joystick axis event data
    pub jaxis: SDL_JoyAxisEvent,

    /// Joystick ball event data
    pub jball: SDL_JoyBallEvent,

    /// Joystick hat event data
    pub jhat: SDL_JoyHatEvent,

    /// Joystick button event data
    pub jbutton: SDL_JoyButtonEvent,

    /// Joystick battery event data
    pub jbattery: SDL_JoyBatteryEvent,

    /// Gamepad device event data
    pub gdevice: SDL_GamepadDeviceEvent,

    /// Gamepad axis event data
    pub gaxis: SDL_GamepadAxisEvent,

    /// Gamepad button event data
    pub gbutton: SDL_GamepadButtonEvent,

    /// Gamepad touchpad event data
    pub gtouchpad: SDL_GamepadTouchpadEvent,

    /// Gamepad sensor event data
    pub gsensor: SDL_GamepadSensorEvent,

    /// Audio device event data
    pub adevice: SDL_AudioDeviceEvent,

    /// Camera device event data
    pub cdevice: SDL_CameraDeviceEvent,

    /// Sensor event data
    pub sensor: SDL_SensorEvent,

    /// Quit request event data
    pub quit: SDL_QuitEvent,

    /// Custom event data
    pub user: SDL_UserEvent,

    /// Touch finger event data
    pub tfinger: SDL_TouchFingerEvent,

    /// Pinch event data
    pub pinch: SDL_PinchFingerEvent,

    /// Pen proximity event data
    pub pproximity: SDL_PenProximityEvent,

    /// Pen tip touching event data
    pub ptouch: SDL_PenTouchEvent,

    /// Pen motion event data
    pub pmotion: SDL_PenMotionEvent,

    /// Pen button event data
    pub pbutton: SDL_PenButtonEvent,

    /// Pen axis event data
    pub paxis: SDL_PenAxisEvent,

    /// Render event data
    pub render: SDL_RenderEvent,

    /// Drag and drop event data
    pub drop: SDL_DropEvent,

    /// Clipboard event data
    pub clipboard: SDL_ClipboardEvent,

    /// This is necessary for ABI compatibility between Visual C++ and GCC. Visual C++ will respect
    /// the push pack pragma and use 52 bytes (size of [`SDL_TextEditingEvent`], the largest
    /// structure for 32-bit and 64-bit architectures) for this union, and GCC will use the
    /// alignment of the largest datatype within the union, which is 8 bytes on 64-bit
    /// architectures.
    ///
    /// So... we'll add padding to force the size to be the same for both.
    ///
    /// On architectures where pointers are 16 bytes, this needs rounding up to the next multiple
    /// of 16, 64, and on architectures where pointers are even larger the size of
    /// [`SDL_UserEvent`] will dominate as being 3 pointers.
    padding: [u8; 128],
}

impl Default for SDL_Event {
    fn default() -> Self {
        SDL_Event { padding: [0; 128] }
    }
}
