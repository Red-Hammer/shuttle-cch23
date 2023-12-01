use axum::{
    routing::get, Router,
    extract::Path,
};

use serde::Deserialize;

#[derive(Deserialize)]
struct TailPath {
    tail: String,
}

async fn packet_recal(Path(TailPath { tail }): Path<TailPath>) -> String {

    let packets: Vec<u32> = tail.split('/').filter_map(|s| s.parse().ok()).collect();

    let mut packet_xor: u32 = packets[0];

    let vec_slice = &packets[1..];

    for (idx,packet) in vec_slice.iter().enumerate() {
        if idx >= 20 {
            break
        }
        packet_xor ^= packet;
    }

    let powed_xor: u32 = packet_xor.pow(3);

    format!("{}", powed_xor)
}


#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/1/*tail", get(packet_recal))
        ;

    Ok(router.into())
}

