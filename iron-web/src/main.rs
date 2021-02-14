extern crate iron;

use iron::mime;
use iron::prelude::*;
use iron::status;

fn main() {
    println!("SErving on http://localhost:9999...");
    Iron::new(get_form).http("localhost:9999").unwrap();
}

fn get_form(_: &mut Request) -> IronResult<Response> {
    let mut response = Response::new();
    let mime: mime::Mime = "text/html".parse().unwrap();
    response.set_mut(status::Ok);
    response.set_mut(mime);
    response.set_mut(
        r#"
        <title>GCD Calculator</title>
        <form action="/gcd" method="post">
            <input type="text" name="n" />
            <input type="text" name="n" />
            <button type="submit">Compoute GCD</button>
        </form>
    "#,
    );

    return Ok(response);
}
