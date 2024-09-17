use std::fmt::format;

use fltk::{
    app,
    button::Button,
    frame::Frame,
    group::{Group, Pack},
    input::Input,
    menu::MenuButtonType,
    prelude::*,
    table::{Table, TableContext, TableRow},
    window::Window,
};

use fltk_table::{SmartTable, TableOpts};

use fltk::enums::FrameType;
use fltk::group::PackType;

mod init;

mod scene;
use scene::*;

mod models;

use crate::models::*;

fn main() {
    let app = app::App::default();
    let mut state = AppState::new();
    let mut main_win = Window::new(100, 100, 400, 400, "Aplicación");

    // Grupo del menú principal
    let mut group_menu = Group::new(0, 0, 400, 400, "");
    let mut input = Input::new(100, 50, 100, 30, "num. articulo");
    let mut ok_button = Button::new(250, 50, 60, 30, "ok");
    // Crear la smart table para mostrar los artículos

    let mut table = SmartTable::default()
        .with_size(300, 200)
        .with_pos(50, 150)
        .with_opts(TableOpts {
            rows: 6,
            cols: 3,
            editable: false,
            ..Default::default()
        });

    table.set_cell_value(0, 0, "Nombre");
    table.set_cell_value(0, 1, "Precio");
    table.set_cell_value(0, 2, "Cantidad");

    for (i, articulo) in state.articulos.iter().enumerate() {
        table.set_cell_value(i as i32 + 1, 0, &articulo.nombre);
        table.set_cell_value(i as i32 + 1, 1, &articulo.precio.to_string());
        table.set_cell_value(i as i32 + 1, 2, &articulo.cantidad.to_string());
    }

    group_menu.end();

    // Grupo de compra de artículo
    let mut group_compra = Group::new(0, 0, 400, 400, "");
    let mut compra_frame = Frame::new(50, 50, 300, 100, "Detalles del producto");
    let mut si_button = Button::new(50, 150, 80, 30, "Sí");
    let mut no_button = Button::new(150, 150, 80, 30, "No");
    group_compra.end();
    group_compra.hide();

    // Grupo de confirmación de compra
    let mut group_confirmacion = Group::new(0, 0, 400, 400, "");
    let mut confirmacion_frame = Frame::new(50, 50, 300, 100, "¿Desea confirmar la compra?");
    let mut comprar_button = Button::new(100, 150, 100, 30, "Comprar");
    group_confirmacion.end();
    group_confirmacion.hide();

    main_win.end();
    main_win.show();

    // Configuración de las callbacks
    ok_button.set_callback({
        let mut group_menu = group_menu.clone();
        let mut group_compra = group_compra.clone();
        move |_| {
            switch_to_compra_articulo(&mut group_menu, &mut group_compra);
        }
    });

    si_button.set_callback({
        let mut group_compra = group_compra.clone();
        let mut group_confirmacion = group_confirmacion.clone();
        move |_| {
            switch_to_confirmacion_compra(&mut group_compra, &mut group_confirmacion);
        }
    });

    no_button.set_callback({
        let mut group_compra = group_compra.clone();
        let mut group_menu = group_menu.clone();
        move |_| {
            group_compra.hide();
            group_menu.show();
        }
    });

    comprar_button.set_callback({
        let mut group_confirmacion = group_confirmacion.clone();
        let mut group_menu = group_menu.clone();
        move |_| {
            switch_to_menu(&mut group_confirmacion, &mut group_menu);
        }
    });

    app.run().unwrap();
}
