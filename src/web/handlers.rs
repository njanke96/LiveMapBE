use iron::prelude::*;
use iron::status;
use router::Router;

pub fn hello(req: &mut Request) -> IronResult<Response> {
    let params = req.extensions.get::<Router>().unwrap();
    let name = params.find("name").unwrap();

    Ok(Response::with((status::Ok, format!("Hello, {}", name))))
}
