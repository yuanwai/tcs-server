use axum::{response::IntoResponse, routing::post, Router};
use hyper::http::Method;
use serde::Deserialize;
use std::{fs::OpenOptions, io::Write, net::SocketAddr};
use tower_http::{
    add_extension::AddExtensionLayer,
    cors::{Any, CorsLayer},
};

#[derive(Deserialize)]
struct Log {
    time: String,
    client_ip: String,
    log_content: String,
}

async fn save_log(log: axum::extract::Json<Log>) -> impl IntoResponse {
    let log = log.0;
    // 构造日志行
    let log_line = format!("{} : {} | {}\n", log.time, log.client_ip, log.log_content);
    println!("============ run app...log:  {}", log_line);

    // 打开日志文件，将日志行写入
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("C:\\Users\\win10\\Desktop\\logs.txt")
        .expect("Failed to open log file");

    file.write_all(log_line.as_bytes())
        .expect("Failed to write to log file");

    "Log received and saved.".to_string().into_response()
}

#[tokio::main]
async fn main() {
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/save_log", post(save_log))
        .layer(AddExtensionLayer::new(()))
        .layer(cors);
    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .unwrap();
}
