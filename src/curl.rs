extern crate flate2;
extern crate http;
extern crate url;

use self::flate2::reader;
use self::http::client::request;
use self::http::method;
use self::url::Url;
use std::io;

fn make_request<'a>(request: request::RequestWriter) -> io::IoResult<Box<Reader + 'a>> {
    let response = try!(request.read_response().map_err(|(_request, error)| error));
    let encoding = response.headers.content_encoding.clone();
    let boxed_reader = match encoding {
        Some(encoding) => match encoding.as_slice() {
            "gzip" => box reader::GzDecoder::new(response) as Box<Reader>,
            _ => box response as Box<Reader>,
        },
        None => box response as Box<Reader>,
    };
    Ok(boxed_reader)
}

pub fn http_get<'a>(url: &str) -> io::IoResult<Box<Reader + 'a>> {
    let url_obj = Url::parse(url).unwrap();
    let request: request::RequestWriter = try!(request::RequestWriter::new(method::Get, url_obj));
    make_request(request)
}

pub fn http_post<'a>(url: &str, data: &str) -> io::IoResult<Box<Reader + 'a>> {
    let url_obj = Url::parse(url).unwrap();
    let mut request: request::RequestWriter = try!(request::RequestWriter::new(method::Get, url_obj));
    let data_bytes = data.as_bytes();
    request.headers.content_length = Some(data_bytes.len());
    try!(request.write(data_bytes));
    make_request(request)
}
