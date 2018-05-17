extern crate actix;
extern crate actix_web;

use actix_web::*;

struct AppState;

fn index<'a>(_req: HttpRequest<AppState>) -> &'a str {
    "Hello"
}

// will fail
fn main() {
    let sys = actix::System::new("guide");
    server::HttpServer::new(
        ||
            {
                let cors = actix_web::middleware::cors::Cors::build()
                    .allowed_methods(vec!["GET", "POST", "OPTION", "DELETE", "PUT"])
                    .supports_credentials()
                    .max_age(3600)
                    .finish();
                vec![
                    App::with_state(AppState)
                        .middleware(actix_web::middleware::Logger::default())
                        .middleware(actix_web::middleware::session::SessionStorage::new(actix_web::middleware::session::CookieSessionBackend::signed(&[1; 32]).secure(false)))
                        .middleware(cors.clone())
                        .resource("/", |r| r.f(index))
                        .boxed()
                ]
            })
        .bind("127.0.0.1:8080").expect("Can not bind to 127.0.0.1:8080")
        .workers(8)
        .start();
    let _ = sys.run();
}
