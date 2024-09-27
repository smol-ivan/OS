use crate::shared_memory::get_shared_memory;
use std::cell::RefCell;
use std::rc::Rc;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Registro {
    pub id: i32,
    pub producto: [u8; 50],
    pub marca: [u8; 50],
    pub precio: i32,
    pub cantidad: i32,
}

impl Registro {
    pub fn new(id: i32, producto: [u8; 50], marca: [u8; 50], precio: i32, cantidad: i32) -> Self {
        let registro = Registro {
            id,
            producto,
            marca,
            precio,
            cantidad,
        };
        registro
    }
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
