use gtk;
use gtk::{
    Builder,
};

#[derive(Clone)]
pub struct Launcher {
    pub window: gtk::Window,
    pub results: gtk::Box,
    pub preferences: gtk::Button,
    pub entry: gtk::Entry,
}

impl Launcher {

    pub fn new() -> Self {

        // Build from glade file
        let app = Builder::new_from_string(include_str!("ui.glade"));

        // Get Window
        let window = app.get_object("window")
            .unwrap();

        // Get Results box
        let results = app.get_object("result_box")
            .unwrap();

        // Get Preferences button
        let preferences = app.get_object("prefs_btn")
            .unwrap();

        // Get Entry box
        let entry = app.get_object("input")
            .unwrap();

        // Apply styles
        let theme = include_str!("theme.css");
        set_theme(&window, theme);

        Launcher {
            window,
            results,
            preferences,
            entry,
        }
    }
}

// Theme Logic

use gtk::{
    STYLE_PROVIDER_PRIORITY_APPLICATION,
    CssProvider,
    CssProviderExt,
    WidgetExt,
    StyleContext,
    StyleContextExt
};

fn set_theme(window: &gtk::Window, theme_dir: &str) {
    let theme_dir = theme_dir.as_bytes();
    let style_context = window.get_style_context().unwrap();
    let provider = CssProvider::new();

    provider.load_from_data( theme_dir ).unwrap();

    // Apparently, adding the styles to the current Widget only doesn't work
    // We must add the styles to the whole app by adding a provider to the screen9
    StyleContext::add_provider_for_screen(&style_context.get_screen().unwrap(), &provider, STYLE_PROVIDER_PRIORITY_APPLICATION);
}
