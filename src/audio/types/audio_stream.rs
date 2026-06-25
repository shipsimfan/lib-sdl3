use std::ffi::c_void;

/// The opaque handle that represents an audio stream.
///
/// # Remarks
/// [`SDL_AudioStream`] is an audio conversion interface.
///  - It can handle resampling data in chunks without generating artifacts, when it doesn't have
///    the complete buffer available.
///  - It can handle incoming data in any variable size.
///  - It can handle input/output format changes on the fly.
///  - It can remap audio channels between inputs and outputs.
///  - You push data as you have it, and pull it when you need it; the stream will buffer data as
///    needed.
///  - It can also function as a basic audio data queue even if you just have sound that needs to
///    pass from one place to another.
///  - You can hook callbacks up to them when more data is added or requested, to manage data
///    on-the-fly.
///
/// Audio streams are the core of the SDL3 audio interface. You create one or more of them, bind
/// them to an opened audio device, and feed data to them (or for recording, consume data from
/// them).
#[repr(C)]
#[allow(non_camel_case_types)]
pub struct SDL_AudioStream(c_void);
