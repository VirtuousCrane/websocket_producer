use std::io::Error;
use argparse::{ArgumentParser, StoreTrue, Store};
use env_logger::{Builder, Env};
use log::{warn, info};
use tokio::{fs::File, io::AsyncReadExt};
use websocket::WebSocketHandler;

mod websocket;

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Initializing the parser variables
    let mut file_path = String::new();
    let mut host = String::from("127.0.0.1");
    let mut verbose = false;
    let mut frequency: u32 = 1000;
    let mut port: u32 = 8888;
    
    // Parsing Argument variables
    {
        let mut arg_parser = ArgumentParser::new();
        arg_parser.set_description("Reads data from a file and sends it to a websocket port line by line");
        
        arg_parser.refer(&mut verbose)
            .add_option(&["-v", "--verbose"], StoreTrue, "Show log messages");
        
        arg_parser.refer(&mut host)
            .add_option(&["-H", "--host"], Store, "The WebSocket host address. Defaults to 127.0.0.1");
        
        arg_parser.refer(&mut port)
            .add_option(&["-p", "--port"], Store, "Sets the WebSocket port to send to. Defaults to 8888");
        
        arg_parser.refer(&mut file_path)
            .add_option(&["-f", "--file"], Store, "The file path to be read from.");
        
        arg_parser.refer(&mut frequency)
            .add_option(&["--frequency"], Store, "The frequency to send the data in ms. Defauls to 1000ms.");
        
        arg_parser.parse_args_or_exit();
    }
    
    // Initializes Logger
    if verbose {
        Builder::from_env(Env::default().default_filter_or("websocket_producer=trace"))
            .init();
        info!("Initialized Logger!");
    }
    
    // Tries to open the data file
    if file_path.is_empty() {
        println!("{}", file_path);
        return Err(Error::new(std::io::ErrorKind::NotFound, "No File Input Path Was Specified."));
    }
    
    info!("Reading File...");
    let mut file = match File::open(file_path).await {
        Ok(f) => f,
        Err(e) => {
            warn!("Failed to open file: {}", e.to_string());
            return Err(e);
        }
    };
    
    // Reads data in file to a temporary buffer
    let mut temp_bfr = String::new();
    file.read_to_string(&mut temp_bfr).await?;
    
    let lines: Vec<String> = temp_bfr.split("\n")
        .map(|s| String::from(s))
        .collect();
    
    // Starts the WebSocket handler
    info!("Initializing WebSocket Handler...");
    let handler = WebSocketHandler::new(&host, port, frequency, lines);
    handler.init().await;
    
    Ok(())
}
