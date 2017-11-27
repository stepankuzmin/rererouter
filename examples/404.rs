extern crate iron;
extern crate regex;
extern crate rererouter;

use regex::Captures;
use iron::prelude::{Iron};
use iron::{status, Request, Response};
use rererouter::RouterBuilder;

fn main() {
    let mut router_builder = RouterBuilder::new();

    router_builder.get(r"/hello-(?P<name>\w*)", |_: &mut Request, captures: Captures| {
        let greeting = format!("Hello, {}!", &captures["name"]);
        Ok(Response::with((status::Ok, greeting)))
    });

    router_builder.not_found(|_: &mut Request| {
        Ok(Response::with((status::NotFound, "Not found")))
    });

    let router = router_builder.finalize();
    Iron::new(router).http("localhost:3000").unwrap();
}