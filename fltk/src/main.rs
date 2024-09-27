use fltk::{
    app, button::Button, frame::Frame, group::Group, input::Input, prelude::*, window::Window,
};

use fltk_table::{SmartTable, TableOpts};

use Tarea04::shared_memory::{
    byte_array_to_string, simular_compra, sm_bloq_articulo, sm_desb_articulo,
};

mod scene;
use scene::*;

use Tarea04::models::*;

fn main() {
    // Antes de iniciar la aplicación, se debe cargar la memoria compartida

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
            cols: 4,
            editable: false,
            ..Default::default()
        });

    table.set_cell_value(0, 0, "Num");
    table.set_cell_value(0, 1, "Nombre");
    table.set_cell_value(0, 2, "Precio");
    table.set_cell_value(0, 3, "Cantidad");

    for i in 0..5 {
        unsafe {
            let registro = &*state.borrow().articulos.add(i);
            let producto = byte_array_to_string(&registro.producto);
            let precio = &registro.precio;
            let cantidad = &registro.cantidad;

            table.set_cell_value(i as i32 + 1, 0, (i + 1).to_string().as_str());
            table.set_cell_value(i as i32 + 1, 1, &producto);
            table.set_cell_value(i as i32 + 1, 2, &precio.to_string());
            table.set_cell_value(i as i32 + 1, 3, &cantidad.to_string());
        }
    }

    group_menu.end();

    // Grupo de compra de artículo
    let mut group_compra = Group::new(0, 0, 400, 400, "");
    let mut _compra_frame = Frame::new(50, 50, 300, 100, "Detalles del producto");
    let mut detalles_articulo = Frame::new(50, 150, 300, 50, "");
    let mut si_button = Button::new(120, 260, 80, 30, "Sí");
    let mut no_button = Button::new(200, 260, 80, 30, "No");
    group_compra.end();
    group_compra.hide();

    // Grupo de confirmación de compra
    let mut group_confirmacion = Group::new(0, 0, 400, 400, "");
    let mut _confirmacion_frame = Frame::new(50, 50, 300, 100, "¿Desea confirmar la compra?");
    let mut table_confir = SmartTable::default()
        .with_size(300, 200)
        .with_pos(50, 150)
        .with_opts(TableOpts {
            rows: 2,
            cols: 3,
            editable: false,
            ..Default::default()
        });
    table_confir.set_cell_value(0, 0, "Num");
    table_confir.set_cell_value(0, 1, "Producto");
    table_confir.set_cell_value(0, 2, "Marca");
    let mut comprar_button = Button::new(100, 280, 100, 30, "Comprar");
    group_confirmacion.end();
    group_confirmacion.hide();

    main_win.end();
    main_win.show();

    // Configuración de las callbacks
    ok_button.set_callback({
        let state = state.clone();
        let mut group_menu = group_menu.clone();
        let mut group_compra = group_compra.clone();
        let mut detalles_articulo = detalles_articulo.clone();

        unsafe {
            move |_| {
                let mut estado = state.borrow_mut();
                let num_articulo = input.value().parse::<usize>().unwrap();
                estado.articulo_seleccionado = num_articulo as i32;
                sm_bloq_articulo(estado.articulos, estado.articulo_seleccionado - 1);
                let articulo = &*estado
                    .articulos
                    .add((estado.articulo_seleccionado - 1) as usize);
                let identificador = &articulo.id;
                let producto = byte_array_to_string(&articulo.producto);
                let marca = byte_array_to_string(&articulo.marca);
                let precio = &articulo.precio;
                let cantidad = &articulo.cantidad;
                // Mostrar los detalles del producto
                let detalles = format!(
                    "ID: {}\nProducto: {}\nMarca: {}\nPrecio: {}\nCantidad: {}",
                    identificador, producto, marca, precio, cantidad
                );
                detalles_articulo.set_label(&detalles);

                switch_to_compra_articulo(&mut group_menu, &mut group_compra);
            }
        }
    });

    si_button.set_callback({
        let mut group_compra = group_compra.clone();
        let mut group_confirmacion = group_confirmacion.clone();
        let mut table_confir = table_confir.clone();
        let state = state.clone();
        unsafe {
            move |_| {
                let estado = state.borrow_mut();
                let articulo = &*estado
                    .articulos
                    .add((estado.articulo_seleccionado - 1) as usize);
                let producto = byte_array_to_string(&articulo.producto);
                let marca = byte_array_to_string(&articulo.marca);
                table_confir.set_cell_value(1, 0, &estado.articulo_seleccionado.to_string());
                table_confir.set_cell_value(1, 1, &producto);
                table_confir.set_cell_value(1, 2, &marca);
                switch_to_confirmacion_compra(&mut group_compra, &mut group_confirmacion);
            }
        }
    });

    no_button.set_callback({
        let mut group_compra = group_compra.clone();
        let mut group_menu = group_menu.clone();
        let state = state.clone();
        move |_| {
            let estado = state.borrow_mut();
            unsafe {
                sm_desb_articulo(estado.articulos, estado.articulo_seleccionado - 1);
            }
            group_compra.hide();
            group_menu.show();
        }
    });

    comprar_button.set_callback({
        let mut group_confirmacion = group_confirmacion.clone();
        let mut group_menu = group_menu.clone();
        let state = state.clone();
        let mut table = table.clone();
        move |_| {
            let estado = state.borrow_mut();
            simular_compra(estado.articulos, estado.articulo_seleccionado - 1);
            unsafe {
                sm_desb_articulo(estado.articulos, estado.articulo_seleccionado - 1);
            }

            // Actualizar la tabla usando hilos

            for i in 0..5 {
                unsafe {
                    let registro = &*estado.articulos.add(i);
                    let producto = byte_array_to_string(&registro.producto);
                    let precio = &registro.precio;
                    let cantidad = &registro.cantidad;

                    table.set_cell_value(i as i32 + 1, 0, (i + 1).to_string().as_str());
                    table.set_cell_value(i as i32 + 1, 1, &producto);
                    table.set_cell_value(i as i32 + 1, 2, &precio.to_string());
                    table.set_cell_value(i as i32 + 1, 3, &cantidad.to_string());
                }
            }

            switch_to_menu(&mut group_confirmacion, &mut group_menu);
        }
    });

    app.run().unwrap();
}
