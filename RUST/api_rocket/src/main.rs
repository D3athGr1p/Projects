use reqwest::Client;
use rocket::{
    http::{uri::Origin, Status},
    response::Redirect,
    serde::json::Value,
    State,
};

#[macro_use]
extern crate rocket;

const URI_PREFIX: Origin<'static> = uri!("/run");
const GOOGLE_KEEP_DESKTOP_URL: &str = "elibroftw/google-keep-desktop-app";

async fn get_github_release(client: &State<Client>, repo: &str) -> Result<Value, reqwest::Error> {
    let uri = format!("https://api.github.com/repos/{repo}/releases/latest");
    let response = client.get(&uri).send().await?;
    let github_repos = response.json::<Value>().await?;

    Ok(github_repos)
}

#[get("/")]
fn index() -> Redirect {
    let msg: Option<&str> = None;
    Redirect::to(uri!(URI_PREFIX, get("ubuntu", "22.04", msg)))
}

#[get("/run/<_plateform>/<_version>?<msg>")]
async fn get(
    _plateform: &str,
    _version: &str,
    msg: Option<&str>,
    client: &State<Client>,
) -> Result<Value, Status> {
    if let Some(msg) = msg {
        println!("{}", msg);
        return Err(Status::NoContent);
    }

    get_github_release(client, GOOGLE_KEEP_DESKTOP_URL)
        .await
        .or(Err(Status::NoContent))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(
            reqwest::Client::builder()
                .user_agent("reqwest")
                .build()
                .unwrap(),
        )
        .mount("/", routes![index])
        .mount(URI_PREFIX, routes![get])
}
