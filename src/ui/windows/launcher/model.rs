use std::sync::{
    Arc,
    Mutex,
};

use relm::Relm;
use relm_core::{
    Sender,
    Channel,
};

use ::search::search::Search;

use super::launcher_ui::LauncherWindow;
use super::msg::Msg;

pub struct Model {
    pub relm: Relm<LauncherWindow>,
    pub is_window_showing: bool,
    pub _channel: Channel<Msg>,
    pub search: Arc<Mutex<Search>>,
}