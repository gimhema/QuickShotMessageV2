use std::io::{self, Cursor, Read, Write};


pub fn handle_message(buffer: &[u8]) {
    // BaseMessage의 ID 확인
    let base_message = BaseMessage::deserialize(buffer).unwrap();

    let base_message_id = base_message.id; // id를 복사

    match base_message_id {
        0 => {
            // id가 0이면 Invalid 출력
            println!("Invalid message id: 0");
        }
        1 => {
            // id가 1이면 PackedData의 값을 출력
            println!("Check Packed Message");
            let packed_data = PackedData::deserialize(buffer).unwrap();
            println!("PackedData received: {:?}", packed_data);
        }
        2 => {
            println!("Check ExampleMessage");
            let example_message = ExampleMessage::deserialize(buffer).unwrap();
            println!("ExampleMessage received: {:?}", example_message);
        }
        _ => {
            println!("Unknown message id: {}", base_message_id);
        }
    }
}


#[repr(packed)]
pub struct BaseMessage {
    id: u32,   // 메시지 타입을 나타냄
}

impl BaseMessage {
    // 새로운 BaseMessage 생성
    pub fn new(id: u32) -> Self {
        BaseMessage { id }
    }

    // 메시지의 바이너리 직렬화
    pub fn serialize(&self) -> Vec<u8> {
        let mut buffer = Vec::with_capacity(std::mem::size_of::<BaseMessage>());
        buffer.extend(&self.id.to_le_bytes()); // id 값을 리틀 엔디안으로 직렬화
        buffer
    }

    pub fn deserialize(buffer: &[u8]) -> Result<Self, &'static str> {
        if buffer.len() < 4 {
            return Err("Buffer too short");
        }
        let id = u32::from_le_bytes([buffer[0], buffer[1], buffer[2], buffer[3]]);
        Ok(BaseMessage { id })
    }
}


// 데이터 패킹을 위한 구조체 정의
#[repr(packed)]
#[derive(Debug, Clone)]
pub struct PackedData {
    id: u32,       // 4 bytes
    size: u32,     // 4 bytes
    value: u64,    // 8 bytes
}

impl PackedData {
    pub fn new(id: u32, value: u64) -> Self {
        let size = std::mem::size_of::<PackedData>() as u32; // 구조체의 크기
        PackedData { id, size, value }
    }

    // 바이너리 직렬화
    pub fn serialize(&self) -> Vec<u8> {
        let mut buffer = Vec::with_capacity(self.size as usize);
        let mut cursor = Cursor::new(&mut buffer);
        cursor.write_all(&self.id.to_le_bytes()).unwrap();
        cursor.write_all(&self.size.to_le_bytes()).unwrap();
        cursor.write_all(&self.value.to_le_bytes()).unwrap();
        buffer
    }


    pub fn deserialize(buffer: &[u8]) -> std::io::Result<Self> {
        if buffer.len() < std::mem::size_of::<PackedData>() {
            return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "Buffer too short"));
        }
        
        let id = u32::from_le_bytes([buffer[0], buffer[1], buffer[2], buffer[3]]);
        let size = u32::from_le_bytes([buffer[4], buffer[5], buffer[6], buffer[7]]);
        let value = u64::from_le_bytes([buffer[8], buffer[9], buffer[10], buffer[11], buffer[12], buffer[13], buffer[14], buffer[15]]);

        Ok(PackedData { id, size, value })
    }

}

// ExampleMessage 구조체 정의 (문자열 및 정수 배열 포함)
#[repr(C)]  // 안전한 메모리 레이아웃 보장
#[derive(Debug, Clone)]
pub struct ExampleMessage {
    id: u32,            // 메시지 타입
    string_length: u32,  // 문자열 길이
    array_length: u32,   // 배열 길이
    text: String,        // 문자열
    numbers: Vec<i32>,   // 정수 배열
}

impl ExampleMessage {
    pub fn new(id: u32, text: String, numbers: Vec<i32>) -> Self {
        Self {
            id,
            string_length: text.len() as u32,
            array_length: numbers.len() as u32,
            text,
            numbers,
        }
    }

    // 직렬화
    pub fn serialize(&self) -> Vec<u8> {
        let mut buffer = Vec::new();

        // id 직렬화 (u32)
        buffer.extend(&self.id.to_le_bytes());

        // string_length 직렬화 (u32)
        buffer.extend(&self.string_length.to_le_bytes());

        // 문자열 직렬화
        buffer.extend(self.text.as_bytes());

        // array_length 직렬화 (u32)
        buffer.extend(&self.array_length.to_le_bytes());

        // 정수 배열 직렬화 (i32 배열)
        for num in &self.numbers {
            buffer.extend(&num.to_le_bytes());
        }

        buffer
    }

    // 역직렬화
    pub fn deserialize(buffer: &[u8]) -> io::Result<Self> {
        let mut offset = 0;

        // id 역직렬화
        let mut id_bytes = [0u8; 4];
        id_bytes.copy_from_slice(&buffer[offset..offset + 4]);
        let id = u32::from_le_bytes(id_bytes);
        offset += 4;

        // string_length 역직렬화
        let mut string_length_bytes = [0u8; 4];
        string_length_bytes.copy_from_slice(&buffer[offset..offset + 4]);
        let string_length = u32::from_le_bytes(string_length_bytes);
        offset += 4;

        // 문자열 역직렬화
        let text = String::from_utf8(buffer[offset..offset + string_length as usize].to_vec())
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Invalid UTF-8 string"))?;
        offset += string_length as usize;

        // array_length 역직렬화
        let mut array_length_bytes = [0u8; 4];
        array_length_bytes.copy_from_slice(&buffer[offset..offset + 4]);
        let array_length = u32::from_le_bytes(array_length_bytes);
        offset += 4;

        // 배열 역직렬화
        let mut numbers = Vec::new();
        for _ in 0..array_length {
            let mut num_bytes = [0u8; 4];
            num_bytes.copy_from_slice(&buffer[offset..offset + 4]);
            let num = i32::from_le_bytes(num_bytes);
            numbers.push(num);
            offset += 4;
        }

        Ok(ExampleMessage {
            id,
            string_length,
            array_length,
            text,
            numbers,
        })
    }
}
