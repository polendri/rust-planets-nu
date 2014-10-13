extern crate http;
extern crate url;

use self::http::client::RequestWriter;
use self::http::method::Get;
use self::url::Url;
use std::io::IoResult;

pub fn http_get(url: &str) -> IoResult<String> {
    let url_obj = Url::parse(url).unwrap();
    let request: RequestWriter = try!(RequestWriter::new(Get, url_obj));
    let mut response = try!(request.read_response().map_err(|(_request, error)| error));
    response.read_to_string()
}

pub fn http_post(url: &str, data: &[u8]) -> IoResult<String> {
    let url_obj = Url::parse(url).unwrap();
    let mut request: RequestWriter = try!(RequestWriter::new(Get, url_obj));
    request.headers.content_length = Some(data.len());
    try!(request.write(data));
    let mut response = try!(request.read_response().map_err(|(_request, error)| error));
    response.read_to_string()
}
