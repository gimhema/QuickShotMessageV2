#pragma once
#include "TestNode.h"
#include "MessageCaster.h"
#include <iostream>
#include <vector>
#include <thread>
#include <cstring>
#include <memory>
#include <sstream>
#include <winsock2.h>
#include <ws2tcpip.h>

// 링커 설정 필요: ws2_32.lib
#pragma comment(lib, "ws2_32.lib")

TestNode::TestNode() : sock(INVALID_SOCKET)
{
    WSADATA wsaData;
    if (WSAStartup(MAKEWORD(2, 2), &wsaData) != 0) {
        std::cerr << "WSAStartup failed\n";
    }
}

TestNode::~TestNode()
{
    if (sock != INVALID_SOCKET) {
        closesocket(sock);
    }
    WSACleanup();
}

void TestNode::Init(const std::string& _ip_address, int _port)
{
    this->ip_address = _ip_address;
    this->port = _port;
}

int TestNode::Run()
{
    struct sockaddr_in node_addr;
    std::unique_ptr<MessageCaster> m_caster = std::make_unique<MessageCaster>();

    if ((sock = socket(AF_INET, SOCK_STREAM, 0)) == INVALID_SOCKET) {
        std::cerr << "Socket creation error\n";
        return -1;
    }

    node_addr.sin_family = AF_INET;
    node_addr.sin_port = htons(this->port);

    if (inet_pton(AF_INET, this->ip_address.c_str(), &node_addr.sin_addr) <= 0) {
        std::cerr << "Invalid address / Address not supported\n";
        closesocket(sock);
        return -1;
    }

    if (connect(sock, (struct sockaddr*)&node_addr, sizeof(node_addr)) == SOCKET_ERROR) {
        std::cerr << "Connection Failed\n";
        closesocket(sock);
        return -1;
    }

    std::thread send_thread(&TestNode::SendMessage, this);

    std::vector<uint8_t> buffer(2048);

    while (true) {
        int bytes_received = recv(sock, reinterpret_cast<char*>(buffer.data()), static_cast<int>(buffer.size()), 0);

        if (bytes_received > 0) {
            buffer.resize(bytes_received);
            m_caster->HandleMessage(buffer);
        }
        else if (bytes_received == 0) {
            std::cout << "Connection closed by server\n";
            break;
        }
        else {
            std::cerr << "Receive error: " << WSAGetLastError() << "\n";
            break;
        }
    }

    if (send_thread.joinable()) {
        send_thread.join();
    }

    closesocket(sock);
    return 0;
}

void TestNode::SendMessage()
{
    while (true) {
        std::string message;
        std::cout << "Enter message to send (or type 'exit' to quit): ";
        std::getline(std::cin, message);

        if (message == "exit") {
            std::cout << "Closing connection...\n";
            closesocket(sock);
            break;
        }

        if (!message.empty()) {
            ExampleMessage _exampleMsg = ParseMessage(message);
            std::vector<uint8_t> _sendMsg = _exampleMsg.serialize();

            if (send(sock, reinterpret_cast<const char*>(_sendMsg.data()), static_cast<int>(_sendMsg.size()), 0) == SOCKET_ERROR) {
                std::cerr << "Send error: " << WSAGetLastError() << "\n";
                break;
            }
        }
    }
}

ExampleMessage TestNode::ParseMessage(const std::string& input)
{
    std::istringstream stream(input);
    uint32_t id;
    float val;
    std::string name;
    std::vector<int32_t> nums;

    stream >> id >> val;
    stream >> name;
    int32_t num;
    while (stream >> num) {
        nums.push_back(num);
    }

    return ExampleMessage(id, val, name, nums);
}
