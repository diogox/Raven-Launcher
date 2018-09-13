// TODO: This is should be `pub(crate)` so that
// extension makers can't call `run()` or `keep_app_open()` 
pub mod base_action;

mod constants;

#[macro_use]
pub mod action_list;
pub mod copy_to_clipboard; // TODO: 'run' method!
pub mod do_nothing;
pub mod extension_custom_action; // TODO: 'run' method
pub mod hide_window;
pub mod launch_app; // TODO: 'run' method!
pub mod open; // TODO: 'run' method!
pub mod open_url;
pub mod render_result_list; // TODO: 'run' method & ResultItem trait objects!
//pub mod run_script; //! This concept might not work in Rust!
pub mod set_user_query; // TODO: 'run' method!

pub use self::action_list::ActionList;
pub use self::copy_to_clipboard::CopyToClipboardAction;
pub use self::do_nothing::DoNothingAction;
pub use self::extension_custom_action::ExtensionCustomAction;
pub use self::hide_window::HideWindowAction;
pub use self::launch_app::LaunchAppAction;
pub use self::open_url::OpenUrlAction;
pub use self::open::OpenAction;
pub use self::render_result_list::RenderResultListAction;
//pub use self::run_script::RunScriptAction;
pub use self::set_user_query::SetUserQueryAction;