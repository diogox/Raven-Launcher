use relm::Component;

use super::super::super::super::api::items::ExtensionResultItem;
use super::super::super::result_item::ResultItem;

#[derive(Msg)]
pub enum Msg {
    HandleQuery(String),
    ShowResults(Vec<ExtensionResultItem>),
    KeyPress(u32),
    NavGoUp,
    NavGoDown,
    NavAddResult(ExtensionResultItem),
    NavClearResults,
    Toggle,
    Hide,
    Quit,
}