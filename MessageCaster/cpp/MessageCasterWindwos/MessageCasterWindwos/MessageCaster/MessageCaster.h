#pragma once

#include <iostream>
#include <vector>
#include <cstring>  // memcpy
#include <cstdint>  // uint32_t, uint64_t
#include <stdexcept> // std::runtime_error


#pragma pack(push, 1)
struct BaseMessage {
    uint32_t id;

    BaseMessage(uint32_t id) : id(id) {}

    std::vector<uint8_t> serialize() const {
        std::vector<uint8_t> buffer(sizeof(BaseMessage));
        std::memcpy(buffer.data(), &id, sizeof(id));
        return buffer;
    }

    static BaseMessage deserialize(const std::vector<uint8_t>& buffer) {
        if (buffer.size() < sizeof(uint32_t)) {
            throw std::runtime_error("Buffer too short");
        }
        uint32_t id;
        std::memcpy(&id, buffer.data(), sizeof(uint32_t));
        return BaseMessage(id);
    }
};
#pragma pack(pop)

class MessageCaster
{
public:
    MessageCaster();
    ~MessageCaster();

private:
    int MAX_BUUFER_SIZE = 2048;


public:
    void RecvPostProcess();
    void SendPreProcess();
    void HandleMessage(const std::vector<uint8_t>& buffer);

};

