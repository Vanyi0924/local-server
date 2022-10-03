use actix_web::http::header::ContentType;
use mime;
use std::net::TcpListener;

/// 获取 mime type
///
/// 默认为 \*/\*
pub fn get_mime_type(ext: &str) -> ContentType {
    match ext {
        "html" => ContentType::html(),
        "webp" | "jpg" | "jpeg" => ContentType(mime::IMAGE_JPEG),
        "json" => ContentType::json(),
        "md" => ContentType(mime::TEXT_PLAIN_UTF_8),
        _ => ContentType(mime::STAR_STAR),
    }
}

/// 获取可用端口
///
/// 默认为 9211
pub fn get_unused_port(port: u16) -> u16 {
    let port_string = port.to_string();
    let localhost = String::from("127.0.0.1:");
    match TcpListener::bind((localhost + port_string.as_str()).as_str()) {
        Ok(listener) => port,
        Err(err) => get_unused_port(port + 1),
    }
}
