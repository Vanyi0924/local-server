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
        "png" => ContentType::png(),
        "svg" => ContentType(mime::IMAGE_SVG),
        "json" => ContentType::json(),
        "js" => ContentType(mime::APPLICATION_JAVASCRIPT),
        "css" => ContentType(mime::TEXT_CSS),
        "woff2" => ContentType(mime::FONT_WOFF2),
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
