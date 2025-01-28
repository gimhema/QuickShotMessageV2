mod qsm2;

use qsm2::qsm::*;

use std::io::{Read, Write};
use std::net::TcpListener;

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    println!("Server listening on port 8080");

    for stream in listener.incoming() {
        let mut stream = stream?;
        let mut buffer = vec![0; 2048]; // 충분한 크기의 버퍼 생성

        // 데이터 수신
        let mut total_read = 0;
        while total_read < buffer.len() {
            let bytes_read = stream.read(&mut buffer[total_read..])?;
            if bytes_read == 0 {
                // 연결이 종료된 경우
                break;
            }
            total_read += bytes_read;
        }

        // 받은 메시지를 처리
        handle_message(&buffer);

        // 수신한 메시지를 그대로 클라이언트로 다시 전송 (Echo)
        if let Err(e) = stream.write_all(&buffer) {
            eprintln!("Failed to send echo message: {}", e);
        } else {
            println!("Echoed message back to client.");
        }
    }

    Ok(())
}
