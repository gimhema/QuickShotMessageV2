#pragma once
#include <winsock2.h>
#include <ws2tcpip.h>
#include <string>
#include <iostream>
#include "../Messages/ExampleMessage.hpp"

// 링커 설정 필요: ws2_32.lib
#pragma comment(lib, "ws2_32.lib")

class TestNode
{
public:
    TestNode();
    ~TestNode();

public:
    void Init(const std::string& _ip_address, int _port);
    int Run();

private:
    void SendMessage();
    ExampleMessage ParseMessage(const std::string& input);

public:
    std::string ip_address = "";
    int port = 0;
    SOCKET sock = INVALID_SOCKET;
};