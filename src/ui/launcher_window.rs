use gtk::{
    Inhibit,
    WidgetExt,
    Window,
};
use relm::{Channel, Relm, Update, Widget};

use super::model::Model;
use super::launcher_msg::LauncherMsg;
use super::launcher::Launcher;

pub struct Win {
    model: Model,
    widgets: Launcher,
}

impl Update for Win {
    // Specify the model used for this widget.
    type Model = Model;
    // Specify the model parameter used to init the model.
    type ModelParam = ();
    // Specify the type of the messages sent to the update function.
    type Msg = LauncherMsg;

    fn model(relm: &Relm<Self>, _: ()) -> Model {
        use::std::thread;

        // Get stream
        let stream = relm.stream().clone();
        // Create a channel to be able to send a message from another thread.
        let (channel, sender) = Channel::new(move |msg| {
            // This closure is executed whenever a message is received from the sender.
            // We send a message to the current widget.
            stream.emit(msg);
        });

        thread::spawn(move || {
            use std::sync::{
                Arc,
                Mutex,
            };
            use ::server::deferred_result_renderer::DeferredResultRenderer;
            use ::search::search::Search;
            // TODO: Start search here?

            let result_renderer = DeferredResultRenderer::new(Arc::new(Mutex::new(sender)));
            Search::new(Arc::new(Mutex::new(result_renderer)));
            //sender.send(LauncherMsg::Hide).expect("send message");
        });

        Model {
            _channel: channel,
        }
    }

    fn update(&mut self, event: LauncherMsg) {
        use gtk::EditableExt;
        use ::api::items::result_item::ResultItem;
        
        //let label = &self.widgets.counter_label;
    
        match event {
            LauncherMsg::ShowResults(results) => {
                for result in results {
                    println!("{}", 
                        result.name().unwrap()
                    );
                }
            },
            LauncherMsg::ClearInput => {
                self.widgets.entry.set_position(-1);
                let last = self.widgets.entry.get_position();
                self.widgets.entry.delete_text(0, last);
            },
            LauncherMsg::Hide => {
                println!("Hiding!");
            },
            LauncherMsg::Quit => gtk::main_quit(),
        }
    }
}

impl Widget for Win {
    // Specify the type of the root widget.
    type Root = Window;

    // Return the root widget.
    fn root(&self) -> Self::Root {
        self.widgets.window.clone()
    }

    fn view(relm: &Relm<Self>, model: Self::Model) -> Self {
        use gtk::ButtonExt;
        use gtk::EditableSignals;

        let launcher = Launcher::new();

        launcher.window.show_all();

        // Send the message Increment when the button is clicked.
        connect!(relm, launcher.window, connect_delete_event(_, _), return (Some(LauncherMsg::Quit), Inhibit(false)));
        connect!(relm, launcher.preferences, connect_clicked(_), LauncherMsg::Hide);
        connect!(relm, launcher.entry, connect_changed(_), LauncherMsg::ClearInput);

        Win {
            model,
            widgets: launcher,
        }
    }
}

/*
use std::sync::{
    Arc,
    Mutex,
};

use ::server::deferred_result_renderer::DeferredResultRenderer;
use ::search::search::Search;
use super::launcher_gui::LauncherGUI;

pub struct LauncherWindow {
    
    //gui_controller: Box<dyn LauncherGUI>,
    deferred_result_render: Arc< Mutex<DeferredResultRenderer> >,
    search: Search,
}

impl LauncherWindow {

    // TODO: Make structs that need to change the UI take Arc<Mutex<gui_controller>> as a constructor argument? 
    pub fn with_gui(launcher_gui: impl LauncherGUI + Clone + 'static) -> Self {

        //let gui_controller = Box::new(launcher_gui);
        //let gui_controller_clone = Box::clone(&gui_controller);
        let gui_controller_clone = Box::new(launcher_gui);
        
        let deferred_result_render = Arc::new( Mutex::new(DeferredResultRenderer::new(gui_controller_clone)) );
        let search = Search::new(Arc::clone(&deferred_result_render));

        LauncherWindow {
            //gui_controller, // Cloning works for GTK, will it work for other GUI frameworks?
            deferred_result_render,
            search,
        }
    }
}
*/