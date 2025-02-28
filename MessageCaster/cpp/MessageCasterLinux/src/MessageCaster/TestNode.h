#pragma once
#include <sys/socket.h>
#include <arpa/inet.h>
#include <string>
#include <cstring>

class TestNode
{
public:
    TestNode();
    ~TestNode();

public:
    std::string ip_address = "";
    int port = 0;
};