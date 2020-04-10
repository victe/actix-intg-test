use actix_web::{get, web, App, HttpServer, Responder};

#[get("/{id}/{name}")]
async fn index(info: web::Path<(u32, String)>) -> impl Responder {
    format!("Hello {}! id:{}", info.1, info.0)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let address = std::env::args().nth(1).expect("Need an address and port");
    println!("{}", address);
    HttpServer::new(|| App::new().service(index))
        .workers(1)
        .bind(address)?
        .run()
        .await
}
