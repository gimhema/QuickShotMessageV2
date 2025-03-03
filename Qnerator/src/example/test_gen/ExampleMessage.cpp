#pragma pack(push, 1)
struct ExampleMessage {
    uint32_t id;
    float val;
    std::string name;
    std::vector<int32_t> nums;

    ExampleMessage(uint32_t id, float val, std::string name, std::vector<int32_t> nums) : id(id), val(val), name(name), nums(nums) {}

    std::vector<uint8_t> serialize() const {
        std::vector<uint8_t> buffer;
        buffer.insert(buffer.end(), reinterpret_cast<const uint8_t*>(&id), reinterpret_cast<const uint8_t*>(&id + 1));
        buffer.insert(buffer.end(), reinterpret_cast<const uint8_t*>(&val), reinterpret_cast<const uint8_t*>(&val + 1));
        uint32_t name_length = name.size();
        buffer.insert(buffer.end(), reinterpret_cast<const uint8_t*>(&name_length), reinterpret_cast<const uint8_t*>(&name_length + 1));
        buffer.insert(buffer.end(), name.begin(), name.end());
        uint32_t nums_size = nums.size();
        buffer.insert(buffer.end(), reinterpret_cast<const uint8_t*>(&nums_size), reinterpret_cast<const uint8_t*>(&nums_size + 1));
        buffer.insert(buffer.end(), reinterpret_cast<const uint8_t*>(nums.data()), reinterpret_cast<const uint8_t*>(nums.data() + nums_size));
        return buffer;
    }

    static ExampleMessage deserialize(const std::vector<uint8_t>& buffer) {
        size_t offset = 0;
        uint32_t id = *reinterpret_cast<const uint32_t*>(buffer.data() + offset);
        offset += sizeof(uint32_t);
        float val = *reinterpret_cast<const float*>(buffer.data() + offset);
        offset += sizeof(float);
        uint32_t name_length = *reinterpret_cast<const uint32_t*>(buffer.data() + offset);
        offset += sizeof(uint32_t);
        std::string name(buffer.begin() + offset, buffer.begin() + offset + name_length);
        offset += name_length;
        uint32_t nums_size = *reinterpret_cast<const uint32_t*>(buffer.data() + offset);
        offset += sizeof(uint32_t);
        std::vector<int32_t> nums(nums_size);
        std::memcpy(nums.data(), buffer.data() + offset, nums_size * sizeof(int32_t));
        offset += nums_size * sizeof(int32_t);
        return ExampleMessage(id, val, name, nums);
    }
};
#pragma pack(pop)
