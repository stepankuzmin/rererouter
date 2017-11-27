# rererouter

[![Build Status](https://travis-ci.org/stepankuzmin/rererouter.svg?branch=master)](https://travis-ci.org/stepankuzmin/rererouter)
[![Crates.io Status](http://meritbadge.herokuapp.com/rererouter)](https://crates.io/crates/rererouter)

[Iron](https://github.com/iron/iron) router with [regex](https://github.com/rust-lang/regex) captures support.

### Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
rererouter = "0.1"
```

and this to your crate root:

```rust
extern crate rererouter;
```

### Example

```rust
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

    router_builder.not_found(|_: &mut Request| {
        Ok(Response::with((status::NotFound, "Not found")))
    });

    let router = router_builder.finalize();
    Iron::new(router).http("localhost:3000").unwrap();
}
```

Usage:

```shell
$ curl -i http://localhost:3000/hello-rererouter

HTTP/1.1 200 OK
Content-Length: 18
Content-Type: text/plain
Date: Mon, 27 Nov 2017 08:36:47 GMT

Hello, rererouter!
```

```shell
$ curl -i -X POST http://localhost:3000/count-to-10

HTTP/1.1 200 OK
Content-Length: 18
Content-Type: text/plain
Date: Mon, 27 Nov 2017 08:37:19 GMT

Let's count to 10!
```

```shell
$ curl -i -X POST http://localhost:3000/not-found

HTTP/1.1 404 Not Found
Content-Length: 9
Content-Type: text/plain
Date: Mon, 27 Nov 2017 08:38:21 GMT

Not found
```