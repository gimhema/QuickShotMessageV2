#pragma once

#include <iostream>
#include <vector>
#include <cstring>  // memcpy
#include <cstdint>  // uint32_t, uint64_t
#include <stdexcept> // std::runtime_error



// BaseMessage 구조체 정의
#pragma pack(push, 1)
struct BaseMessage {
    uint32_t id; // 메시지 타입을 나타냄

    BaseMessage(uint32_t id) : id(id) {}

    // 메시지 직렬화
    std::vector<uint8_t> serialize() const {
        std::vector<uint8_t> buffer(sizeof(BaseMessage));
        std::memcpy(buffer.data(), &id, sizeof(id));
        return buffer;
    }
    // std::vector<uint8_t> serialize() const {
    //     std::vector<uint8_t> buffer(sizeof(BaseMessage));
    //     std::memcpy(buffer.data(), &id, sizeof(id)); // id 값을 리틀 엔디안으로 직렬화
    //     return buffer;
    // }

    // 바이너리 데이터를 역직렬화하여 BaseMessage 생성
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
    // 
public:


public:
    MessageCaster();
    ~MessageCaster();

};


