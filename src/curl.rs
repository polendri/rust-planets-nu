extern crate http;
extern crate url;

use self::http::client::request::RequestWriter;
use self::http::client::response::ResponseReader;
use self::http::method::Get;
use self::url::Url;
use std::io;

pub fn http_get(url: &str) -> io::IoResult<ResponseReader> {
    let url_obj = Url::parse(url).unwrap();
    let request: RequestWriter = try!(RequestWriter::new(Get, url_obj));
    request.read_response().map_err(|(_request, error)| error)
}

pub fn http_post<'a>(url: &str, data: &[u8]) -> io::IoResult<Box<Reader + 'a>> {
    let url_obj = Url::parse(url).unwrap();
    let mut request: RequestWriter = try!(RequestWriter::new(Get, url_obj));
    request.headers.content_length = Some(data.len());
    try!(request.write(data));
    match request.read_response() {
        Ok(response) => Ok(box response as Box<Reader>),
        Err((_request, error)) => Err(error),
    }
}
