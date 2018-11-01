use std::sync::{
    Arc,
    Mutex,
    mpsc,
};
use relm_core::Sender;
use timer::{
    Timer,
    Guard,
};

use ::ui::windows::launcher::msg::Msg as LauncherMsg;
use ::api::response::Response;
use ::api::events::base_event::BaseEvent;
use ::api::actions::do_nothing::DoNothingAction;
use super::IdType;

/// Handles asynchronous render for extensions
pub struct DeferredResultRenderer {
    timer: Timer,
    loading: Option<Guard>,
    active_event: Option< Box<dyn BaseEvent> >,
    active_extension: Option<IdType>,
    pub launcher_gui: Arc<Mutex<Sender<LauncherMsg>>>,
}

impl DeferredResultRenderer {

    pub fn new(launcher_gui: Arc< Mutex<Sender<LauncherMsg>> >) -> Self {

        DeferredResultRenderer {
            timer: Timer::new(),
            loading: None,
            active_event: None,
            active_extension: None,
            launcher_gui,
        }
    }

    pub fn active_extension(&self) -> Option<IdType> {
        self.active_extension
    }

    pub fn handle_event(&mut self, event: impl BaseEvent + 'static, extension_id: IdType) -> DoNothingAction {
        use chrono::Duration;

        // Cancel timer if it's running
        self.cancel_loading();

        // Start timer
        // wait 0.3 seconds
        let default_wait_time = Duration::milliseconds(300);
        let loading = self.timer.schedule_with_delay(default_wait_time, || {
            println!("Loading Results...");
        });

        // Keep Guard
        self.loading = Some( loading );

        // Keep active values
        self.active_event = Some(Box::new(event));
        self.active_extension = Some(extension_id);

        DoNothingAction::new()
    }

    pub fn handle_response(&mut self, response: Response, extension_id: IdType) {
        use ::api::actions::base_action::BaseAction;

        // Check if it's the same extension or event
        {
            // ! For testing purposes
            self.active_extension = Some(extension_id);

            let active_extension = self.active_extension.as_mut()
                .expect("Could not unwrap current active extension in result renderer!");
            if *active_extension != extension_id
                //|| self.active_event.unwrap() != response.event.unwrap()  // ! Implement 'PartialEq' on Response
            {
                return;
            }
        }
        
        self.cancel_loading();

        // TODO: Do I need to deserialize it here?
        use serde_json;

        // ! TODO: Implement something to allow downcasting
        let action: DoNothingAction = serde_json::from_str(&response.action).unwrap();
        let keep_app_open = action.keep_app_open();
        action.run(&self.launcher_gui).unwrap();

        if !keep_app_open {
            let controller = self.launcher_gui.lock()
                .unwrap();
            controller.send(LauncherMsg::Hide).unwrap();
            controller.send(LauncherMsg::NavClearResults).unwrap();
        }
    }

    pub fn on_query_change(&mut self) {
        self.cancel_loading();
        self.active_event = None;
        self.active_extension = None;
    }

}

impl DeferredResultRenderer {

    fn cancel_loading(&mut self) {
        if self.loading.is_some() {
            self.loading = None;
        }
    }

    fn render_loading(&self, icon: &str) {
        use ::api::items::ExtensionResultItem;
        use ::api::actions::{
            base_action::BaseAction,
            RenderResultListAction,
        };
        
        let mut loading_item = ExtensionResultItem::new();
        loading_item.set_name("Loading...".to_string());
        loading_item.set_icon(icon.to_string());
        loading_item.set_highlightable(false);

        let mut render = RenderResultListAction::new();
        render.push(loading_item);
        render.run(&self.launcher_gui).unwrap();
    }
}