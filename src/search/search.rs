use std::sync::{
    Arc,
    Mutex,
};
use relm_core::Sender;
use downcast_rs::Downcast;

use ::ui::launcher_msg::LauncherMsg;
use ::server::deferred_result_renderer::DeferredResultRenderer;
use super::base_search_mode::BaseSearchMode;
use super::extension_search_mode::ExtensionSearchMode;

pub struct Search  {
    search_modes: Vec< Box<dyn BaseSearchMode> >,
    gui_controller: Arc< Mutex<Sender<LauncherMsg>> >,
}

impl Search {

    pub fn new(result_renderer: Arc< Mutex<DeferredResultRenderer> >) -> Self {

        let gui_controller = Arc::clone(&result_renderer.lock().unwrap().launcher_gui);

        // Initialize search modes
        let extension_search_mode = ExtensionSearchMode::new(result_renderer);
        // TODO: Add other search modes

        let mut search_modes = Vec::new();
        search_modes.push(Box::new(extension_search_mode) as Box<BaseSearchMode>);

        Search {
            search_modes,
            gui_controller,
        }
    }

    /// Iterate over all search modes and run first enabled.
    // TODO: Make 'AppSearchMode is always enabled' a true statement
    pub fn on_query_change(&mut self, query: &str) {
        let query = String::from(query);

        // 
        self.search_modes.iter_mut().for_each(|mode| {
            mode.on_query(&query);
        });

        // ! Rust, why you do this...
        let controller = Arc::clone(&self.gui_controller);
        // Run active search mode
        if let Some(mode) = self.choose_search_mode(&query) {

            let base_action = mode.handle_query(&query);

            use ::api::actions::constants::*;

            let mut action_type = ""; 

            // ! Seriously, why? 
            // Downcast Action
            {
                if let Some(_action) = base_action.downcast_ref::<::api::actions::action_list::ActionList>() {
                    action_type = ACTION_LIST;
                } else if let Some(_action) = base_action.downcast_ref::<::api::actions::copy_to_clipboard::CopyToClipboardAction>() {
                    action_type = COPY_TO_CLIPBOARD_ACTION;
                } else if let Some(_action) = &base_action.downcast_ref::<::api::actions::do_nothing::DoNothingAction>() {
                    action_type = DO_NOTHING_ACTION;
                } else if let Some(_action) = base_action.downcast_ref::<::api::actions::extension_custom_action::ExtensionCustomAction>() {
                    action_type = EXTENSION_CUSTOM_ACTION;
                } else if let Some(_action) = base_action.downcast_ref::<::api::actions::hide_window::HideWindowAction>() {
                    action_type = HIDE_WINDOW_ACTION;
                } else if let Some(_action) = base_action.downcast_ref::<::api::actions::launch_app::LaunchAppAction>() {
                    action_type = LAUNCH_WINDOW_ACTION;
                } else if let Some(_action) = base_action.downcast_ref::<::api::actions::open_url::OpenUrlAction>() {
                    action_type = OPEN_URL_ACTION;
                } else if let Some(_action) = base_action.downcast_ref::<::api::actions::open::OpenAction>() {
                    action_type = OPEN_ACTION;
                } else if let Some(_action) = base_action.downcast_ref::<::api::actions::render_result_list::RenderResultListAction>() {
                    action_type = RENDER_RESULT_LIST_ACTION;
                } else if let Some(_action) = base_action.downcast_ref::<::api::actions::set_user_query::SetUserQueryAction>() {
                    action_type = SET_USER_QUERY_ACTION;
                };
            }

            match action_type {
                ACTION_LIST => {
                    let action = base_action.downcast::<::api::actions::action_list::ActionList>()
                        .unwrap();
                    action.run(&controller)
                        .unwrap(); // TODO: Deal with Result
                },
                COPY_TO_CLIPBOARD_ACTION => {
                    let action = base_action.downcast::<::api::actions::copy_to_clipboard::CopyToClipboardAction>()
                        .unwrap();
                    action.run(&controller)
                        .unwrap(); // TODO: Deal with Result
                },
                DO_NOTHING_ACTION => {
                    let action = base_action.downcast::<::api::actions::do_nothing::DoNothingAction>()
                        .unwrap();
                    action.run(&controller)
                        .unwrap(); // TODO: Deal with Result
                },
                EXTENSION_CUSTOM_ACTION => {
                    let action = base_action.downcast::<::api::actions::extension_custom_action::ExtensionCustomAction>()
                        .unwrap();
                    action.run(&controller)
                        .unwrap(); // TODO: Deal with Result
                },
                HIDE_WINDOW_ACTION => {
                    let action = base_action.downcast::<::api::actions::hide_window::HideWindowAction>()
                        .unwrap();
                    action.run(&controller)
                        .unwrap(); // TODO: Deal with Result
                },
                LAUNCH_WINDOW_ACTION => {
                    let action = base_action.downcast::<::api::actions::launch_app::LaunchAppAction>()
                        .unwrap();
                    action.run(&controller)
                        .unwrap(); // TODO: Deal with Result
                },
                OPEN_URL_ACTION => {
                    let action = base_action.downcast::<::api::actions::open_url::OpenUrlAction>()
                        .unwrap();
                    action.run(&controller)
                        .unwrap(); // TODO: Deal with Result
                },
                OPEN_ACTION => {
                    let action = base_action.downcast::<::api::actions::open::OpenAction>()
                        .unwrap();
                    action.run(&controller)
                        .unwrap(); // TODO: Deal with Result
                },
                RENDER_RESULT_LIST_ACTION => {
                    let action = base_action.downcast::<::api::actions::render_result_list::RenderResultListAction>()
                        .unwrap();
                    action.run(&controller)
                        .unwrap(); // TODO: Deal with Result
                },
                SET_USER_QUERY_ACTION => {
                    let action = base_action.downcast::<::api::actions::set_user_query::SetUserQueryAction>()
                        .unwrap();
                    action.run(&controller)
                        .unwrap(); // TODO: Deal with Result
                }
                _ => {}
            }

        }
    }

