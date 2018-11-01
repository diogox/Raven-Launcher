use gtk;
use relm::{
    Relm,
    Component,
};
use super::results_navigation::ResultsNavigation;
use super::super::result_item::ResultItem;

pub struct NavigationModel {

    pub selected_index: Option<usize>,
    pub results: Vec< Component<ResultItem> >,
    pub scrolled_adjustment: gtk::Adjustment,
    pub scroll_upper_bound_index: usize,
    pub relm: Relm<ResultsNavigation>
}