use std::fs::rename;
use axum::{
    routing::get, routing::post,
    Router,
    extract::Path, Json,
};

use serde::{
    Deserialize,
    Serialize
};


#[derive(Deserialize)]
struct Reindeer {
    name: String,
    strength: u32,
    speed: f32,
    height: u32,
    antler_width: u32,
    snow_magic_power: u32,
    favorite_food: String,
    #[serde(rename = "cAnD13s_3ATeN-yesT3rdAy")]
    candies_eaten_yesterday: u32
}

#[derive(Serialize)]
struct Competition {
    fastest: String,
    tallest: String,
    magician: String,
    consumer: String,
}

impl Competition {
    fn new() -> Competition {
        let mut fastest = String::new();
        let mut tallest = String::new();
        let mut magician = String::new();
        let mut consumer = String::new();

        Competition{fastest:fastest, tallest:tallest, magician:magician, consumer: consumer}
    }

    fn set_fastest(&mut self, str: u32, name:&String) -> () {
        self.fastest = format!("Speeding past the finish line with a strength of {} is {}", str, name);
    }
}


#[derive(Deserialize)]
struct TailPath {
    tail: String,
}

async fn reindeer_contest(Json(data): Json<Vec<Reindeer>>) -> Json<Competition> {
    let mut fastest = 0;
    let mut fastest_speed = 0.0;
    let mut tallest = 0;
    let mut tallest_height = 0;
    let mut magician = 0;
    let mut most_magic = 0;
    let mut consumer = 0;
    let mut most_candies_eaten = 0;

    for (idx, reindeer) in data.iter().enumerate() {
        if reindeer.speed > fastest_speed {
            fastest = idx;
            fastest_speed = reindeer.speed;
        }

        if reindeer.height > tallest_height {
            tallest = idx;
            tallest_height = reindeer.height;

        }

        if reindeer.snow_magic_power > most_magic {
            magician = idx;
            most_magic = reindeer.snow_magic_power;

        }

        if reindeer.candies_eaten_yesterday > most_candies_eaten {
            consumer = idx;
            most_candies_eaten = reindeer.candies_eaten_yesterday;
        }
    }

    let mut winners = Competition::new();

    winners.set_fastest(data[fastest].strength, &data[fastest].name);

    Json(winners)
}

async fn determine_reindeer_strength(Json(data): Json<Vec<Reindeer>>) -> String {
    let mut str: u32 = 0;
    for reindeer in data {
        str += reindeer.strength;
    }
    format!("{}", str)
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
        .route("/4/strength", post(determine_reindeer_strength))
        .route("/4/contest", post(reindeer_contest))
        ;

    Ok(router.into())
}

