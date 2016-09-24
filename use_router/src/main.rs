extern crate iron;
extern crate router;

use iron::prelude::*;
use iron::status;
use iron::modifiers::{Redirect};
use router::{Router, url_for};

fn main() {

    fn top_handler(_: &mut Request) -> IronResult<Response> {
        Ok(Response::with((status::Ok, "Welcome!")))
    }

    fn redirect_handler(req: &mut Request) -> IronResult<Response> {
        let ref top_url = url_for(req, "index", ::std::collections::HashMap::new());
        return Ok(Response::with((status::Found, Redirect(top_url.clone()))))
    }

    fn greet_handler(req: &mut Request) -> IronResult<Response> {
        let ref router = req.extensions.get::<Router>();
        let ref name = router
            .unwrap()
            .find("name")
            .unwrap();
        return Ok(Response::with(
            (status::Ok,
             format!("Hello {}", name).as_str())
        ));
    }
    
    let mut router = Router::new();
    router.get("/", top_handler, "index");
    router.get("/greet/:name", greet_handler, "greeting");
    router.get("/to-top", redirect_handler, "to-top");

    Iron::new(router).http("localhost:3000").unwrap();
    println!("Listen on localhost:3000");
}
