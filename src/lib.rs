extern crate iron;
extern crate regex;

use std::collections::HashMap;
use iron::method::Method;
use iron::{status, Handler, Request, Response, IronResult};
use regex::{Regex, Captures};

type RouteHandler = fn(&mut Request, Captures) -> IronResult<Response>;
type NotFoundHandler = fn(&mut Request) -> IronResult<Response>;
type Routes = HashMap<Method, Vec<(Regex, RouteHandler)>>;

pub struct Router {
    routes: Routes,
    not_found: NotFoundHandler,
}

impl Handler for Router {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        let url = ["/", &req.url.path().join("/")].concat();
        let routes = self.routes.get(&req.method).unwrap();
        match routes.clone().into_iter().find(|ref x| x.0.is_match(&url)) {
            Some((re, handler)) => {
                let captures = re.captures(&url).unwrap();
                handler(req, captures)
            }
            None => (self.not_found)(req)
        }
    }
}

pub struct RouterBuilder {
    pub routes: Routes,
    pub not_found: Option<NotFoundHandler>
}

impl RouterBuilder {
    pub fn new() -> RouterBuilder {
        let mut routes = HashMap::new();
        routes.insert(Method::Get, Vec::new());
        routes.insert(Method::Post, Vec::new());
        routes.insert(Method::Put, Vec::new());
        routes.insert(Method::Delete, Vec::new());

        RouterBuilder {
            routes: routes,
            not_found: None
        }
    }

    fn route(&mut self, method: Method, pattern: &str, handler: RouteHandler) -> &mut RouterBuilder {
        let pattern = [r"\A", pattern, r"\z"].join("");
        let re = Regex::new(&pattern).unwrap();
        self.routes.get_mut(&method).unwrap().push((re, handler));
        self
    }

    pub fn get(&mut self, pattern: &str, handler: RouteHandler) -> &mut RouterBuilder {
        self.route(Method::Get, pattern, handler)
    }

    pub fn post(&mut self, pattern: &str, handler: RouteHandler) -> &mut RouterBuilder {
        self.route(Method::Post, pattern, handler)
    }

    pub fn put(&mut self, pattern: &str, handler: RouteHandler) -> &mut RouterBuilder {
        self.route(Method::Put, pattern, handler)
    }

    pub fn delete(&mut self, pattern: &str, handler: RouteHandler) -> &mut RouterBuilder {
        self.route(Method::Delete, pattern, handler)
    }

    pub fn not_found(&mut self, handler: NotFoundHandler) -> &mut RouterBuilder {
        self.not_found = Some(handler);
        self
    }

    pub fn finalize(self) -> Router {
        Router {
            routes: self.routes,
            not_found: self.not_found.unwrap_or_else(|| default_not_found)
        }
    }
}

fn default_not_found(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::NotFound)))
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
