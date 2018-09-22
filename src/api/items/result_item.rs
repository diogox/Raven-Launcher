use std::fmt::Debug;
use serde_traitobject::{
    Serialize,
    Deserialize,
};

use ::api::actions::base_action::BaseAction;

pub trait ResultItem: Debug + Serialize + Deserialize {

    fn keyword(&self) -> Option<String> { None }
    fn name(&self) -> Option<String> { None }
    fn search_name(&self) -> Option<String> { self.name() }
    fn is_highlightable(&self) -> bool { true }
    // What is this?
    fn name_highlighted(&self) -> Option<String> { None }
    fn description(&self) -> Option<String> { None }
    // ? Return trait for image instead
    fn icon(&self) -> Option<String> { None }
    fn icon_size(&self) -> usize;
    fn include_in_results(&self) -> bool { true }
    fn selected_by_default(&self) -> bool { false }
    fn on_enter(&self) -> &Box<BaseAction>;
    fn on_alt_enter(&self) -> &Box<BaseAction>;
}