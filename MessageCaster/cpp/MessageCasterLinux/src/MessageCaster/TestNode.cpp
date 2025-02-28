#include "TestNode.h"
#include "MessageCaster.h"


TestNode::TestNode()
{

}

TestNode::~TestNode()
{

}

void TestNode::Init(std::string _ip_address, int _port)
{
    this->ip_address = _ip_address;
    this->port = _port;
}

int TestNode::Run()
{
    int sock = 0;
    struct sockaddr_in node_addr;

    MessageCaster* m_caster = new MessageCaster();

    if ((sock = socket(AF_INET, SOCK_STREAM, 0)) < 0) {
        std::cerr << "Socket creation error" << std::endl;
        return -1;
    }

    node_addr.sin_family = AF_INET;
    node_addr.sin_port = htons(this->port);

    if (inet_pton(AF_INET, this->ip_address.c_str(), &node_addr.sin_addr) <= 0) {
        std::cerr << "Invalid address/ Address not supported" << std::endl;
        return -1;
    }

    if (connect(sock, (struct sockaddr*)&node_addr, sizeof(node_addr)) < 0) {
        std::cerr << "Connection Failed" << std::endl;
        return -1;
    }

    std::vector<uint8_t> buffer(2048); // 충분히 큰 버퍼 준비
    int bytes_received = recv(sock, buffer.data(), buffer.size(), 0);

    if (bytes_received > 0) {
        buffer.resize(bytes_received); // 실제 수신된 데이터만큼 버퍼 크기 조정
        m_caster->HandleMessage(buffer);
    } else if (bytes_received == 0) {
        std::cout << "Connection closed by server" << std::endl;
    } else {
        std::cerr << "Receive error" << std::endl;
    }

    close(sock); // 소켓 종료
    
    return 0;
}

void TestNode::SendMessage()
{
    //ExampleMessage packed_data(2, "Hello Message", {5, 7, 9, 47});
    //    // PackedData packed_data(1, 123456789);
    //    std::vector<uint8_t> serialized_data = packed_data.serialize();
    //    send(sock, serialized_data.data(), serialized_data.size(), 0);
    //    std::cout << "PackedData message sent to server" << std::endl;

}




