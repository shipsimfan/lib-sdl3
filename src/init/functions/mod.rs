mod get_app_metadata_property;
mod init;
mod init_sub_system;
mod quit;
mod quit_sub_system;
mod set_app_metadata;
mod set_app_metadata_property;

pub use get_app_metadata_property::SDL_GetAppMetadataProperty;
pub use init::SDL_Init;
pub use init_sub_system::SDL_InitSubSystem;
pub use quit::SDL_Quit;
pub use quit_sub_system::SDL_QuitSubSystem;
pub use set_app_metadata::SDL_SetAppMetadata;
pub use set_app_metadata_property::SDL_SetAppMetadataProperty;
