use actix_web::HttpResponse;
use serde_json::json;
use crate::model::producto::Producto;

pub fn render_productos(productos: Vec<Producto>) -> HttpResponse {
    let json_productos = json!(productos);
    HttpResponse::Ok().json(json_productos)
}
