/// Various types of file dialogs.
///
/// # Remarks
/// This is used by [`SDL_ShowFileDialogWithProperties`] to decide what kind of dialog to present
/// to the user.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
#[allow(missing_docs)]
pub enum SDL_FileDialogType {
    OpenFile,
    SaveFile,
    OpenFolder,
}
