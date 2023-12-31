use std::fs::rename;
use axum::{
    routing::get, routing::post,
    Router,
    extract::Path, Json,
};


use serde::{
    Deserialize,
    Serialize,
};

use regex::Regex;


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
    candies_eaten_yesterday: u32,
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

        Competition { fastest, tallest, magician, consumer }
    }

    fn set_fastest(&mut self, str: u32, name: &String) -> () {
        self.fastest = format!("Speeding past the finish line with a strength of {} is {}", str, name);
    }

    fn set_tallest(&mut self, name: &String, antler_size: u32) -> () {
        self.tallest = format!("{} is standing tall with his {} cm wide antlers", name, antler_size);
    }

    fn set_magician(&mut self, name: &String, magic_power: u32) -> () {
        self.magician = format!("{} could blast you away with a snow magic power of {}", name, magic_power);
    }

    fn set_consumer(&mut self, name: &String, fave_food: &String) -> () {
        self.consumer = format!("{} ate lots of candies, but also some {}", name, fave_food)
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
    winners.set_tallest(&data[tallest].name, data[tallest].antler_width);
    winners.set_magician(&data[magician].name, data[magician].snow_magic_power);
    winners.set_consumer(&data[consumer].name, &data[consumer].favorite_food);

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

    for (idx, packet) in vec_slice.iter().enumerate() {
        if idx >= 20 {
            break;
        }
        packet_xor ^= packet;
    }

    let powed_xor: u32 = packet_xor.pow(3);

    format!("{}", powed_xor)
}

#[derive(Serialize)]
struct ElfCount {
    elf: usize,
    #[serde(rename = "elf on a shelf")]
    elf_on_a_shelf: usize,
    #[serde(rename = "shelf with no elf on it")]
    shelf_no_elf: usize

}

impl ElfCount {
    fn new() -> ElfCount {
        let mut elf: usize = 0;
        let mut elf_on_a_shelf: usize = 0;
        let mut shelf_no_elf: usize = 0;

        ElfCount{ elf, elf_on_a_shelf, shelf_no_elf}
    }

    fn set_elf(&mut self, num_elf:usize) -> () {
        self.elf = num_elf;
    }

    fn set_on_shelf(&mut self, num_elf_shelf:usize) -> () {
        self.elf_on_a_shelf = num_elf_shelf;
    }

    fn set_shelf_no_elf(&mut self, num_no_shelf:usize) -> () {
        self.shelf_no_elf = num_no_shelf;
    }
}
async fn elf_count(raw_string:String) -> Json<ElfCount> {
    let substring = "elf";
    let num_elf = raw_string.matches(substring).count();

    let substring_2 = "elf on a shelf";
    let num_elf_shelf = raw_string.matches(substring_2).count();

    // let pattern_elf = Regex::new(r"(?!elf on a )shelf").unwrap();
    // let shelf_sub_matches: Vec<_> = pattern_elf.find_iter(&raw_string).collect();

    // let num_shelf_no_elf = shelf_sub_matches.len();



    let mut elfs = ElfCount::new();
    elfs.set_elf(num_elf);
    elfs.set_on_shelf(num_elf_shelf);
    // elfs.set_shelf_no_elf(num_shelf_no_elf);

    Json(elfs)

}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/1/*tail", get(packet_recal))
        .route("/4/strength", post(determine_reindeer_strength))
        .route("/4/contest", post(reindeer_contest))
        .route("/6", post(elf_count))
        ;

    Ok(router.into())
}

