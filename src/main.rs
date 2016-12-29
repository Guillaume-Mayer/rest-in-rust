extern crate iron;
extern crate router;

use iron::prelude::*;
use iron::status;
use iron::mime::Mime;
use router::Router;
use std::error::Error;
use std::env;

fn main() {

    // Get port from args
    let mut args = env::args();
    args.next();
    let port : u16 = args.next().unwrap_or(String::from("3000")).parse().unwrap_or(3000);
    println!("Port: {}", port);

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
    match Iron::new(router).http(("localhost", port)) {
        Ok(_) => println!("Listening on {}", port),
        Err(e) => println!("Error: {}", e.description()),
    };
    
}