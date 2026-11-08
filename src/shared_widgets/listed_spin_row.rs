use relm4::{adw, adw::prelude::*, gtk};

pub fn listed_spin_row(row_title: &str) -> adw::SpinRow {
    let spin_row = adw::SpinRow::builder()
        .accessible_role(gtk::AccessibleRole::ListItem)
        .use_markup(false)
        .title(row_title)
        .build();
    spin_row
        .upcast_ref::<gtk::Widget>()
        .update_property(&[gtk::accessible::Property::Label(row_title)]);
    spin_row
}
