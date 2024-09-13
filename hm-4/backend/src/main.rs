use actix_cors::Cors;
use actix_web::{
    get, post,
    web::{self, Query},
    App, HttpResponse, HttpServer, Responder,
};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufReader;

#[derive(Deserialize)]
struct Registro {
    #[serde(rename = "Identificador")]
    identificador: i32,
    #[serde(rename = "Producto")]
    producto: String,
    #[serde(rename = "Marca")]
    marca: String,
    #[serde(rename = "Precio")]
    precio: i32,
    #[serde(rename = "Cantidad")]
    cantidad: i32,
}

#[derive(Serialize)]
struct Articulo {
    numero: i32,
    producto: String,
    precio: i32,
}

#[get("/articulos")]
async fn obtener_articulos() -> Result<impl Responder, actix_web::Error> {
    //TODO: Obtener el contenido de memoria compartida, en lugar de leer el archivo
    let file = File::open("./src/registro.json").expect("Error al abrir el archivo");
    let reader = BufReader::new(file);
    let registros: Vec<Registro> =
        serde_json::from_reader(reader).expect("Error al formatear el archivo");
    let articulos = registros
        .iter()
        .enumerate()
        .map(|(index, registro)| Articulo {
            numero: (index as i32) + 1,
            producto: registro.producto.clone(),
            precio: registro.precio,
        })
        .collect::<Vec<Articulo>>();
    Ok(web::Json(articulos))
}

#[get("/articulos/{id}")]
async fn obtener_articulo(Query(id): Query<i32>) -> impl Responder {
    Ok(format!("Articulo con id: {}", id))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(obtener_articulos)
            .service(obtener_articulo)
            .wrap(Cors::permissive())
    })
    .bind(("127.0.0.1", 1234))?
    .run()
    .await
}
