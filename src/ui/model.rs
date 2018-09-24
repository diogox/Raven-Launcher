use relm_core::{
    Sender,
    Channel,
};
use super::launcher_msg::LauncherMsg;

pub struct Model {
    // Add things like result_items and such...
    pub _channel: Channel<LauncherMsg>,
} 