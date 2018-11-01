use relm::Relm;
use super::super::navigation::results_navigation::ResultsNavigation;

pub struct ResultItemModel {
    pub title: String,
    pub relm: Option< Relm<ResultsNavigation> >,
    pub index: u32,
}