use std::io::Error;

use tokio::{fs::File, io::AsyncReadExt};
use websocket::WebSocketHandler;

mod websocket;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let mut file = File::open("mock_data.txt").await?;
    let mut dst = String::new();
    
    file.read_to_string(&mut dst).await?;
    let lines: Vec<String> = dst.split("\n")
        .map(|s| String::from(s))
        .collect();
    
    let ws_handler = WebSocketHandler::new("0.0.0.0", 8888, lines);
    ws_handler.init().await;
    
    loop {}
}
