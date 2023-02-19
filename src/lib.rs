pub mod twitch;

#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[shuttle_service::main]
async fn rocket() -> shuttle_service::ShuttleRocket {
    let rocket = rocket::build().mount("/hello", routes![index]);

    twitch::initialize_twitch_chat("techygrrrl").await;

    Ok(rocket)
}