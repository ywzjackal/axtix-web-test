extern crate actix;
extern crate actix_web;
extern crate futures;

use actix_web::*;
use actix_web::http::Method;
use futures::*;

pub fn handle_multipart_field(field: multipart::Field<HttpRequest>) -> Box<Future<Item=i64, Error=Error>> {
    use std::fs;
    use std::io::Write;
    let file_path_string = "upload.png";
    let mut file = match fs::File::create(file_path_string.clone()) {
        Ok(file) => file,
        Err(e) => return Box::new(future::err(error::ErrorInternalServerError(e)))
    };
    // debug!("handle_multipart_field...");
    Box::new(field.fold(0i64, move |acc, bytes| {
        let rt = file.write_all(bytes.as_ref()).map(|_| {
            acc + bytes.len() as i64
        }).map_err(|e| { 
            println!("handle_multipart_field file.write_all failed: {:?}", e);
            error::MultipartError::Payload(error::PayloadError::Io(e))
        });
        future::result(rt)
    }).map_err(|e|{
        println!("handle_multipart_field failed, {:?}", e);
        error::ErrorInternalServerError(e)
    }))
}

pub fn handle_multipart_item(item: multipart::MultipartItem<HttpRequest>) -> Box<Stream<Item=i64, Error=Error>> {
    // debug!("handle_multipart_item...");
    match item {
        multipart::MultipartItem::Field(field) => {
            // debug!("handle_multipart_item field");
            Box::new(handle_multipart_field(field).into_stream())
        }
        multipart::MultipartItem::Nested(mp) => {
            // debug!("handle_multipart_item nested");
            handle_multipart(mp)
        }
    }
}

pub fn handle_multipart(mp: multipart::Multipart<HttpRequest>) -> Box<Stream<Item=i64, Error=Error>> {
    // debug!("handle_multipart ...");
    Box::new(mp.map_err(error::ErrorInternalServerError)
        .map(move |item| handle_multipart_item(item)).flatten())
}

pub fn upload(req: HttpRequest) -> Box<Future<Item=HttpResponse, Error=Error>> {
    // debug!("upload handler...");
    Box::new(handle_multipart(req.clone().multipart()).collect().map(|sizes| {
        HttpResponse::Ok().json(sizes)
    }).map_err(|e| {
        println!("{:?}", e);
        e
    }))
}

fn index(_req: HttpRequest) -> Result<HttpResponse> {
   let html = r#"<html>
        <head><title>Upload Test</title></head>
        <body>
            <form target="/" method="post" enctype="multipart/form-data">
                <input type="file" name="file"/>
                <input type="submit" value="Submit"></button>
            </form>
        </body>
    </html>"#;

    return Ok(HttpResponse::Ok().body(html))
}

fn main() {
    let sys = actix::System::new("upload-test");
    server::HttpServer::new(
        ||
            {
                vec![
                    App::new()
                        .middleware(actix_web::middleware::Logger::default())
                        .resource("/", |r| {
                            r.method(Method::GET).with(index);
                            r.method(Method::POST).with(upload);
                        })
                        .boxed()
                ]
            })
        .bind("127.0.0.1:8080").expect("Can not bind to 127.0.0.1:8080")
        .start();
    let _ = sys.run();
}
