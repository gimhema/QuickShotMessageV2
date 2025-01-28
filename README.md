# Overview

QuickShotMessage generator

## Detail

### Message File Format (for generate)

<MESSAGE_FILE_MANE>.qsmb
```
msg <MESSAGE_NAME>
{
  <TYPE_NAME> <VALUE_NAME>
}

```

### Example

ExampleMessage.qsmb
```
msg ExampleMessage
{
    Integer id
    Long timestamp
    Float val
    String name
    ArrayInteger nums
    ArrayFloat vals
}
```

### Generate Output Result


#### cpp

ExampleMessage.hpp
```
#pragma pack(push, 1)
struct ExampleMessage {
    uint32_t id;
    uint64_t timestamp;
    float val;
    std::string name;
    std::vector<int32_t> nums;
    std::vector<float> vals;

    ExampleMessage(uint32_t id, uint64_t timestamp, float val, std::string name, std::vector<int32_t> nums, std::vector<float> vals) : id(id), timestamp(timestamp), val(val), name(name), nums(nums), vals(vals) {}

    std::vector<uint8_t> serialize() const {
        std::vector<uint8_t> buffer;
        buffer.insert(buffer.end(), reinterpret_cast<const uint8_t*>(&id), reinterpret_cast<const uint8_t*>(&id + 1));
        buffer.insert(buffer.end(), reinterpret_cast<const uint8_t*>(&timestamp), reinterpret_cast<const uint8_t*>(&timestamp + 1));
        buffer.insert(buffer.end(), reinterpret_cast<const uint8_t*>(&val), reinterpret_cast<const uint8_t*>(&val + 1));
        uint32_t name_length = name.size();
        buffer.insert(buffer.end(), reinterpret_cast<const uint8_t*>(&name_length), reinterpret_cast<const uint8_t*>(&name_length + 1));
        buffer.insert(buffer.end(), name.begin(), name.end());
        uint32_t nums_size = nums.size();
        buffer.insert(buffer.end(), reinterpret_cast<const uint8_t*>(&nums_size), reinterpret_cast<const uint8_t*>(&nums_size + 1));
        buffer.insert(buffer.end(), reinterpret_cast<const uint8_t*>(nums.data()), reinterpret_cast<const uint8_t*>(nums.data() + nums_size));
        uint32_t vals_size = vals.size();
        buffer.insert(buffer.end(), reinterpret_cast<const uint8_t*>(&vals_size), reinterpret_cast<const uint8_t*>(&vals_size + 1));
        buffer.insert(buffer.end(), reinterpret_cast<const uint8_t*>(vals.data()), reinterpret_cast<const uint8_t*>(vals.data() + vals_size));
        return buffer;
    }

    static ExampleMessage deserialize(const std::vector<uint8_t>& buffer) {
        size_t offset = 0;
        uint32_t id = *reinterpret_cast<const uint32_t*>(buffer.data() + offset);
        offset += sizeof(uint32_t);
        uint64_t timestamp = *reinterpret_cast<const uint64_t*>(buffer.data() + offset);
        offset += sizeof(uint64_t);
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
        uint32_t vals_size = *reinterpret_cast<const uint32_t*>(buffer.data() + offset);
        offset += sizeof(uint32_t);
        std::vector<float> vals(vals_size);
        std::memcpy(vals.data(), buffer.data() + offset, vals_size * sizeof(float));
        offset += vals_size * sizeof(float);
        return ExampleMessage(id, timestamp, val, name, nums, vals);
    }
};
#pragma pack(pop)
```

#### Rust
QS_ExampleMessage.rs
```
        #[repr(C)]
        #[derive(Debug, Clone)]
        pub struct ExampleMessage {
            id: u32,
    val: f32,
    name: String,
    name_length: u32,
    nums: Vec<i32>,
    nums_length: u32,}
    
        impl ExampleMessage {
            pub fn new(id: u32, val: f32, name: String, nums: Vec<i32>) -> Self {
                Self {
                    id,
            val,
            name,
            nums,        }
            }
    
            pub fn serialize(&self) -> Vec<u8> {
                let mut buffer = Vec::new();
                buffer.extend(&self.id.to_le_bytes());
        buffer.extend(&self.val.to_le_bytes());
        buffer.extend(&self.name.len().to_le_bytes());
buffer.extend(self.name.as_bytes());
        buffer.extend(&self.nums.len().to_le_bytes());
for num in &self.nums {
buffer.extend(&num.to_le_bytes());
}
                buffer
            }
    
            pub fn deserialize(buffer: &[u8]) -> io::Result<Self> {
                let mut offset = 0;
                let mut id_bytes = [0u8; 4];
id_bytes.copy_from_slice(&buffer[offset..offset + 4]);
let id = u32::from_le_bytes(id_bytes);
offset += 4;
        let mut val_bytes = [0u8; 4];
val_bytes.copy_from_slice(&buffer[offset..offset + 4]);
let val = f32::from_le_bytes(val_bytes);
offset += 4;
        let mut name_length_bytes = [0u8; 4];
name_length_bytes.copy_from_slice(&buffer[offset..offset + 4]);
let name_length = u32::from_le_bytes(name_length_bytes);
offset += 4;
let name = String::from_utf8(buffer[offset..offset + name_length as usize].to_vec())
.map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Invalid UTF-8 string"))?;
offset += name_length as usize;
        let mut nums_length_bytes = [0u8; 4];
nums_length_bytes.copy_from_slice(&buffer[offset..offset + 4]);
let nums_length = u32::from_le_bytes(nums_length_bytes);
offset += 4;
let mut nums = Vec::new();
for _ in 0..nums_length {
let mut num_bytes = [0u8; 4];
num_bytes.copy_from_slice(&buffer[offset..offset + 4]);
let num = i32::from_le_bytes(num_bytes);
nums.push(num);
offset += 4;
}
                Ok(Self {
                                id,
            val,
            name,
            nums,
                })
            }
        }
```


## Command Usage

qnerator -d <TARGET_FILE_DIRECTORY> <GENERATE_LANGUAGE> <GENERATE_DIRECTORY>

### Example

Example 1: qnerator -d /targets rust - => will generate /gen directory

Example 2: qnerator -d /targets rust /custom => will generate /custom directory











