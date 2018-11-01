use gtk::{
    self,
    WidgetExt,
    StyleContextExt,
    OrientableExt,
    ScrolledWindowExt,
    AdjustmentExt,
    ContainerExt,
};

use relm::{
    Relm,
    Widget,
};
use relm_attributes::widget;

use super::super::result_item::msg::ResultItemMsg;
use super::model::NavigationModel;
use super::nav_msg::NavMsg;

type Index = usize;

#[widget]
impl Widget for ResultsNavigation {

    fn init_view(&mut self) {

        // Set styles
        self.results_box.get_style_context()
            .unwrap()
            .add_class("result-box");
    }

    fn model(relm: &Relm<Self>, _: ()) -> NavigationModel {

        let scrolled_adjustment = gtk::Adjustment::new(
            0.0,
            0.0,
            504.0,
            56.0,
            56.0,
            504.0
        );

        let relm_clone = relm.clone();
        scrolled_adjustment.connect_value_changed(move |_| {
            relm_clone.stream().emit(NavMsg::_AdjustScroll);
        });

        NavigationModel {
            selected_index: None,
            results: Vec::new(),
            scrolled_adjustment,
            scroll_upper_bound_index: 0,
            relm: relm.clone(),
        }
    }

    fn calculate_height(&self) -> i32 {

        let n = self.model.results.len() as i32;

        if n < 9 {
            56*n
        } else {
            56*9
        }
    }

    fn is_item_in_scrollable(&self, index: Index) -> bool {
        let upper_bound = self.model.scroll_upper_bound_index;
        let lower_bound = upper_bound - 9;
        println!("UPPER: {}", upper_bound);
        println!("LOWER: {}", lower_bound);
        println!("NEW_INDEX: {}", index);
        if index >= lower_bound && index < upper_bound {
            return true;
        }
        println!("NOT IN SCROLLABLE!");
        false
    }

