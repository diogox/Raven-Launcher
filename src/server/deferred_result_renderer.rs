use timer::{
    Timer,
    Guard,
};

use ::api::response::Response;
use ::api::events::base_event::BaseEvent;
use ::api::actions::do_nothing::DoNothingAction;
use super::extension::Extension;

/// Handles asynchronous render for extensions
pub struct DeferredResultRenderer {
    timer: Timer,
    loading: Option<Guard>,
    active_event: Option< Box<dyn BaseEvent> >,
    active_extension: Option<Extension>,
}

impl DeferredResultRenderer {

    pub fn new() -> Self {

        DeferredResultRenderer {
            timer: Timer::new(),
            loading: None,
            active_event: None,
            active_extension: None,
        }
    }

    pub fn active_extension(&self) -> &Option<Extension> {
        &self.active_extension
    }

    pub fn handle_event(&mut self, event: impl BaseEvent + 'static, extension: Extension) -> DoNothingAction {
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
        self.active_extension = Some(extension);

        DoNothingAction::new()
    }

    pub fn handle_response(&mut self, response: Response, extension: Extension) {
        use ::api::actions::base_action::BaseAction;

        // Check if it's the same extension or event
        {
            let active_extension = self.active_extension.as_mut().unwrap();
            if active_extension.id() != extension.id()
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
        action.run().unwrap();

        if !keep_app_open {
            // TODO: Hide window and clear input
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
        render.run().unwrap();
    }
}