use std::ffi::OsString;
use std::fs::File;
use std::io::Read;
use std::path::Path;

pub fn get_mime_type(ext: &str) -> &'static str {
    match ext {
        "html" => "text/html; charset=utf-8",
        "webp" => "image/webp",
        _ => "text/plain; charset=UTF-8",
    }
}

pub fn read_the_file(path: &str) -> (&[u8], OsString) {
    let file = File::open(path);
    // TODO
    match file {
        Ok(mut data) => {
            let mut contents = [];
            // data.read_to_string(&mut contents).unwrap();
            data.read(&mut contents).unwrap();
            let extension = Path::new(path).extension().unwrap().to_owned();
            (&[1], extension)
        }
        Err(err) => (
            // String::from("[read_the_file Error] 404 not found file").as_bytes(),
            &[1],
            OsString::from(""),
        ),
    }
}
