use gdk::ScreenExt;
use gtk::{
    Window,
    WidgetExt,
};

pub fn set_transparent(window: &Window) {
    let visual = window.get_screen()
        .unwrap()
        .get_rgba_visual()
        .expect("Transparency not available in this machine!");

    window.set_visual(&visual);
}