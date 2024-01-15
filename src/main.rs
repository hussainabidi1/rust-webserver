use actix::{Actor, StreamHandler};
use actix_files::{Files, NamedFile};
use actix_web::{get, web, App, Error, HttpRequest, HttpResponse, HttpServer, Responder};
use actix_web_actors::ws::{start, Message, ProtocolError, WebsocketContext};
use std::io;

struct WsActor;

impl Actor for WsActor {
    type Context = WebsocketContext<Self>;
}

impl StreamHandler<Result<Message, ProtocolError>> for WsActor {
    fn handle(&mut self, msg: Result<Message, ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(Message::Text(_text)) => ctx.text("no u"),
            _ => (),
        }
    }
}

#[get("/ws")]
async fn handle_websockets(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let resp = start(WsActor {}, &req, stream);
    resp
}

#[get("/")]
async fn index() -> io::Result<NamedFile> {
    Ok(NamedFile::open("public/index.html")?)
}

async fn not_found() -> impl Responder {
    HttpResponse::NotFound().body("404 Not Found")
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(handle_websockets)
            .service(Files::new("/public", "./public"))
            .default_service(web::to(not_found as fn() -> _))
    })
    .bind(("127.0.0.1", 3000))?
    .run()
    .await
}
