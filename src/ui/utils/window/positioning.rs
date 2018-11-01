use gdk::{
    Display,
    DisplayExt,
    DeviceManagerExt,
    DeviceExt,
    MonitorExt,
    Screen,
    ScreenExt,
    Rectangle,
};

use gtk::{
    Window,
    GtkWindowExt,
};

use super::super::super::windows::launcher::LAUNCHER_WIDTH;

pub fn position_window_in_monitor(window: &Window) {

    // Get monitor geometry
    let monitor_geometry: Rectangle = get_current_monitor_geometry();

    // This causes a bug where, sometimes, it will equal 604 and others 200... no idea why...
    //let (window_width, _) = window.get_size();

    // Get the gtk window width
    let window_width = LAUNCHER_WIDTH;


    // Calculate new position
    let new_x = monitor_geometry.width / 2 - window_width / 2 + monitor_geometry.x;
    let new_y = monitor_geometry.height / 5 + monitor_geometry.y;

    // Position Window
    window.move_(new_x, new_y);

}

fn get_current_monitor_geometry() -> Rectangle {

    // Get default screen
    let main_screen = Screen::get_default();

    // Get display
    let display = Display::get_default()
        .unwrap();

    // Get device manager
    let device_manager = display.get_device_manager()
        .unwrap();

    // Get pointer device
    let pointer = device_manager.get_client_pointer()
        .unwrap();

    // Get pointer position
    let (_, x, y) = pointer.get_position();

    // Get monitor where the pointer is resting at
    let current_monitor = display.get_monitor_at_point(x, y)
        .unwrap();

    return current_monitor.get_geometry();
}