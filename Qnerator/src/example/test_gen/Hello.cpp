#pragma pack(push, 1)
struct Hello {
    std::string message;

    Hello(std::string message) : message(message) {}

    std::vector<uint8_t> serialize() const {
        std::vector<uint8_t> buffer;
        uint32_t message_length = message.size();
        buffer.insert(buffer.end(), reinterpret_cast<const uint8_t*>(&message_length), reinterpret_cast<const uint8_t*>(&message_length + 1));
        buffer.insert(buffer.end(), message.begin(), message.end());
        return buffer;
    }

    static Hello deserialize(const std::vector<uint8_t>& buffer) {
        size_t offset = 0;
        uint32_t message_length = *reinterpret_cast<const uint32_t*>(buffer.data() + offset);
        offset += sizeof(uint32_t);
        std::string message(buffer.begin() + offset, buffer.begin() + offset + message_length);
        offset += message_length;
        return Hello(message);
    }
};
#pragma pack(pop)
