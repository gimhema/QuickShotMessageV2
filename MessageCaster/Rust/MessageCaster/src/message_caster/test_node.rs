use std::io::{Read, Write};
use std::net::TcpListener;
use super::caster::*;

pub struct QTestNode {
    net_info: String,
}

impl QTestNode {
    pub fn new(_net_info: String) -> QTestNode {
        Self { net_info: _net_info }
    }

    pub fn listen(&mut self) -> std::io::Result<()> {
        let listener = TcpListener::bind(self.net_info.clone())?; // `?`를 사용하여 오류 처리

        println!("Server listening on {}", self.net_info);

        for stream in listener.incoming() {
            match stream {
                Ok(mut stream) => {
                    let mut buffer = vec![0; 2048]; // 충분한 크기의 버퍼 생성

                    // 데이터 수신
                    let bytes_read = stream.read(&mut buffer)?;
                    if bytes_read == 0 {
                        println!("Client disconnected.");
                        continue;
                    }

                    // 받은 메시지를 처리
                    handle_quicksot_message(&buffer); // 받은 크기만큼 처리

                    // 수신한 메시지를 그대로 클라이언트로 다시 전송 (Echo)
                    if let Err(e) = stream.write_all(&buffer[..bytes_read]) {
                        eprintln!("Failed to send echo message: {}", e);
                    } else {
                        println!("Echoed message back to client.");
                    }
                }
                Err(e) => {
                    eprintln!("Failed to accept connection: {}", e);
                }
            }
        }

        Ok(())
    }
}
