use std::{sync::Arc, time::Duration, error::Error, net::SocketAddr};

use futures::SinkExt;
use log::{warn, info};
use tokio::{net::{TcpListener, TcpStream}};
use tokio_tungstenite::tungstenite::Message;

pub struct WebSocketHandler {
    addr: String,
    frequency: u32,
    mock_data: Arc<Vec<String>>,
}

struct WebSocketHandlerWorker {
    addr: String,
    frequency: u32,
    mock_data: Arc<Vec<String>>,
}

impl WebSocketHandler {
    pub fn new(host: &str, port: u32, frequency: u32, mock_data: Vec<String>) -> WebSocketHandler {
        let addr = format!("{}:{}", host, port);
        WebSocketHandler { addr, frequency, mock_data: Arc::new(mock_data) }
    }
    
    pub async fn init(&self) {
        let addr_clone = self.addr.clone();
        let mock_data_ptr = self.mock_data.clone();
        let frequency = self.frequency;
        
        let handler = WebSocketHandlerWorker::new(addr_clone, frequency, mock_data_ptr);
        if let Err(e) = handler.start().await {
            warn!("Failed to Start WebSocketHandlerWorker: {}", e.to_string());
        }
    }
}

impl WebSocketHandlerWorker {
    pub fn new(addr: String, frequency: u32, mock_data: Arc<Vec<String>>) -> WebSocketHandlerWorker {
        WebSocketHandlerWorker {
            addr,
            frequency,
            mock_data
        }
    }
    
    pub async fn start(&self) -> Result<(), Box<dyn Error>> {
        let addr_clone = self.addr.clone();
        
        info!("Binding TCP Port...");
        let listener = match TcpListener::bind(addr_clone).await {
            Ok(t) => t,
            Err(e) => {
                warn!("Failed to bind TCP Listener: {}", e.to_string());
                return Err(Box::new(e));
            }
        };
        
        // Listen for New WebSocket connections
        info!("Waiting for WebSocket Connection...");
        while let Ok((stream, in_addr)) = listener.accept().await {
            info!("Incoming Connection from: {}", in_addr.to_string());
            self.ws_send(stream, in_addr).await;
        }
        
        Ok(())
    }
    
    async fn ws_send(&self, stream: TcpStream, socket_addr: SocketAddr) {
        let mock_data_ptr = self.mock_data.clone();
        let freq = self.frequency as u64;
        
        tokio::spawn(async move {
            let mut ws_stream = match tokio_tungstenite::accept_async(stream).await {
                Ok(ws) => ws,
                Err(e) => {
                    warn!("Failed to Initiate WebSocket Handshake: {}", e.to_string());
                    return;
                }
            };
            info!("Established WebSocket Connection with: {}", socket_addr.to_string());
            
            for line in mock_data_ptr.iter() {
                if let Err(e) = ws_stream.send(Message::Text(line.clone())).await {
                    warn!("Failed to send Message to WebSocket: {}", e.to_string());
                    continue;
                }
                tokio::time::sleep(Duration::from_millis(freq)).await;
            }
            
            // Attempt to Close WebSocket Connection Gracefully
            match ws_stream.send(Message::Close(None)).await {
                Ok(_) => info!("Closed Connection with: {}", socket_addr.to_string()),
                Err(e) => warn!("Failed to Close Connection Gracefully: {}", e.to_string()),
            };
        });
    }
}
