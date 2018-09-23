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