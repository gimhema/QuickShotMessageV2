#pragma once
#include <sys/socket.h>
#include <arpa/inet.h>
#include <string>
#include <cstring>
#include <unistd.h>

class TestNode
{
public:
    TestNode();
    ~TestNode();

public:
    void Init(std::string _ip_address, int _port);
    int Run();
    void SendMessage();

public:
    std::string ip_address = "";
    int port = 0;
    int sock = 0;
};



















