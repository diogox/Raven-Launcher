use relm::Component;
use super::super::result_item::ResultItem;

#[derive(Msg)]
pub enum NavMsg {
    GoUp,
    GoDown,
    AddResult(Component<ResultItem>),
    Clear,
    Resize,
    SelectItem(u32),
    _AdjustScroll,
}