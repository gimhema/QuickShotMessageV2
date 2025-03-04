use std::io;

// 자동 생성된 구조체 및 관련 메서드
#[repr(C)]
#[derive(Debug, Clone)]
pub struct ExampleMessage {
    pub id: u32,
    pub val: f32,
    pub name: String,
    pub nums: Vec<i32>,}

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