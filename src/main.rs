extern crate iron;
extern crate router;

use iron::prelude::*;
use iron::status;
use iron::mime::Mime;
use router::Router;
use std::error::Error;
use std::env;
use std::net::{Ipv4Addr, SocketAddrV4};

fn main() {

    // Get port from args
    let mut args = env::args();
    args.next();
    let port: u16 = args.next().unwrap_or(String::from("3000")).parse().unwrap_or(3000);

    // Routes
    let mut router = Router::new();
    router.get("/hello/:name", hello, "hello");
    router.get("/version", version, "version");

    // Handlers
    fn hello(req: &mut Request) -> IronResult<Response> {
        let ref name = req.extensions.get::<Router>().unwrap().find("name").unwrap();
        let content_type : Mime = "application/json".parse().unwrap();
        Ok(Response::with((content_type, status::Ok, format!("{{\"response\":\"Hello {}!\"}}", name))))
    }
    fn version(_: &mut Request) -> IronResult<Response> {
        Ok(Response::with(("v0.1.0", status::Ok)))
    }

    // Server
    match Iron::new(router).http(SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), port)) {
        Ok(_) => println!("Listening on {}", port),
        Err(e) => println!("Error: {}", e.description()),
    };

}