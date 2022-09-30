use actix_web::http::header::ContentType;
use mime;

pub fn get_mime_type(ext: &str) -> ContentType {
    match ext {
        "html" => ContentType::html(),
        "webp" | "jpg" | "jpeg" => ContentType(mime::IMAGE_JPEG),
        "json" => ContentType::json(),
        _ => ContentType(mime::STAR_STAR),
    }
}
