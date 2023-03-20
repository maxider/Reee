use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

#[get("/{count}/{wrap}")]
async fn hello(info: web::Path<(u64, u64)>) -> impl Responder {
    let info = info.into_inner();
    let count = info.0;
    let wrap = info.1;

    let mut body = String::from("R");

    for i in 1..count {
        if i % wrap == 0 && i > 0 {
            body.push('\n');
        }
        body.push('e');
    }
    HttpResponse::Ok().body(body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let address = ("127.0.0.1", 8080);
    println!("Binding server to address {:?}", address);
    HttpServer::new(|| {
        App::new()
            .service(hello)
    })
        .bind(address)?
        .run()
        .await
}