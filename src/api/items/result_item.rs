use ::api::actions::base_action::BaseAction;

pub trait ResultItem {

    fn get_keyword(&self) -> Option<String> { None }
    fn get_name(&self) -> Option<String> { None }
    fn get_search_name(&self) -> Option<String> { self.get_name() }
    fn is_highlightable(&self) -> bool { true }
    // What is this?
    fn get_name_highlighted(&self) -> Option<String> { None }
    fn get_description(&self) -> Option<String> { None }
    // ? Return trait for image instead
    fn get_icon(&self) -> Option<String> { None }
    fn get_icon_size(&self) -> usize;
    fn include_in_results(&self) -> bool { true }
    fn selected_by_default(&self) -> bool { false }
    fn on_enter(&self) -> &Box<BaseAction>;
    fn on_alt_enter(&self) -> &Box<BaseAction>;
}