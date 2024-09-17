use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Articulo {
    pub id: i32,
    pub nombre: String,
    pub marca: String,
    pub precio: i32,
    pub cantidad: i32,
}

pub fn articulo_prueba() -> Vec<Articulo> {
    let mut articulos = vec![
        Articulo {
            id: 1,
            nombre: "Artículo 1".to_string(),
            marca: "Marca 1".to_string(),
            precio: 100,
            cantidad: 10,
        },
        Articulo {
            id: 2,
            nombre: "Artículo 2".to_string(),
            marca: "Marca 2".to_string(),
            precio: 200,
            cantidad: 20,
        },
        Articulo {
            id: 3,
            nombre: "Artículo 3".to_string(),
            marca: "Marca 3".to_string(),
            precio: 300,
            cantidad: 30,
        },
        Articulo {
            id: 4,
            nombre: "Artículo 4".to_string(),
            marca: "Marca 4".to_string(),
            precio: 400,
            cantidad: 40,
        },
        Articulo {
            id: 5,
            nombre: "Artículo 5".to_string(),
            marca: "Marca 5".to_string(),
            precio: 500,
            cantidad: 50,
        },
    ];
    articulos
}

pub struct AppState {
    pub articulos: Vec<Articulo>,
    pub articulo_seleccionado: Option<i32>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            articulos: articulo_prueba(),
            articulo_seleccionado: None,
        }
    }
}
