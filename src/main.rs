use actix_web::{get, post, web::{self, Bytes}, App, HttpResponse, HttpServer, Responder, http::Error, rt};
use openai_api::openai_api::{ChatCompletionRequestBody, ChatCompletionRequestMessage};
use urlencoding::encode;
use serde::Serialize;
use serde_json::json;
mod openai_api;
use futures::{future::ok, stream::once, FutureExt};


#[get("/")]
async fn hello() -> String {
   "日本語Hello world!".to_string()
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[get("/stream")]
async fn stream() -> HttpResponse {
    let body = once(ok::<_, Error>(web::Bytes::from_iter(vec![1,2,3,4])));


    HttpResponse::Ok()
        .content_type("application/json")
        .streaming(body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

async fn chatgpt_post() -> HttpResponse {
    let body = ChatCompletionRequestBody{
        model: "gpt-4".to_string(),
        messages: vec![ChatCompletionRequestMessage {
            role: "user".to_string(),
            content:  encode("ラーメン好きですか").to_string(),
        }],
        stream: true
    };

    let request = openai_api::openai_api::chat_complesion_stream_request(&body);
    let s =  request.send().await.unwrap().bytes_stream();

    HttpResponse::Ok()
        .content_type("application/json")
        .streaming(s)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .service(stream)
            .route("/hey", web::get().to(manual_hello))
            .route("/gpt", web::get().to(chatgpt_post))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