    pub fn on_key_press_event(&mut self, query: &str) {
        let controller = Arc::clone(&self.gui_controller);
        if let Some(mode) = self.choose_search_mode(query) {
            let base_action = mode.handle_key_press_event(query);

            use ::api::actions::constants::*;

            let mut action_type = ""; 

            // ! Can't wait for NLL's... 
            // Downcast Action
            {
                if let Some(_action) = base_action.downcast_ref::<::api::actions::action_list::ActionList>() {
                    action_type = ACTION_LIST;
                } else if let Some(_action) = base_action.downcast_ref::<::api::actions::copy_to_clipboard::CopyToClipboardAction>() {
                    action_type = COPY_TO_CLIPBOARD_ACTION;
                } else if let Some(_action) = &base_action.downcast_ref::<::api::actions::do_nothing::DoNothingAction>() {
                    action_type = DO_NOTHING_ACTION;
                } else if let Some(_action) = base_action.downcast_ref::<::api::actions::extension_custom_action::ExtensionCustomAction>() {
                    action_type = EXTENSION_CUSTOM_ACTION;
                } else if let Some(_action) = base_action.downcast_ref::<::api::actions::hide_window::HideWindowAction>() {
                    action_type = HIDE_WINDOW_ACTION;
                } else if let Some(_action) = base_action.downcast_ref::<::api::actions::launch_app::LaunchAppAction>() {
                    action_type = LAUNCH_WINDOW_ACTION;
                } else if let Some(_action) = base_action.downcast_ref::<::api::actions::open_url::OpenUrlAction>() {
                    action_type = OPEN_URL_ACTION;
                } else if let Some(_action) = base_action.downcast_ref::<::api::actions::open::OpenAction>() {
                    action_type = OPEN_ACTION;
                } else if let Some(_action) = base_action.downcast_ref::<::api::actions::render_result_list::RenderResultListAction>() {
                    action_type = RENDER_RESULT_LIST_ACTION;
                } else if let Some(_action) = base_action.downcast_ref::<::api::actions::set_user_query::SetUserQueryAction>() {
                    action_type = SET_USER_QUERY_ACTION;
                };
            }

            match action_type {
                ACTION_LIST => {
                    let action = base_action.downcast::<::api::actions::action_list::ActionList>()
                        .unwrap();
                    action.run(&controller)
                        .unwrap(); // TODO: Deal with Result
                },
                COPY_TO_CLIPBOARD_ACTION => {
                    let action = base_action.downcast::<::api::actions::copy_to_clipboard::CopyToClipboardAction>()
                        .unwrap();
                    action.run(&controller)
                        .unwrap(); // TODO: Deal with Result
                },
                DO_NOTHING_ACTION => {
                    let action = base_action.downcast::<::api::actions::do_nothing::DoNothingAction>()
                        .unwrap();
                    action.run(&controller)
                        .unwrap(); // TODO: Deal with Result
                },
                EXTENSION_CUSTOM_ACTION => {
                    let action = base_action.downcast::<::api::actions::extension_custom_action::ExtensionCustomAction>()
                        .unwrap();
                    action.run(&controller)
                        .unwrap(); // TODO: Deal with Result
                },
                HIDE_WINDOW_ACTION => {
                    let action = base_action.downcast::<::api::actions::hide_window::HideWindowAction>()
                        .unwrap();
                    action.run(&controller)
                        .unwrap(); // TODO: Deal with Result
                },
                LAUNCH_WINDOW_ACTION => {
                    let action = base_action.downcast::<::api::actions::launch_app::LaunchAppAction>()
                        .unwrap();
                    action.run(&controller)
                        .unwrap(); // TODO: Deal with Result
                },
                OPEN_URL_ACTION => {
                    let action = base_action.downcast::<::api::actions::open_url::OpenUrlAction>()
                        .unwrap();
                    action.run(&controller)
                        .unwrap(); // TODO: Deal with Result
                },
                OPEN_ACTION => {
                    let action = base_action.downcast::<::api::actions::open::OpenAction>()
                        .unwrap();
                    action.run(&controller)
                        .unwrap(); // TODO: Deal with Result
                },
                RENDER_RESULT_LIST_ACTION => {
                    let action = base_action.downcast::<::api::actions::render_result_list::RenderResultListAction>()
                        .unwrap();
                    action.run(&controller)
                        .unwrap(); // TODO: Deal with Result
                },
                SET_USER_QUERY_ACTION => {
                    let action = base_action.downcast::<::api::actions::set_user_query::SetUserQueryAction>()
                        .unwrap();
                    action.run(&controller)
                        .unwrap(); // TODO: Deal with Result
                }
                _ => {}
            }
        }
    }
}

use ::api::actions::base_action::BaseAction;


impl Search {

    fn choose_search_mode(&mut self, query: &str) -> Option< Box<&mut BaseSearchMode> > {
        let query = String::from(query);

        for mode in self.search_modes.iter_mut() {
            if mode.is_enabled(&query) {
                return Some(Box::new(&mut *mode.as_mut()));
            }
        }

        None   
    }
}
