use gtk;

use relm::{
    Relm,
    Widget,
};
use relm_attributes::widget;

use gtk::{
    LabelExt,
    WidgetExt,
    ImageExt,
    FixedExt,
    OrientableExt,
    StyleContextExt,
    FixedExtManual,
};

use super::msg::ResultItemMsg;
use super::super::navigation::navigation_item::NavigationItem;
use super::model::ResultItemModel;
use super::super::utils::FixedUtils;

#[widget]
impl Widget for ResultItem {

    fn init_view(&mut self) {

        self.item_frame.get_style_context()
            .unwrap()
            .add_class("item-frame");

        self.item_box.get_style_context()
            .unwrap()
            .add_class("item-box");

        self.item_container.get_style_context()
            .unwrap()
            .add_class("item-container");

        self.item_name.get_style_context()
            .unwrap()
            .add_class("item-name");

        self.item_name.get_style_context()
            .unwrap()
            .add_class("item-text");

        self.item_descr.get_style_context()
            .unwrap()
            .add_class("item-descr");

        self.item_descr.get_style_context()
            .unwrap()
            .add_class("item-text");

        self.item_shortcut.get_style_context()
            .unwrap()
            .add_class("item-shortcut");

        self.item_shortcut.get_style_context()
            .unwrap()
            .add_class("item-text");

        // Set position of items in 'Fixed'-type
        self.item_fixed.set_child_y(&self.item_name, 5);
        self.item_fixed.set_child_y(&self.item_descr, 28);
    }

    fn model(relm: &Relm<Self>, _: ()) -> ResultItemModel {

        ResultItemModel {
            title: "".to_string(),
            relm: None,
            index: 0,
        }
    }

    fn update(&mut self, event: ResultItemMsg ) {
        use super::super::super::api::items::result_item::ResultItem;

        match event {
            ResultItemMsg::SetInfo(item) => {
                self.item_name.set_text(&item.name().unwrap());
                self.item_descr.set_text(&item.description().unwrap());
            },
            ResultItemMsg::Select => {
                self.item_frame.select();
            },
            ResultItemMsg::Deselect => {
                self.item_frame.deselect();
            },
            ResultItemMsg::Hover => {
                use super::super::navigation::nav_msg::NavMsg;
                self.model.relm.as_ref().unwrap().stream().emit(NavMsg::SelectItem(self.model.index));
            },
            ResultItemMsg::_SetParentRelm(relm) => {
                self.model.relm = Some(relm);
            },
            ResultItemMsg::_SetItemIndex(index) => {
                self.model.index = index;
            },
        }
    }

    view! {

        #[name = "item_frame"]
        gtk::Box {
            visible: true,
            can_focus: false,

            #[name = "item_box"]
            gtk::EventBox {
                visible: true,
                can_focus: false,
                enter_notify_event(_, _) => (ResultItemMsg::Hover, gtk::Inhibit(false)),

                #[name = "item_container"]
                gtk::Box {
                    visible: true,
                    can_focus: false,
                    margin_top: 3,
                    margin_bottom: 3,
                    margin_left: 20,
                    margin_right: 20,

                    gtk::Image {
                        property_height_request: 50,
                        property_width_request: 50,
                        visible: true,
                        can_focus: false,
                        property_stock: Some("gtk-missing-image"),

                        // Packing
                        hexpand: false,
                        valign: gtk::Align::Fill,
                    },

                    #[name = "item_fixed"]
                    gtk::Fixed {
                        visible: true,
                        can_focus: false,
                        margin_left: 5,
                        vexpand: false,
                        hexpand: false,

                        #[name = "item_name"]
                        gtk::Label {
                            label: "IntelliJ IDEA Ultimate Edition",
                            property_width_request: 410,
                            visible: true,
                            can_focus: false,
                            //ellipsize: pango::EllipsizeMode::Middle,
                            max_width_chars: 1,
                            xalign: 0.0,
                        },

                        #[name = "item_descr"]
                        gtk::Label {
                            label: "The Drive to Develop",
                            property_width_request: 410,
                            visible: true,
                            can_focus: false,
                            //ellipsize: pango::EllipsizeMode::Middle,
                            max_width_chars: 1,
                            xalign: 0.0,

                            // Packing
                            hexpand: false,
                            valign: gtk::Align::Fill,
                        },
                    },

                    #[name = "item_shortcut"]
                    gtk::Label {
                        property_width_request: 44,
                        visible: true,
                        can_focus: false,
                        margin_left: 15,
                        label: "Alt+1",
                        justify: gtk::Justification::Right,
                    }
                }
            }
        }
    }

}