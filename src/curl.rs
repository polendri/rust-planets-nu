/*!
Contains methods to make HTTP GET and POST requests.
*/

extern crate flate2;
extern crate http;
extern crate url;

use self::flate2::reader;
use self::http::client::request;
use self::http::method;
use self::url::Url;
use std::io;

/// Possible types of encoding for an HTTP response.
enum ContentEncoding {
    NoEncoding,
    Gzip,
    Other,
}

/// Given a request, runs the request and returns a reader for the response (or an error).
///
/// If the response is gzip-compressed, it will be decompressed automatically.
fn make_request<'a>(request: request::RequestWriter) -> io::IoResult<Box<Reader + 'a>> {
    let response = try!(request.read_response().map_err(|(_request, error)| error));
    let encoding = match response.headers.content_encoding {
        Some(ref encoding) => match encoding.as_slice() {
            "gzip" => Gzip,
            _ => Other,
        },
        None => NoEncoding,
    };
    Ok(match encoding {
        Gzip => box reader::GzDecoder::new(response) as Box<Reader>,
        _ => box response as Box<Reader>,
    })
}

/// Performs an HTTP GET request, returning a reader to the response (or an error).
pub fn http_get<'a>(url: &str) -> io::IoResult<Box<Reader + 'a>> {
    let url_obj = Url::parse(url).unwrap();
    let request: request::RequestWriter = try!(request::RequestWriter::new(method::Get, url_obj));
    make_request(request)
}

/// Performs an HTTP POST request, returning a reader to the response (or an error).
pub fn http_post<'a>(url: &str, data: &str) -> io::IoResult<Box<Reader + 'a>> {
    let url_obj = Url::parse(url).unwrap();
    let mut request: request::RequestWriter = try!(request::RequestWriter::new(method::Get, url_obj));
    let data_bytes = data.as_bytes();
    request.headers.content_length = Some(data_bytes.len());
    try!(request.write(data_bytes));
    make_request(request)
}
