use relm::Relm;
use super::super::navigation::results_navigation::ResultsNavigation;
use super::super::super::api::items::ExtensionResultItem;

#[derive(Msg)]
pub enum ResultItemMsg {
    SetInfo(ExtensionResultItem),
    Select,
    Deselect,
    Hover,
    _SetParentRelm(Relm<ResultsNavigation>),
    _SetItemIndex(u32),
}