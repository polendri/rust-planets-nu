extern crate http;
extern crate url;

use self::http::client;
use self::http::client::request;
use self::http::client::response;
use self::http::method;
use self::url::Url;
use std::io;

pub fn http_get(url: &str) -> io::IoResult<response::ResponseReader<client::NetworkStream>> {
    let url_obj = Url::parse(url).unwrap();
    let request: request::RequestWriter = try!(request::RequestWriter::new(method::Get, url_obj));
    request.read_response().map_err(|(_request, error)| error)
}

pub fn http_post(url: &str, data: &[u8]) -> io::IoResult<response::ResponseReader<client::NetworkStream>> {
    let url_obj = Url::parse(url).unwrap();
    let mut request: request::RequestWriter = try!(request::RequestWriter::new(method::Get, url_obj));
    request.headers.content_length = Some(data.len());
    try!(request.write(data));
    request.read_response().map_err(|(_request, error)| error)
}
