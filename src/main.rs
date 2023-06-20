use axum::{
    routing::get,
    Router, Json,
};

use serde::Serialize;

#[derive(Serialize)]
pub struct User{
    pub id:i32,
    pub username:String,
    pub age:i32,
} 


#[tokio::main]
async fn main() {
    // build our application with a single route

    let json = || async move {
        let new_user =User{
            id : 1,
            username : String::from("lxd"),
            age : 39,
        };
        Ok::<_, std::convert::Infallible>(Json(new_user))
    };

    let app = Router::new()
    .route("/", get(|| async { "Hello, World!" }))
    .route("/json", get(json));

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}