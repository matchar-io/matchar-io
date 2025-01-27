use crate::UserSession;
use axum::{
    extract::{
        ws::{Message as AxumMessage, WebSocket},
        WebSocketUpgrade,
    },
    response::IntoResponse,
    Extension,
};
use futures::{
    stream::{SplitSink, SplitStream},
    SinkExt, StreamExt,
};
use matchar_app_service::user::domain::UserActor;
use postbox::PostOffice;
use tunnel::{Message, Receiver};

pub async fn handler(
    UserSession(session): UserSession,
    ws: WebSocketUpgrade,
    Extension(mut office): Extension<PostOffice>,
) -> impl IntoResponse {
    let user_id = session.user_id();

    ws.on_upgrade(move |socket| async move {
        let (client_emitter, client_receiver) = socket.split();
        let (tunnel_emitter, tunnel_receiver) = tunnel::channel(20);

        let user = UserActor::new(user_id, tunnel_emitter.clone());
        let user = office.spawn(user_id.as_uuid(), user);

        tokio::spawn(to_client(tunnel_receiver, client_emitter, office.clone()));
        tokio::spawn(from_client(client_receiver, tunnel_emitter, office));
    })
}

async fn to_client(
    mut receiver: Receiver,
    mut emitter: SplitSink<WebSocket, AxumMessage>,
    _office: PostOffice,
) {
    loop {
        match receiver.message().await {
            Some(Message::Event(text)) => {
                let _ = emitter.send(AxumMessage::text(text)).await;
            }
            Some(Message::Close) => break,
            None => {
                // NO OP
            }
        }
    }
}

async fn from_client(
    mut receiver: SplitStream<WebSocket>,
    mut emitter: tunnel::Emitter,
    _office: PostOffice,
) {
    while let Some(message) = receiver.next().await {
        match message {
            Ok(AxumMessage::Text(_text)) => {
                // TODO: Implement
            }
            Ok(AxumMessage::Close(_)) => break,
            _ => {
                // NO OP
            }
        }
    }
}
