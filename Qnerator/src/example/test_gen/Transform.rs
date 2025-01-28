// 자동 생성된 구조체 및 관련 메서드
        #[repr(C)]
        #[derive(Debug, Clone)]
        pub struct Transform {
            x: f32,
    y: f32,
    z: f32,
    roll: f32,
    pitch: f32,
    yaw: f32,}
    
        impl Transform {
            pub fn new(x: f32, y: f32, z: f32, roll: f32, pitch: f32, yaw: f32) -> Self {
                Self {
                    x,
            y,
            z,
            roll,
            pitch,
            yaw,        }
            }
    
            pub fn serialize(&self) -> Vec<u8> {
                let mut buffer = Vec::new();
                buffer.extend(&self.x.to_le_bytes());
        buffer.extend(&self.y.to_le_bytes());
        buffer.extend(&self.z.to_le_bytes());
        buffer.extend(&self.roll.to_le_bytes());
        buffer.extend(&self.pitch.to_le_bytes());
        buffer.extend(&self.yaw.to_le_bytes());
                buffer
            }
    
            pub fn deserialize(buffer: &[u8]) -> io::Result<Self> {
                let mut offset = 0;
                let mut x_bytes = [0u8; 4];
x_bytes.copy_from_slice(&buffer[offset..offset + 4]);
let x = f32::from_le_bytes(x_bytes);
offset += 4;
        let mut y_bytes = [0u8; 4];
y_bytes.copy_from_slice(&buffer[offset..offset + 4]);
let y = f32::from_le_bytes(y_bytes);
offset += 4;
        let mut z_bytes = [0u8; 4];
z_bytes.copy_from_slice(&buffer[offset..offset + 4]);
let z = f32::from_le_bytes(z_bytes);
offset += 4;
        let mut roll_bytes = [0u8; 4];
roll_bytes.copy_from_slice(&buffer[offset..offset + 4]);
let roll = f32::from_le_bytes(roll_bytes);
offset += 4;
        let mut pitch_bytes = [0u8; 4];
pitch_bytes.copy_from_slice(&buffer[offset..offset + 4]);
let pitch = f32::from_le_bytes(pitch_bytes);
offset += 4;
        let mut yaw_bytes = [0u8; 4];
yaw_bytes.copy_from_slice(&buffer[offset..offset + 4]);
let yaw = f32::from_le_bytes(yaw_bytes);
offset += 4;
                Ok(Self {
                                x,
            y,
            z,
            roll,
            pitch,
            yaw,
                })
            }
        }