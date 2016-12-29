extern crate iron;
extern crate router;

use iron::prelude::*;
use iron::status;
use iron::mime::Mime;
use router::Router;
use std::error::Error;

fn main() {

    // Routes
    let mut router = Router::new();
    router.get("/hello/:name", hello, "hello");

    // Handlers
    fn hello(req: &mut Request) -> IronResult<Response> {
        let ref name = req.extensions.get::<Router>().unwrap().find("name").unwrap_or("/");
        let content_type : Mime = "application/json".parse().unwrap();
        Ok(Response::with((content_type, status::Ok, format!("{{\"response\":\"Hello {}!\"}}", name))))
    }

    // Server
    match Iron::new(router).http("localhost:3000") {
        Ok(_) => println!("Listening on 3000"),
        Err(e) => println!("Error: {}", e.description()),
    };
    
}