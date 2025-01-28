// g++ -o qsm src/main.cpp

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

// PackedData 구조체 정의
#pragma pack(push, 1)
struct PackedData {
    uint32_t id;    // 4 bytes
    uint32_t size;  // 4 bytes
    uint64_t value; // 8 bytes

    PackedData(uint32_t id, uint64_t value) 
        : id(id), value(value) {
        size = sizeof(id) + sizeof(size) + sizeof(value); // 동적으로 size 설정
    }

    // PackedData 직렬화
    std::vector<uint8_t> serialize() {
        size = sizeof(id) + sizeof(size) + sizeof(value); // 직렬화 전에 size 갱신
        std::vector<uint8_t> buffer(size); // 버퍼 크기를 size로 설정
        std::memcpy(buffer.data(), &id, sizeof(id));
        std::memcpy(buffer.data() + sizeof(id), &size, sizeof(size));
        std::memcpy(buffer.data() + sizeof(id) + sizeof(size), &value, sizeof(value));
        return buffer;
    }

    // 바이너리 데이터를 역직렬화하여 PackedData 생성
    static PackedData deserialize(const std::vector<uint8_t>& buffer) {
        if (buffer.size() < sizeof(PackedData)) {
            throw std::runtime_error("Buffer too short");
        }
        uint32_t id, size;
        uint64_t value;
        std::memcpy(&id, buffer.data(), sizeof(id));
        std::memcpy(&size, buffer.data() + sizeof(id), sizeof(size));
        std::memcpy(&value, buffer.data() + sizeof(id) + sizeof(size), sizeof(value));

        // if (size != buffer.size()) {
        //     throw std::runtime_error("Size mismatch");
        // }
        return PackedData(id, value);
    }

    // PackedData 구조체를 출력하는 함수
    friend std::ostream& operator<<(std::ostream& os, const PackedData& data) {
        os << "PackedData { id: " << data.id
           << ", size: " << data.size
           << ", value: " << data.value << " }";
        return os;
    }
};
#pragma pack(pop)



struct ExampleMessage {
    uint32_t id;            // 메시지 타입
    uint32_t string_length;  // 문자열 길이
    uint32_t array_length;   // 배열 길이
    std::string text;        // 문자열
    std::vector<int32_t> numbers; // 정수 배열

    ExampleMessage(uint32_t id, const std::string& text, const std::vector<int32_t>& numbers) 
        : id(id), string_length(text.size()), array_length(numbers.size()), text(text), numbers(numbers) {}

    // 직렬화 함수
    std::vector<uint8_t> serialize() const {
        std::vector<uint8_t> buffer(sizeof(id) + sizeof(string_length) + string_length + sizeof(array_length) + array_length * sizeof(int32_t));

        size_t offset = 0;

        // id 복사
        std::memcpy(buffer.data() + offset, &id, sizeof(id));
        offset += sizeof(id);

        // 문자열 길이 복사
        std::memcpy(buffer.data() + offset, &string_length, sizeof(string_length));
        offset += sizeof(string_length);

        // 문자열 복사
        std::memcpy(buffer.data() + offset, text.c_str(), string_length);
        offset += string_length;

        // 배열 길이 복사
        std::memcpy(buffer.data() + offset, &array_length, sizeof(array_length));
        offset += sizeof(array_length);

        // 배열 값 복사
        std::memcpy(buffer.data() + offset, numbers.data(), array_length * sizeof(int32_t));

        return buffer;
    }

    // 역직렬화 함수
    static ExampleMessage deserialize(const std::vector<uint8_t>& buffer) {
        size_t offset = 0;

        // id 복사
        uint32_t id;
        std::memcpy(&id, buffer.data() + offset, sizeof(id));
        offset += sizeof(id);

        // 문자열 길이 복사
        uint32_t string_length;
        std::memcpy(&string_length, buffer.data() + offset, sizeof(string_length));
        offset += sizeof(string_length);

        // 문자열 복사
        std::string text(buffer.begin() + offset, buffer.begin() + offset + string_length);
        offset += string_length;

        // 배열 길이 복사
        uint32_t array_length;
        std::memcpy(&array_length, buffer.data() + offset, sizeof(array_length));
        offset += sizeof(array_length);

        // 배열 값 복사
        std::vector<int32_t> numbers(array_length);
        std::memcpy(numbers.data(), buffer.data() + offset, array_length * sizeof(int32_t));

        return ExampleMessage(id, text, numbers);
    }

    // ExampleMessage 출력
    friend std::ostream& operator<<(std::ostream& os, const ExampleMessage& msg) {
        os << "ExampleMessage { id: " << msg.id
           << ", text: " << msg.text
           << ", numbers: [";
        for (size_t i = 0; i < msg.numbers.size(); ++i) {
            os << msg.numbers[i];
            if (i < msg.numbers.size() - 1) os << ", ";
        }
        os << "] }";
        return os;
    }
};


#pragma pack(pop)

// 메시지 처리 함수
void handle_message(const std::vector<uint8_t>& buffer) {
    // BaseMessage의 ID 확인
    BaseMessage base_message = BaseMessage::deserialize(buffer);

    switch (base_message.id) {
        case 0: {
            // id가 0이면 Invalid 출력
            std::cout << "Invalid message id: 0\n";
            break;
        }
        case 1: {
            // id가 1이면 PackedData의 값을 출력
            std::cout << "Case 1 " << std::endl;
            PackedData packed_data = PackedData::deserialize(buffer);
            std::cout << packed_data << std::endl;
            break;
        }
        case 2: {
            std::cout << "Case 2 " << std::endl;
            ExampleMessage example_data = ExampleMessage::deserialize(buffer);
            std::cout << example_data << std::endl;
            break;
        }
        default: {
            std::cout << "Unknown message id: " << base_message.id << "\n";
            break;
        }
    }
}


