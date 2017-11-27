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

    router_builder.post(r"/count-to-(?P<count>\d*)", |_: &mut Request, captures: Captures| {
        let count = format!("Let's count to {}!", &captures["count"]);
        Ok(Response::with((status::Ok, count)))
    });

    let router = router_builder.finalize();
    Iron::new(router).http("localhost:3000").unwrap();
}