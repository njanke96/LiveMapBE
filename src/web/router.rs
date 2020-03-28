use iron::prelude::*;
use iron::{status, AfterMiddleware, Chain};
use router::{Router, NoRoute};

use crate::web::handlers;

struct NotFoundMiddleware;

impl AfterMiddleware for NotFoundMiddleware {
    fn catch(&self, _: &mut Request, err: IronError) -> IronResult<Response> {

        if err.error.is::<NoRoute>() {
            Ok(Response::with((status::NotFound, "Resource not found.")))
        } else {
            Err(err)
        }
    }
}

pub fn app() -> Chain {
    let mut router = Router::new();
    router.get("/", |_: &mut Request| -> IronResult<Response> {
        Ok(Response::with((status::Ok, "Hello World!")))
    }, "index");

    router.get("/hello/:name", handlers::hello, "hello");
    let mut chain = Chain::new(router);
    chain.link_after(NotFoundMiddleware);

    chain
}
