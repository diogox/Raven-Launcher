type PLACEHOLDER = String;

// ! Might have to change the 'query' types from &str to a Query struct.

use ::api::actions::{
    base_action::BaseAction,
    do_nothing::DoNothingAction,
};
use ::api::items::result_item::ResultItem;

pub trait BaseSearchMode {

    /// Return True if mode should be enabled for a query.
    fn is_enabled(&self, _query: &str) -> bool { false }

    /// Triggered when user changes a search query.
    fn on_query(&mut self, query: &str);
 
    // TODO: Make this take a 'GuiItem' trait object
    // or closure that performs an action on the GUI
    fn handle_key_press_event(&self, _query: &str)
        -> Box<BaseAction> { Box::new(DoNothingAction::new()) }

    fn handle_query(&self, _query: &str) -> Box<dyn BaseAction> { 
        Box::new(DoNothingAction::new()) as Box<BaseAction> 
    }

    /// Returns a list of default result items 
    /// that should be displayed if no results found.
    // ???
    fn get_default_items(&self) -> Vec<Box<ResultItem>> { Vec::new() }

    /// Returns a list of result items that
    /// can be looked up by name or keyword.
    fn get_searchable_items(&self) -> Vec<Box<ResultItem>> { Vec::new() }
}