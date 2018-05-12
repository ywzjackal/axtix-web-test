extern crate actix;
extern crate actix_web;

use actix::*;
use actix_web::*;
use std::cell::Cell;

struct AppState {
    counter: Cell<usize>,
}

fn index(req: HttpRequest<AppState>) -> String {
    let count = req.state().counter.get() + 1; // <- get count
    req.state().counter.set(count);            // <- store new count in state

    format!("Request number: {}", count)       // <- response with count
}
// will fail
fn main() {
    let sys = actix::System::new("guide");
    server::HttpServer::new(
        || vec![
            App::with_state(AppState { counter: Cell::new(0) })
                .resource("/", |r| r.f(index))
        ])
        .bind("127.0.0.1:8080").expect("Can not bind to 127.0.0.1:8080")
        .workers(8)
        .start();
    let _ = sys.run();
}

// will success
//fn main() {
//    let sys = actix::System::new("guide");
//    server::HttpServer::new(
//        || {
//            App::with_state(AppState { counter: Cell::new(0) })
//                .resource("/", |r| r.f(index))
//        })
//        .bind("127.0.0.1:8080").expect("Can not bind to 127.0.0.1:8080")
//        .workers(8)
//        .start();
//    let _ = sys.run();
//}
