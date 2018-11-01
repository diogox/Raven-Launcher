use gdk::{
    ScreenExt,
    DisplayExt,
};
use gtk;
use gtk::{
    WidgetExt,
    GtkWindowExt,
    OrientableExt,
    EntryExt,
    EditableExt,
    ButtonExt,
    STYLE_PROVIDER_PRIORITY_APPLICATION,
    CssProvider,
    CssProviderExt,
    StyleContext,
    StyleContextExt,
    ImageExt,
    BoxExt,
    ScrolledWindowExt,
    ContainerExt,
    EditableSignals,
};

use relm::{
    Relm,
    Widget,
};
use relm_attributes::widget;

use super::super::super::navigation::nav_msg::NavMsg;
use super::super::super::utils::WindowUtils;
use super::super::super::result_item::ResultItem;
use ::ui::navigation::ResultsNavigation;

use super::{
    msg::Msg,
    model::Model,
};

#[widget]
impl Widget for LauncherWindow {

    fn init_view(&mut self) {

        // Set style classes
        self.body.get_style_context()
            .unwrap()
            .add_class("app");

        self.input.get_style_context()
            .unwrap()
            .add_class("input");

        self.prefs_btn.get_style_context()
            .unwrap()
            .add_class("prefs-btn");

        /* SET STYLES */

        // Adjust the look of the entry.
        let style_context = self.launcher.get_style_context().unwrap();
        // TODO: remove the next line when relm supports css.
        let style = include_bytes!("ui/windows/launcher/theme.css");
        let provider = CssProvider::new();
        provider.load_from_data(style).unwrap();

        // Apparently, adding the styles to the current Widget only doesn't work
        // We must add the styles to the whole app by adding a provider to the screen.
        StyleContext::add_provider_for_screen(&style_context.get_screen().unwrap(), &provider, STYLE_PROVIDER_PRIORITY_APPLICATION);

        /* Bind Hotkey */
        // TODO: How???

    }

    fn model(relm: &Relm<Self>, _: ()) -> Model {
        use super::super::super::super::server::deferred_result_renderer::DeferredResultRenderer;
        use super::super::super::super::search::search::Search;
        use std::sync::Arc;


        use relm_core::Channel;
        use std::sync::mpsc;


        use::std::thread;

        // Get stream
        let stream = relm.stream().clone();

        // Create a channel to be able to send a message from another thread.
        let (channel, sender) = Channel::new(move |msg| {
            // This closure is executed whenever a message is received from the sender.
            // We send a message to the current widget.
            stream.emit(msg);
        });

        // Create channel to be able to send the queries to the other thread
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            use std::sync::{
                Arc,
                Mutex,
            };
            use ::server::deferred_result_renderer::DeferredResultRenderer;
            use ::search::search::Search;

            let result_renderer = DeferredResultRenderer::new(Arc::new(Mutex::new(sender)));

            let search = Arc::new( Mutex::new(
                Search::new(
                    Arc::new( Mutex::new(result_renderer)))
            )
            );

            // Send
            tx.send(
                Arc::clone(&search)
            ).unwrap();
        });

        let search = rx.recv()
            .unwrap();

        Model {
            relm: relm.clone(),
            is_window_showing: true,
            _channel: channel,
            search,
        }
    }

    fn toggle_window(&self) {
        self.model.relm.stream().emit(Msg::Toggle);
    }

    fn update(&mut self, event: Msg) {
        match event {
            Msg::HandleQuery(query) => {
                self.model.search.lock()
                    .unwrap()
                    .on_query_change(&query);
            },
            Msg::ShowResults(results) => {

                let item = relm::init::<ResultItem>( () )
                    .unwrap();

                let results = results.into_iter()
                    .take(12)
                    .map(|result| {
                        use super::super::super::result_item::msg::ResultItemMsg;

                        let new_result = relm::init::<ResultItem>( () ).unwrap();
                        new_result.stream().emit(ResultItemMsg::SetInfo(result));
                        new_result
                    });

                // Clear previous results
                self.results_navigation.emit(NavMsg::Clear);

                // Display new results
                results.into_iter().for_each(|result| {

                    self.results_navigation.emit(NavMsg::AddResult(result));
                });

            }
            Msg::KeyPress(key) => {
                if key == 65362 {
                    println!("pressed {}", gdk::keyval_name(key).unwrap());
                    self.results_navigation.emit(NavMsg::GoUp);
                } else if key == 65364 {
                    println!("pressed {}", gdk::keyval_name(key).unwrap());
                    self.results_navigation.emit(NavMsg::GoDown);
                } else if key == 65293 {
                    println!("pressed {}", gdk::keyval_name(key).unwrap());
                } else {
                    println!("pressed {}", key);
                }
            },
            Msg::Toggle => {
                if self.model.is_window_showing {
                    // TODO: Hide
                    println!("HIDE");
                } else {
                    // TODO: Show
                    println!("SHOW");
                }
            },
            Msg::Quit => gtk::main_quit(),
            _ => {}
        }
    }

    view! {

        #[name = "launcher"]
        gtk::Window {
            property_width_request: super::LAUNCHER_WIDTH,
            title: "Raven Launcher",
            visible: true,
            resizable: false,
            can_focus: false,
            icon_name: "raven",
            skip_taskbar_hint: true,
            skip_pager_hint: true,
            urgency_hint: true,
            decorated: false,
            deletable: false,
            position: gtk::WindowPosition::Center,
            transparent: true,
            position_in_monitor: true,
            key_press_event(_, key) => (Msg::KeyPress(key.get_keyval()), gtk::Inhibit(false)),
            enter_notify_event(_, _) => ({}, gtk::Inhibit(false)),

            #[name = "body"]
            gtk::Box {
                name: "app", // This is the style's class name as well.
                visible: true,
                app_paintable: true,
                can_focus: false,
                margin_top: 20,
                margin_bottom: 20,
                margin_left: 20,
                margin_right: 20,
                orientation: gtk::Orientation::Vertical,

                // Packing
                hexpand: false,
                valign: gtk::Align::Fill,

                #[name = "input_box"]
                gtk::Box {
                    visible: true,
                    can_focus: false,
                    orientation: gtk::Orientation::Horizontal,

                    #[name = "input"]
                    gtk::Entry {
                        name: "input",
                        visible: true,
                        can_focus: true,
                        property_is_focus: true,
                        property_height_request: 30,
                        margin_top: 15,
                        margin_bottom: 15,
                        margin_left: 20,
                        margin_right: 20,
                        changed(entry) => Msg::HandleQuery(entry.get_text().unwrap()),

                        // Packing
                        hexpand: true,
                        valign: gtk::Align::Fill,
                    },

                    #[name = "prefs_btn"]
                    gtk::Button {
                        name: "prefs-btn",
                        property_height_request: 24,
                        property_width_request: 24,
                        visible: true,
                        can_focus: false,
                        valign: gtk::Align::Center,
                        halign: gtk::Align::Center,
                        margin_right: 15,

                        // TODO: Change path from hard coded to use a dir dedicated to the launcher
                        image: Some(&gtk::Image::new_from_file("/home/diogox/Desktop/raven-gui/target/debug/prefs.svg")),

                        // Packing
                        hexpand: false,
                    },
                },

                #[name = "results_navigation"]
                ResultsNavigation {
                    //GoUp => Msg::NavGoUp,
                    //GoDown => Msg::NavGoDown,
                    //AddResult(item) => Msg::NavAddResult(item),
                    //Clear => Msg::NavClearResults,
                }
            }
        }
    }

}