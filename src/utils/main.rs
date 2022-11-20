use std::fs::File;
use std::io::prelude::*;

#[tokio::main]
async fn main() {
    if let Some(subcommand) = std::env::args().nth(1) {
        if subcommand.eq("fetch_input") {
            let day: i32 = std::env::args()
                .nth(2)
                .expect("Must specify a day")
                .parse()
                .unwrap();
            fetch_input(day).await;
        } else {
            println!("Unrecognized command");
        }
    } else {
        println!("No subcommand specified");
    }
}

async fn fetch_input(day: i32) {
    let token = std::env::var("AOC_SESSION_TOKEN").expect("No AOC_SESSION_TOKEN set");
    let url = format!("https://adventofcode.com/2022/day/{}/input", day);

    let client = reqwest::Client::new();
    let res = client
        .get(url)
        .header("Cookie", format!("session={}", token))
        .send()
        .await
        .unwrap();

    let status = res.status();
    let text = res.text().await.unwrap();

    if status != 200 {
        println!("{}", text);
    } else {
        let file_name = format!("inputs/real/day{:02}.txt", day);
        let mut file = File::create(&file_name).unwrap();
        file.write(text.as_bytes()).unwrap();
        println!("Wrote input to file '{}'", file_name);
    }
}
