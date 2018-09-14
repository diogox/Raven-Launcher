use ::api::actions::base_action::BaseAction;
use ::api::actions::DoNothingAction;

#[derive(Serialize, Deserialize, Debug)]
pub struct ExtensionResultItem {
    keyword: Option<String>,
    name: Option<String>,
    description: Option<String>,
    icon: Option<String>, // ? Change with image trait ?
    icon_size: usize,
    include_in_results: bool,
    selected_by_default: bool,
    highlightable: bool,
    
    #[serde(with = "serde_traitobject")]
    on_enter: Box<dyn BaseAction>,
    #[serde(with = "serde_traitobject")]
    on_alt_enter: Box<dyn BaseAction>,
}

impl ExtensionResultItem {
    
    pub fn new() -> Self { 
        ExtensionResultItem {
            keyword: None,
            name: None,
            description: None,
            icon: None,
            icon_size: 40,
            include_in_results: true,
            selected_by_default: false,
            highlightable: true,
            on_enter: Box::new(DoNothingAction::new()),
            on_alt_enter: Box::new(DoNothingAction::new()),
        }
    }

    pub fn new_small() -> Self { 
        ExtensionResultItem {
            keyword: None,
            name: None,
            description: None,
            icon: None,
            icon_size: 25,
            include_in_results: true,
            selected_by_default: false,
            highlightable: true,
            on_enter: Box::new(DoNothingAction::new()),
            on_alt_enter: Box::new(DoNothingAction::new()),
        }
    }

    pub fn keyword(&mut self, new_keyword: String) {
        self.keyword = Some(new_keyword);
    }

    pub fn name(&mut self, new_name: String) {
        self.name = Some(new_name);
    }

    pub fn description(&mut self, new_description: String) {
        self.description = Some(new_description);
    }

    pub fn icon(&mut self, new_icon: String) {
        self.icon = Some(new_icon);
    }

    pub fn selected_by_default(&mut self, selected_by_default: bool) {
        self.selected_by_default = selected_by_default;
    }

    pub fn highlightable(&mut self, highlightable: bool) {
        self.highlightable = highlightable;
    }

    pub fn on_enter(&mut self, action: impl BaseAction + 'static) {
        self.on_enter = Box::new(action);
    }

    pub fn on_alt_enter(&mut self, action: impl BaseAction + 'static) {
        self.on_alt_enter = Box::new(action);
    }
}

use super::result_item::ResultItem;
impl ResultItem for ExtensionResultItem {
    fn get_keyword(&self) -> Option<String> { self.keyword.clone() }
    fn get_name(&self) -> Option<String> { self.name.clone() }
    fn is_highlightable(&self) -> bool { self.highlightable }
    // What is this?
    fn get_name_highlighted(&self) -> Option<String> { None } // TODO
    fn get_description(&self) -> Option<String> { self.description.clone() }
    fn get_icon(&self) -> Option<String> { self.icon.clone() }
    fn get_icon_size(&self) -> usize { self.icon_size }
    fn include_in_results(&self) -> bool { self.include_in_results }
    fn selected_by_default(&self) -> bool { self.selected_by_default }
    fn on_enter(&self) -> &Box<BaseAction> { &self.on_enter }
    fn on_alt_enter(&self) -> &Box<BaseAction> { &self.on_alt_enter }
}