    fn update(&mut self, event: NavMsg) {
        match event {

            NavMsg::GoUp => { // TODO: Refactor this unreadable mess
                use super::super::result_item::msg::ResultItemMsg;

                let n_results = self.model.results.len();
                let index = self.model.selected_index.unwrap_or(0);

                let mut new_index = index + 1;

                // Check if out of bounds
                if new_index >= n_results {

                    // Go to the last result
                    new_index = 0;
                    self.model.scroll_upper_bound_index = 9;

                    // Set window scroll to the bottom
                    let offset = (0..self.model.results.len()).rev().take(9).collect::<Vec<usize>>()[0];
                    self.model.scrolled_adjustment.set_value((offset as f64) * 56.0);
                } else {

                    // Check if out of scrolled window viewable area
                    if !self.is_item_in_scrollable(new_index) {
                        let current_adj = self.model.scrolled_adjustment.get_value();
                        let adj = current_adj - 56.0;
                        self.model.scrolled_adjustment.set_value(adj);

                        if self.model.scroll_upper_bound_index == self.model.results.len() {
                            self.model.scroll_upper_bound_index = 9;
                        } else {
                            self.model.scroll_upper_bound_index += 1;
                        }
                    }
                    println!("UPPER_BOUND: {}", self.model.scroll_upper_bound_index);
                }

                // Deselect previous
                self.model.results[index].emit(ResultItemMsg::Deselect);

                // Select desired item
                self.model.results[new_index].emit(ResultItemMsg::Select);

                // Save new index
                self.model.selected_index = Some(new_index);

                println!("{}", self.model.scrolled_adjustment.get_value());
            },
            NavMsg::GoDown => { // TODO: Refactor this unreadable mess
                use super::super::result_item::msg::ResultItemMsg;

                let n_results = self.model.results.len();
                let index = self.model.selected_index.unwrap_or(n_results - 1);

                // Check if out of bounds
                let mut new_index = index;
                if index == 0 {

                    // Go to the last result
                    new_index = n_results - 1;
                    self.model.scroll_upper_bound_index = new_index + 1;

                    // Set window scroll to the top
                    self.model.scrolled_adjustment.set_value(0.0);
                } else {
                    new_index = index - 1;

                    // Check if out of scrolled window viewable area
                    if !self.is_item_in_scrollable(new_index) {
                        let current_adj = self.model.scrolled_adjustment.get_value();
                        let adj = current_adj + 56.0;
                        self.model.scrolled_adjustment.set_value(adj);
                        self.model.scroll_upper_bound_index -= 1;
                    }
                }

                // Deselect previous
                self.model.results[index].emit(ResultItemMsg::Deselect);

                // Select desired item
                self.model.results[new_index].emit(ResultItemMsg::Select);

                // Save new index
                self.model.selected_index = Some(new_index);

                println!("{}", self.model.scrolled_adjustment.get_value());
            },
            NavMsg::AddResult(result) => {
                use gtk::BoxExt;
                use super::super::result_item::msg::ResultItemMsg;

                result.emit(ResultItemMsg::_SetParentRelm(self.model.relm.clone()));
                result.emit(ResultItemMsg::_SetItemIndex( (self.model.results.len() as u32) ));

                self.results_box.pack_end(
                    result.widget(),
                    false,
                    false,
                    0,
                );

                // Add to model
                self.model.results.push(result);

                // Resize
                self.update(NavMsg::Resize);

                // Update offset (to keep the biggest index as the upper scroll bound)
                self.model.scroll_upper_bound_index += 1;
            },
            NavMsg::Clear => {
                use gtk::{
                    BoxExt,
                    ContainerExt,
                };

                let children = self.results_box.get_children();
                children.iter()
                    .for_each(|child|
                        self.results_box.remove(child)
                    );

                // Reset selected index
                self.model.selected_index = None;

                // Reset bound
                self.model.scroll_upper_bound_index = 0;
            },
            NavMsg::Resize => {
                let n = self.model.results.len() as i32;

                // Set height
                let mut height = 0;
                if n < 9 {
                    height = 56*n;
                } else {
                    height = 56*9;
                }

                self.scrolled_window.set_property_height_request(height);

                // Set margin
                if n==0 {
                    self.scrolled_window.set_margin_top(0);
                    self.scrolled_window.set_margin_bottom(0);
                } else {
                    self.scrolled_window.set_margin_top(3);
                    self.scrolled_window.set_margin_bottom(10);
                }
            },
            NavMsg::SelectItem(new_index) => {

                let new_index = new_index as usize;

                let index_opt = self.model.selected_index;
                if let Some(index) = index_opt {

                    // Deselect previous
                    self.model.results[index].emit(ResultItemMsg::Deselect);
                }

                // Select desired item
                self.model.results[new_index].emit(ResultItemMsg::Select);

                // Save new index
                self.model.selected_index = Some(new_index);
            },
            NavMsg::_AdjustScroll => {
                let v_adj = self.scrolled_window.get_vadjustment()
                    .unwrap()
                    .get_value();

                let n = self.model.results.len() - 1;
                let upper_bound_index = n - ((v_adj / 56.0) as usize);
                self.model.scroll_upper_bound_index = upper_bound_index;
                println!("V_ADJ: {}", v_adj);
            }
        }
    }

    view! {

        #[name = "scrolled_window"]
        gtk::ScrolledWindow {
            property_hscrollbar_policy: gtk::PolicyType::Never,
            visible: true,
            overlay_scrolling: true,
            kinetic_scrolling: true,
            property_height_request: 0,
            vadjustment: &self.model.scrolled_adjustment,
            margin_top: 0,
            margin_bottom: 0,

            // Packing
            hexpand: false,
            valign: gtk::Align::Fill,

            #[container]
            #[name = "results_box"]
            gtk::Box {
                name: "result-box",
                visible: true,
                can_focus: false,
                orientation: gtk::Orientation::Vertical,

                // Packing
                hexpand: false,
                valign: gtk::Align::Fill,
            }
        }
    }

}