use fltk::{group::Group, prelude::*};

pub fn switch_to_compra_articulo(group_menu: &mut Group, group_compra: &mut Group) {
    group_menu.hide();
    group_compra.show();
}

pub fn switch_to_confirmacion_compra(group_compra: &mut Group, group_confirmacion: &mut Group) {
    group_compra.hide();
    group_confirmacion.show();
}

pub fn switch_to_menu(group_confirmacion: &mut Group, group_menu: &mut Group) {
    group_confirmacion.hide();
    group_menu.show();
}
