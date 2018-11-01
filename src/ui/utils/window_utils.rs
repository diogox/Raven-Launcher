use gtk::Window;

use super::window::{
    transparency::set_transparent,
    positioning::position_window_in_monitor,
};

pub trait WindowUtils {
    fn set_transparent(&self, _: bool);
    fn set_position_in_monitor(&self, _:bool);
}

impl WindowUtils for Window {

    fn set_transparent(&self, _: bool) {
        set_transparent(self);
    }

    fn set_position_in_monitor(&self, _:bool) {
        position_window_in_monitor(self);
    }
}