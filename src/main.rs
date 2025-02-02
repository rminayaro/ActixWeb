use actix_web::{web, App, HttpServer};
use viewmodel::producto_viewmodel::{get_productos};
use aumento_precios::aplicar_aumento_precios;
use descuento_precios::aplicar_descuento_precios;
use env_logger;

mod model;
mod view;
mod viewmodel;
mod producto_count;
mod aumento_precios;
mod descuento_precios;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init(); // Inicializa el logger

    let client = reqwest::Client::new();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(client.clone()))
            .route("/productos", web::get().to(get_productos))
            .route("/aumentarprecio", web::post().to(aplicar_aumento_precios)) // Nueva ruta para aumentar precio
            .route("/reducirprecio", web::post().to(aplicar_descuento_precios)) // Nueva ruta para reducir precio
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
                                        