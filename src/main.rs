use std::marker::PhantomData;

use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::{middleware, web, App, Error, HttpResponse, HttpServer};

use futures::future::{ok, Future, FutureResult};

struct SomeMiddleware {}

impl SomeMiddleware {
    pub fn new() -> Self {
        SomeMiddleware {}
    }
}

impl<S, B> Transform<S> for SomeMiddleware
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = SomeMiddlewareService<S>;
    type Future = FutureResult<Self::Transform, Self::InitError>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(SomeMiddlewareService {
            phantom: PhantomData,
        })
    }
}

struct SomeMiddlewareService<S> {
    phantom: PhantomData<S>,
}

impl<S, B> Service for SomeMiddlewareService<S>
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Box<dyn Future<Item = Self::Response, Error = Self::Error>>;

    fn poll_ready(&mut self) -> Result<futures::Async<()>, Self::Error> {
        return Ok(futures::Async::Ready(()));
    }

    fn call(&mut self, sreq: ServiceRequest) -> Self::Future {
        Box::new(ok(sreq.into_response(HttpResponse::Ok().body("olleh"))))
    }
}

pub fn handle_request() -> impl Future<Item = HttpResponse, Error = Error> {
    ok(HttpResponse::Ok().body("hello"))
}

fn main() -> std::io::Result<()> {
    println!("Hello, world!");

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(SomeMiddleware::new())
            .data(web::JsonConfig::default().limit(4096))
            .service(web::resource("/").route(web::get().to_async(handle_request)))
    })
    .bind("127.0.0.1:3000")?
    .run()
}
