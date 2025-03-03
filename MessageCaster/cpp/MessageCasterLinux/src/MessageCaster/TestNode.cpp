#include "TestNode.h"
#include "MessageCaster.h"
#include <iostream>
#include <vector>
#include <thread>
#include <cstring>
#include <unistd.h>
#include <memory>
#include <sstream>

TestNode::TestNode() : sock(-1)
{

}

TestNode::~TestNode()
{
    if (sock!= -1) {
        close(sock);
    }
}

void TestNode::Init(std::string _ip_address, int _port)
{
    this->ip_address = _ip_address;
    this->port = _port;
}

int TestNode::Run()
{
    struct sockaddr_in node_addr;

    std::unique_ptr<MessageCaster> m_caster = std::make_unique<MessageCaster>();

    if ((sock = socket(AF_INET, SOCK_STREAM, 0)) < 0) {
        std::cerr << "Socket creation error" << std::endl;
        return -1;
    }

    node_addr.sin_family = AF_INET;
    node_addr.sin_port = htons(this->port);

    if (inet_pton(AF_INET, this->ip_address.c_str(), &node_addr.sin_addr) <= 0) {
        std::cerr << "Invalid address/ Address not supported" << std::endl;
        close(sock);
        return -1;
    }

    if (connect(sock, (struct sockaddr*)&node_addr, sizeof(node_addr)) < 0) {
        std::cerr << "Connection Failed" << std::endl;
        close(sock);
        return -1;
    }

    std::thread send_thread(&TestNode::SendMessage, this);

    std::vector<uint8_t> buffer(2048);

    // 지속적으로 메시지를 수신하는 루프
    while (true) {
        int bytes_received = recv(sock, buffer.data(), buffer.size(), 0);

        if (bytes_received > 0) {
            buffer.resize(bytes_received);
            m_caster->HandleMessage(buffer); // 수신된 메시지 처리
        } else if (bytes_received == 0) {
            std::cout << "Connection closed by server" << std::endl;
            break;
        } else {
            std::cerr << "Receive error" << std::endl;
            break;
        }
    }

    if (send_thread.joinable()) {
        send_thread.join();
    }

    close(sock); // 루프 종료 후 소켓 닫기
    return 0;
}

void TestNode::SendMessage()
{
    //ExampleMessage packed_data(2, "Hello Message", {5, 7, 9, 47});
    //    // PackedData packed_data(1, 123456789);
    //    std::vector<uint8_t> serialized_data = packed_data.serialize();
    //    send(sock, serialized_data.data(), serialized_data.size(), 0);
    //    std::cout << "PackedData message sent to server" << std::endl;
    while (true) {
        std::string message;
        std::cout << "Enter message to send (or type 'exit' to quit): ";
        std::getline(std::cin, message);

        if (message == "exit") {
            std::cout << "Closing connection..." << std::endl;
            close(sock);
            break;
        }

        // Input Use Case : 1001 3.14 Player1 10 20 30 40
        // std::string input = "1001 3.14 Player1 10 20 30 40";

        if (!message.empty()) {
            ExampleMessage _exampleMsg = ParseMessage(message);
            std::vector<uint8_t> _sendMsg = _exampleMsg.serialize();

            if (send(sock, _sendMsg.data(), _sendMsg.size(), 0) < 0) {
                std::cerr << "Send error" << std::endl;
                break;
            }

        }
    }
}

ExampleMessage TestNode::ParseMessage(const std::string& input) {
    std::istringstream stream(input);
    uint32_t id;
    float val;
    std::string name;
    std::vector<int32_t> nums;

    // 문자열을 파싱하여 필드에 저장
    stream >> id >> val;
    stream >> name;  // 단일 단어 기준
    int32_t num;
    while (stream >> num) {  // 남은 숫자들을 벡터에 저장
        nums.push_back(num);
    }

    return ExampleMessage(id, val, name, nums);
}


