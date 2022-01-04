#[macro_use]
extern crate rocket;
extern crate bitcoincore_rpc;

use bitcoincore_rpc::{Auth, Client, RpcApi};
use rocket::serde::{json::Json, Deserialize, Serialize};
use std::env;

#[derive(Deserialize)]
struct UnbrodcastRawTransaction<'a> {
    pub raw_transaction: &'a str,
}

#[derive(Serialize)]
struct Success {
    transaction_id: String,
}

#[derive(Serialize)]
struct Error {
    message: String,
}

#[post("/broadcast", data = "<data>")]
fn broadcast(data: Json<UnbrodcastRawTransaction<'_>>) -> Result<Json<Success>, Json<Error>> {
    let rpc = Client::new(
        "http://garlicoind:42068",
        Auth::UserPass(
            env::var("RPCUSER").unwrap(),
            env::var("RPCPASSWORD").unwrap(),
        ),
    )
    .unwrap();

    let transaction_id = rpc
        .send_raw_transaction(data.raw_transaction)
        .map_err(|err| Error {
            message: err.to_string(),
        })?;

    Ok(Json(Success {
        transaction_id: transaction_id.to_string(),
    }))
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, broadcast])
}
