#![allow(unused_variables)]
extern crate actix;
extern crate actix_web;
extern crate env_logger;
extern crate futures;

use actix::*;
use actix_web::*;

use futures::{Future, Stream};
use futures::future::{result, Either};

use std::fs::File;
use std::io::prelude::*;

fn javascript(_: HttpRequest) -> Result<HttpResponse> {
    // css
		let css = include_str!("../website/main.js");
		
    // response
    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("text/javascript; charset=utf-8")
        .body(css).unwrap())
}

fn stylesheet(_: HttpRequest) -> Result<HttpResponse> {
    // css
		let css = include_str!("../website/main.css");
		
    // response
    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("text/css; charset=utf-8")
        .body(css).unwrap())
}

/// simple index handler
fn index(_: HttpRequest) -> Result<HttpResponse> {
    // html
		let html = include_str!("../website/index.xhtml");
		
    // response
    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(html).unwrap())
}

fn upload(mut req: HttpRequest) -> Box<Future<Item=HttpResponse, Error=Error>> {
	println!("OK");
	println!("{:?}", req);

	req.multipart()            // <- get multipart stream for current request
		.from_err()            // <- convert multipart errors
		.and_then(|item| {     // <- iterate over multipart items
			match item {
				// Handle multipart Field
				multipart::MultipartItem::Field(field) => {
					println!("==== FIELD ==== {:?}", field);
					let mut buffer = File::create("foo.txt").unwrap();

					// Field in turn is stream of *Bytes* object
					Either::A(
						field.map_err(Error::from)
						.map(move |chunk| {
							buffer.write_all(&chunk[..]).unwrap();
							println!("-- CHUNK: \n{}",
											 std::str::from_utf8(&chunk).unwrap());})
						.finish())
				},
				multipart::MultipartItem::Nested(mp) => {
					// Or item could be nested Multipart stream
					Either::B(result(Ok(())))
				}
			}
		})
	.finish()  // <- Stream::finish() combinator from actix
		.map(|_| httpcodes::HTTPOk.into())
		.responder()
}

fn main() {
	::std::env::set_var("RUST_LOG", "actix_web=info");
	env_logger::init();
	let sys = actix::System::new("multipart-example");

	let addr = HttpServer::new(
		|| Application::new()
		.middleware(middleware::Logger::default()) // <- logger
		.resource("/upload", |r| r.method(Method::POST).a(upload))
		.resource("/index.html", |r| r.f(index))
		.resource("/main.css", |r| r.f(stylesheet))
		.resource("/main.js", |r| r.f(javascript))
		.resource("/", |r| r.f(index))
	).bind("127.0.0.1:8080").unwrap()
	 .start();

	println!("Starting http server: 127.0.0.1:8080");
	let _ = sys.run();
}
