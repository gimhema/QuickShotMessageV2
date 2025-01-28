#include "QuickShotMessage/qsm.hpp"
#include <sys/socket.h>
#include <arpa/inet.h>
#include <unistd.h>
#include <vector>
#include <iostream>

int main() {
    int sock = 0;
    struct sockaddr_in serv_addr;

    // 소켓 생성
    if ((sock = socket(AF_INET, SOCK_STREAM, 0)) < 0) {
        std::cerr << "Socket creation error" << std::endl;
        return -1;
    }

    serv_addr.sin_family = AF_INET;
    serv_addr.sin_port = htons(8080);

    // 서버 주소 설정 (127.0.0.1)
    if (inet_pton(AF_INET, "127.0.0.1", &serv_addr.sin_addr) <= 0) {
        std::cerr << "Invalid address/ Address not supported" << std::endl;
        return -1;
    }

    // 서버에 연결
    if (connect(sock, (struct sockaddr*)&serv_addr, sizeof(serv_addr)) < 0) {
        std::cerr << "Connection Failed" << std::endl;
        return -1;
    }

    // 서버로 PackedData 메시지 전송
    {
        ExampleMessage packed_data(2, "Hello Message", {5, 7, 9, 47});
        // PackedData packed_data(1, 123456789);
        std::vector<uint8_t> serialized_data = packed_data.serialize();
        send(sock, serialized_data.data(), serialized_data.size(), 0);
        std::cout << "PackedData message sent to server" << std::endl;

        // 송신 종료 신호를 서버에 전달하여 메시지를 전송 완료
        shutdown(sock, SHUT_WR);
    }

    // 서버로부터 메시지 수신 대기
    std::vector<uint8_t> buffer(2048); // 충분히 큰 버퍼 준비
    int bytes_received = recv(sock, buffer.data(), buffer.size(), 0);

    if (bytes_received > 0) {
        buffer.resize(bytes_received); // 실제 수신된 데이터만큼 버퍼 크기 조정
        handle_message(buffer); // 수신한 메시지 처리
    } else if (bytes_received == 0) {
        std::cout << "Connection closed by server" << std::endl;
    } else {
        std::cerr << "Receive error" << std::endl;
    }

    close(sock); // 소켓 종료
    return 0;
}
