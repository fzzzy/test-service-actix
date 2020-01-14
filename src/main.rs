use std::marker::PhantomData;
use std::pin::Pin;
use std::task::{Context, Poll};

use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::{web, App, Error, HttpResponse, HttpServer};

use futures::future::{ok, Future, Ready};

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
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, _service: S) -> Self::Future {
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
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, sreq: ServiceRequest) -> Self::Future {
        let resp = HttpResponse::Ok().body("olleh");
        let body = resp.into_body();
        let sresp = sreq.into_response(body);
        Box::pin(ok(sresp))
    }
}

fn handle_request() -> impl Future<Output = Result<HttpResponse, Error>> {
    ok(HttpResponse::Ok().body("hello"))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    println!("Hello, world!");

    HttpServer::new(|| {
        App::new()
            .wrap(SomeMiddleware::new())
            .data(web::JsonConfig::default().limit(4096))
            .service(web::resource("/").route(web::get().to(handle_request)))
    })
    .bind("127.0.0.1:3000")?
    .run()
    .await
}
