#[macro_use]
extern crate nickel;

use nickel::{HttpRouter, Nickel};

fn main() {
    let mut server = Nickel::new();
    server.utilize(router! {
        get "/" => |_req,_res|{
            "Hello world!"
        }
    });

    server.get("/index", middleware!(""));
    server.listen("127.0.0.1:8081").unwrap();
}
