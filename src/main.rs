use tokio::net::TcpListener;
use axum::Router;
use axum::routing::post;
use axum::Json;
use serde::{Serialize, Deserialize};
use axum::http::StatusCode;

#[tokio::main]
async fn main() {
    
    let router = Router::new().route("/users", post(cria_user_handle));
    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();
    println!("rodando na porta 8080...");
    axum::serve(listener, router).await.unwrap()
}

async fn cria_user_handle(payload: Json<User>) -> Result<(StatusCode, Json<User>), StatusCode> {
    if payload.nome == "teste" {
        return Err(StatusCode::BAD_REQUEST);
    } 
    
    let user = Json(User{id:1+1, nome: payload.nome.to_string()});
    Ok((StatusCode::CREATED, user))
}

#[derive(Serialize, Deserialize)]
struct User {
    pub id: u16,
    pub nome: String,
}
