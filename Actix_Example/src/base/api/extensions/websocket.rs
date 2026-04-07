use actix_web::{Error, HttpRequest, HttpResponse, web};
use actix_ws::Message;

pub struct WebSocketEndpoint;

impl WebSocketEndpoint {
    pub async fn handler(req: HttpRequest, body: web::Payload) -> Result<HttpResponse, Error> {
        let (response, mut session, mut stream) = actix_ws::handle(&req, body)?;

        actix_web::rt::spawn(async move {
            while let Some(Ok(msg)) = stream.recv().await {
                if let Message::Text(text) = msg {
                    let clean = ammonia::clean(&text);
                    let _ = session.text(format!("Echo: {clean}")).await;
                }
            }
        });

        Ok(response)
    }
}
