mod config;
mod database;

use std::{collections::HashMap, sync::{Arc, RwLock}};

use axum::{extract::{Path, State}, http::StatusCode, response::Redirect, routing::{get, post}, Router};
use uuid::Uuid;

const TEMP_URL:&str =  "https://wx.mail.qq.com/home/index?sid=zTswOYxYUGYuxWo3AEZCSQAA#/list/1/1";


#[derive(Clone)]
pub struct AppState {
    pub hash_map: Arc<RwLock<HashMap<String,String>>>,
    pub redirect_url: String
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let port = config::get().server.port();
    let redirect_url = config::get().dev.clone().default_redirect_url;
    // println!("Using default redirect url {:?}", d);
    // println!("Listening on port {}", c);
    let hash_map = Arc::new(RwLock::new(HashMap::<String,String>::new()));
    let app: Router = Router::new()
        .route("/{hash}", get(root))
        .route("/shorten", post(shorten_url))
        .with_state(AppState {
            hash_map,
            redirect_url,
        });

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{port}")).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
async fn root(Path(hash):Path<String> ,State(state): State<AppState>) -> Redirect {
    println!("the id is: {}",hash);
    let map_guard = state.hash_map.read().unwrap();
    match map_guard.get(&hash) {
        Some(t) => Redirect::permanent(t),
        None => Err(StatusCode::NOT_FOUND).unwrap(),
    }
}

async fn shorten_url(State(state): State<AppState>) -> String {
    let id = Uuid::new_v4();
    println!("Generated UUID: {}", id);
    state.hash_map.write().unwrap().insert(id.to_string()[..8].to_string(), TEMP_URL.to_string());
    id.to_string()
}
