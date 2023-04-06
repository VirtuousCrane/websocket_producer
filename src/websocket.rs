

use std::{sync::Arc, time::Duration};

use futures::SinkExt;
use log::{warn, info};
use futures_util::{future, StreamExt, TryStreamExt};
use tokio::{task::JoinHandle, net::{TcpListener, TcpStream}};
use tokio_tungstenite::{WebSocketStream, tungstenite::Message};

pub struct WebSocketHandler {
    addr: String,
    mock_data: Vec<String>,
}

struct WebSocketHandlerWorker {
    addr: String,
    mock_data: Arc<Vec<String>>,
}

impl WebSocketHandler {
    pub fn new(host: &str, port: i32, mock_data: Vec<String>) -> WebSocketHandler {
        let addr = format!("{}:{}", host, port);
        WebSocketHandler { addr, mock_data }
    }
    
    pub async fn init(&self) {
        let addr_clone = self.addr.clone();
        let mock_data_clone = self.mock_data.clone();
        
        tokio::spawn(async move {
            let handler = WebSocketHandlerWorker::new(addr_clone, mock_data_clone);
            handler.start().await;
        });
    }
}

impl WebSocketHandlerWorker {
    pub fn new(addr: String, mock_data: Vec<String>) -> WebSocketHandlerWorker {
        WebSocketHandlerWorker {
            addr,
            mock_data: Arc::new(mock_data)
        }
    }
    
    pub async fn start(&self) {
        let _listener_thread_handle = self.start_listener_thread().await;
    }
    
    async fn start_listener_thread(&self) -> Option<JoinHandle<()>> {
        let addr_clone = self.addr.clone();
        
        let listener = match TcpListener::bind(addr_clone).await {
            Ok(t) => t,
            Err(e) => {
                warn!("Failed to bind TCP Listener: {}", e.to_string());
                return None;
            }
        };
        
        while let Ok((stream, _)) = listener.accept().await {
            self.listener_process(stream).await;
        }
        
        None
    }
    
    async fn listener_process(&self, stream: TcpStream) {
        let mock_data_ptr = self.mock_data.clone();
        
        tokio::spawn(async move {
            let mut ws_stream = match tokio_tungstenite::accept_async(stream).await {
                Ok(ws) => ws,
                Err(e) => {
                    warn!("Failed to Initiate WebSocket Handshake: {}", e.to_string());
                    return;
                }
            };
            
            for line in mock_data_ptr.iter() {
                if let Err(e) = ws_stream.send(Message::Text(line.clone())).await {
                    warn!("Failed to send Message to WebSocket: {}", e.to_string());
                    continue;
                }
                tokio::time::sleep(Duration::from_millis(200)).await;
            }
            
        });
    }
}
