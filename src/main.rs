use std::io::Cursor;

use image::{codecs::png::PngDecoder, io::Reader, DynamicImage, EncodableLayout, ImageReader};
use log::info;
use reqwest::Client;
use serde_derive::Deserialize;
use tray_item::IconSource;
use tray_item::TrayItem;

static TOKEN: &str = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJleHAiOjE3Mjc0NDEzODcsInVuYW1lIjoiWnVyaWVmYWlzIn0.h5QLB3hAhVIRJdgshX-3fgfUIyWfw1vFq7mda1RYG6A";

#[tokio::main]
async fn main() {
    env_logger::init();
    let url = format!("http://localhost:2017/api/v2ray");

    let cursor = Cursor::new(include_bytes!("../static/v2box.png"));
    let decoder = png::Decoder::new(cursor);
    let mut reader = decoder.read_info().unwrap();
    let mut buf = vec![0; reader.output_buffer_size()];
    reader.next_frame(&mut buf).unwrap();

    let icon = IconSource::Data {
        data: buf,
        height: 460,
        width: 460,
    };

    let mut tray = TrayItem::new("V2rayA", icon).unwrap();
    tray.add_label("test");

    let client = reqwest::Client::new();

    run_or_turn_off(true, client, url).await;

    while true {}
}

async fn run_or_turn_off(run: bool, client: Client, url: String) {
    match client
        .delete(url)
        .header("Authorization", TOKEN)
        .send()
        .await
    {
        Ok(resp) => {
            let json: Object = resp.json().await.unwrap();
            info!("V2rayA is running: {}", json.data.running)
        }
        Err(err) => {
            info!("Reqwest Error: {}", err)
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct Object {
    pub code: String,
    pub data: Data,
}

#[derive(Deserialize, Debug)]
pub struct Data {
    pub running: bool,
    pub touch: serde_json::value::Value,
}
