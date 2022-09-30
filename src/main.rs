use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use local_server::get_mime_type;
use std::env;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

const PORT: u16 = 9211;

async fn all(req: HttpRequest) -> impl Responder {
    let current_dir = env::current_dir().unwrap();
    let file_path = Path::new(current_dir.as_path())
        // .parent()
        // .unwrap()
        .join(&String::from(req.path())[1..]);
    println!("路径:{:?}", file_path);
    let file_result = File::open(file_path);
    match file_result {
        Ok(file) => {
            let mut reader = BufReader::new(file);
            let mut buffer = Vec::new();
            reader.read_to_end(&mut buffer).unwrap();
            let extension = Path::new(req.path())
                .extension()
                .unwrap_or_default()
                .to_str()
                .unwrap_or_default();
            HttpResponse::Ok()
                .content_type(get_mime_type(extension))
                .body(buffer)
        }
        Err(_) => HttpResponse::NotFound().body("404 not found"),
    }
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    println!(
        "your local dev server is runing at: http://127.0.0.1:{}",
        PORT
    );

    HttpServer::new(|| App::new().default_service(web::route().to(all)))
        .bind(("127.0.0.1", PORT))?
        .run()
        .await
}
