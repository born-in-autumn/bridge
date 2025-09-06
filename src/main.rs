mod config;

use std::{collections::{ HashMap }, sync::{Arc, RwLock} };

use axum::{
    extract::{Path, State}, http::StatusCode, response::Redirect, routing::{get, post}, Router
};
use uuid::Uuid;

const TEMP_URL:&str =  "https://wx.mail.qq.com/home/index?sid=zTswOYxYUGYuxWo3AEZCSQAA#/list/1/1";

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let hash_map = Arc::new(RwLock::new(HashMap::<String,String>::new()));
    let app: Router = Router::new()
        .route("/{hash}", get(root))
        .route("/shorten", post(shorten_url))
        .with_state(hash_map);

    let listenr = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listenr, app).await.unwrap();
}

async fn root(Path(id):Path<String> ,State(map): State<Arc<RwLock<HashMap<String,String>>>>) -> Redirect {
    println!("the id is: {}", id);
    let map_guard = map.read().unwrap();
    match map_guard.get(&id) {
        Some(t) => Redirect::permanent(t),
        None => Err(StatusCode::NOT_FOUND).unwrap(),
    }
}

async fn shorten_url(State(map): State<Arc<RwLock<HashMap<String,String>>>>) -> String {
    let id = Uuid::new_v4();
    println!("Generated UUID: {}", id);
    map.write().unwrap().insert(id.to_string()[..8].to_string(), TEMP_URL.to_string());
    id.to_string()
}
