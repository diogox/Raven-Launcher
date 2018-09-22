use super::base_search_mode::BaseSearchMode;
use super::extension_search_mode::ExtensionSearchMode;

pub struct Search  {
    search_modes: Vec< Box<dyn BaseSearchMode> >,
}

impl Search {

    pub fn new() -> Self {

        // Initialize search modes
        let extension_search_mode = ExtensionSearchMode::new();
        // TODO: Add other search modes

        let mut search_modes = Vec::new();
        search_modes.push(Box::new(extension_search_mode) as Box<BaseSearchMode>);

        Search {
            search_modes,
        }
    }

    /// Iterate over all search modes and run first enabled.
    // TODO: Make 'AppSearchMode is always enabled' a true statement
    pub fn on_query_change(&mut self, query: &str) {
        let query = String::from(query);

        // 
        self.search_modes.iter_mut().for_each(|mode| {
            mode.on_query(&query);
        });

        // Run active search mode
        if let Some(mode) = self.choose_search_mode(&query) {
            mode.handle_query(&query)
                .run()
                .unwrap(); // TODO: Deal with Result
        }
    }

    // ! Should, likely, take GTK widget so it can change it
    pub fn on_key_press_event(&mut self, query: &str) {
        if let Some(mode) = self.choose_search_mode(query) {
            mode.handle_key_press_event(query).run()
                .unwrap(); // TODO: Deal with Result
        }
    }
}

impl Search {

    fn choose_search_mode(&mut self, query: &str) -> Option< Box<&mut BaseSearchMode> > {
        let query = String::from(query);

        for mode in self.search_modes.iter_mut() {
            if mode.is_enabled(&query) {
                return Some(Box::new(&mut *mode.as_mut()));
            }
        }

        None   
    }
}
