use relm4::gtk::{self, gio, prelude::*};

const MODEL_BUTTON_TYPE_NAME: &str = "GtkModelButton";
const MODEL_BUTTON_TEXT_PROPERTY: &str = "text";

pub fn text_without_mnemonic(item_text: &str) -> String {
    let mut shown_text = String::with_capacity(item_text.len());
    let mut text_characters = item_text.chars().peekable();
    while let Some(text_character) = text_characters.next() {
        if text_character != '_' {
            shown_text.push(text_character);
        } else if text_characters.next_if_eq(&'_').is_some() {
            shown_text.push('_');
        }
    }
    shown_text
}

fn name_model_buttons_below(parent_widget: &gtk::Widget) {
    let mut next_child = parent_widget.first_child();
    while let Some(child_widget) = next_child {
        if child_widget.type_().name() == MODEL_BUTTON_TYPE_NAME {
            let item_text = child_widget
                .property::<Option<String>>(MODEL_BUTTON_TEXT_PROPERTY)
                .unwrap_or_default();
            child_widget.reset_relation(gtk::AccessibleRelation::LabelledBy);
            child_widget.update_property(&[gtk::accessible::Property::Label(
                &text_without_mnemonic(&item_text),
            )]);
        }
        name_model_buttons_below(&child_widget);
        next_child = child_widget.next_sibling();
    }
}

pub trait NamedMenuItems {
    fn set_named_menu_model(&self, menu_model: &impl IsA<gio::MenuModel>);
}

impl NamedMenuItems for gtk::MenuButton {
    fn set_named_menu_model(&self, menu_model: &impl IsA<gio::MenuModel>) {
        self.set_menu_model(Some(menu_model));
        if let Some(menu_popover) = self.popover() {
            name_model_buttons_below(menu_popover.upcast_ref());
        }
    }
}

#[cfg(test)]
mod named_menu_items_tests {
    use super::*;

    #[test]
    fn mnemonic_underscores_are_dropped_and_doubled_ones_kept() {
        assert_eq!(text_without_mnemonic("_Copy Path"), "Copy Path");
        assert_eq!(
            text_without_mnemonic("Ignore This Detection for Me…"),
            "Ignore This Detection for Me…"
        );
        assert_eq!(text_without_mnemonic("snake__case"), "snake_case");
    }
}
