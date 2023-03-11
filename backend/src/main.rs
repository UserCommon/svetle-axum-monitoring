extern crate dotenv;
mod system;

use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        State,
    },
    http::Response,
    response::{IntoResponse, Json},
    routing::get,
    Router, Server,
};

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sysinfo::{CpuExt, NetworkExt, NetworksExt, ProcessExt, System, SystemExt};
use system::*;

use dotenv::dotenv;
use std::env;
use std::sync::Arc;
use tokio::sync::broadcast;

type Snapshot = Vec<f32>;
type Unsigned = Vec<u64>;

#[derive(Clone)]
struct AppState {
    tx_system: broadcast::Sender<WSSystemJson>,
}

#[derive(Serialize, Deserialize)]
struct Text(String);

#[tokio::main]
async fn main() {
    dotenv().ok();

    let (tx_system, _) = broadcast::channel::<WSSystemJson>(1);

    tracing_subscriber::fmt::init();

    let app_state = AppState {
        tx_system: tx_system.clone(),
    };

    let router = Router::new()
        .route("/", get(root))
        .route("/api/sys", get(realtime_stats_get))
        .with_state(app_state.clone());

    let bindy: String = format!(
        "{}:{}",
        std::env::var("IP").expect("IP must be set in .env file!"),
        std::env::var("PORT").expect("PORT must be set in .env file!")
    );

    tokio::task::spawn_blocking(move || {
        let mut sys = System::new();
        loop {
            sys.refresh_cpu();
            sys.refresh_memory();
            // let mut memory: Vec<u64> = vec![];

            let used_memory = sys.used_memory();
            let used_swap = sys.used_swap();
            let total_memory = sys.total_memory();
            let total_swap = sys.total_swap();

            let cpus: Vec<_> = sys.cpus().iter().map(|cpu| cpu.cpu_usage()).collect();
            let r =WSSystemJson {
                used_memory,
                used_swap,
                total_memory,
                total_swap,
                cpus_usage: cpus,
            };
            // let _ = tx_snap.send(cpus);
            // let _ = tx_u64.send(memory);
            let _ = tx_system.send(r);
            std::thread::sleep(System::MINIMUM_CPU_UPDATE_INTERVAL);
        }
    });

    axum::Server::bind(&&bindy.parse().unwrap())
        .serve(router.into_make_service())
        .await
        .unwrap();
}

async fn root() -> Json<Text> {
    Json(Text("Hey!".to_string()))
}

#[axum::debug_handler]
async fn realtime_stats_get(
    State(state): State<AppState>,
    ws: WebSocketUpgrade,
) -> impl IntoResponse {
    ws.on_upgrade(|ws: WebSocket| async { realtime_stats_stream(state, ws).await })
}

async fn realtime_stats_stream(app_state: AppState, mut ws: WebSocket) {
    let mut rx = app_state.tx_system.subscribe();

    while let Ok(msg) = rx.recv().await {
        // println!("{:?}", &msg);
        ws.send(Message::Text(serde_json::to_string(&msg).unwrap()))
            .await
            .unwrap();
    }
}


/*
async fn stats_system_get() -> impl IntoResponse {
    let mut sys = System::new();
    sys.refresh_memory();

    let total_swap = sys.total_swap();
    let total_memory = sys.total_memory();
    //let kernel = sys.kernel()

    
    Json(StaticSystemJson {
        total_memory,
        total_swap
    })
}
*/