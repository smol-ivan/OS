use crate::shared_memory::{get_shared_memory, Registro};
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Serialize, Deserialize, Debug)]
pub struct Articulo {
    pub id: i32,
    pub nombre: String,
    pub marca: String,
    pub precio: i32,
    pub cantidad: i32,
}

#[derive(Clone, Copy)]
pub struct AppState {
    pub articulos: *mut Registro,
    pub articulo_seleccionado: i32,
}

impl AppState {
    pub fn new() -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            articulos: get_shared_memory(),
            articulo_seleccionado: 0,
        }))
    }
}
