// 자동 생성된 구조체 및 관련 메서드
        #[repr(C)]
        #[derive(Debug, Clone)]
        pub struct Hello {
            message: String,
    message_length: u32,}
    
        impl Hello {
            pub fn new(message: String) -> Self {
                Self {
                    message,        }
            }
    
            pub fn serialize(&self) -> Vec<u8> {
                let mut buffer = Vec::new();
                buffer.extend(&self.message.len().to_le_bytes());
buffer.extend(self.message.as_bytes());
                buffer
            }
    
            pub fn deserialize(buffer: &[u8]) -> io::Result<Self> {
                let mut offset = 0;
                let mut message_length_bytes = [0u8; 4];
message_length_bytes.copy_from_slice(&buffer[offset..offset + 4]);
let message_length = u32::from_le_bytes(message_length_bytes);
offset += 4;
let message = String::from_utf8(buffer[offset..offset + message_length as usize].to_vec())
.map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Invalid UTF-8 string"))?;
offset += message_length as usize;
                Ok(Self {
                                message,
                })
            }
        }