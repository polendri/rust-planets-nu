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

    match response.read_to_string() {
        Ok(result) => Ok(result),
        Err(error) => { return Err(error) },
    }
}

#[test]
fn dummy_test2() {
}
