use axum::{
    routing::{post,get},
    Router
};
use serde::Deserialize;
use std::{fs::OpenOptions,io::Write};

#[derive(Deserialize)]
struct Log {
    time: String,
    client_ip: String,
    log_content: String,
}


async fn save_log(log: axum::extract::Json<Log>) -> String {
    let log = log.0;

    // 构造日志行
    let log_line = format!("{} - {}: {}\n", log.time, log.client_ip, log.log_content);

    // 打开日志文件，将日志行写入
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("logs.txt")
        .expect("Failed to open log file");

    file.write_all(log_line.as_bytes())
        .expect("Failed to write to log file");

    "Log received and saved.".to_string()
}

#[tokio::main]
async fn main() {
    

    let app = Router::new()
    .route("/", get(|| async { "Hello, World!" }))
    .route("/log", post(save_log()));

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}