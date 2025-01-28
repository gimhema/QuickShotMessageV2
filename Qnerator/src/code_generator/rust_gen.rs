
use super::CodeGenerator;
use super::gen_trait::*;

use std::fs::File;
use std::io::{BufRead, BufReader};

use super::code_gen_option::*;


pub struct RustGenerator {

}

impl RustGenerator {
    pub fn new() -> Self {
        return RustGenerator{}
    }

    pub fn format_rust_code(&mut self, file_name: String, fields: Vec<(String, String)>) -> String {
        // 구조체 이름 생성 (파일 이름에서 확장자를 제거하고 PascalCase로 변환)
        let struct_name = file_name
            .split('.')
            .next()
            .unwrap_or("GeneratedStruct")
            .to_string();
    
        let mut struct_fields = String::new();
        let mut constructor_params = String::new();
        let mut constructor_assignments = String::new();
        let mut serialization_code = String::new();
        let mut deserialization_code = String::new();
    
        for (field_type, field_name) in fields {
            match field_type.as_str() {
                "Integer" => {
                    struct_fields.push_str(&format!("    {}: u32,\n", field_name));
                    constructor_params.push_str(&format!("{}: u32, ", field_name));
                    constructor_assignments.push_str(&format!("            {},\n", field_name));
                    serialization_code.push_str(&format!(
                        "        buffer.extend(&self.{}.to_le_bytes());\n",
                        field_name
                    ));
                    deserialization_code.push_str(&format!(
                        "        let mut {0}_bytes = [0u8; 4];\n\
                         {0}_bytes.copy_from_slice(&buffer[offset..offset + 4]);\n\
                         let {0} = u32::from_le_bytes({0}_bytes);\n\
                         offset += 4;\n",
                        field_name
                    ));
                }
                "Float" => {
                    struct_fields.push_str(&format!("    {}: f32,\n", field_name));
                    constructor_params.push_str(&format!("{}: f32, ", field_name));
                    constructor_assignments.push_str(&format!("            {},\n", field_name));
                    serialization_code.push_str(&format!(
                        "        buffer.extend(&self.{}.to_le_bytes());\n",
                        field_name
                    ));
                    deserialization_code.push_str(&format!(
                        "        let mut {0}_bytes = [0u8; 4];\n\
                         {0}_bytes.copy_from_slice(&buffer[offset..offset + 4]);\n\
                         let {0} = f32::from_le_bytes({0}_bytes);\n\
                         offset += 4;\n",
                        field_name
                    ));
                }
                "String" => {
                    struct_fields.push_str(&format!("    {}: String,\n", field_name));
                    struct_fields.push_str(&format!("    {}_length: u32,\n", field_name));
                    constructor_params.push_str(&format!("{}: String, ", field_name));
                    constructor_assignments.push_str(&format!("            {},\n", field_name));
                    serialization_code.push_str(&format!(
                        "        buffer.extend(&self.{0}.len().to_le_bytes());\n\
                         buffer.extend(self.{0}.as_bytes());\n",
                        field_name
                    ));
                    deserialization_code.push_str(&format!(
                        "        let mut {0}_length_bytes = [0u8; 4];\n\
                         {0}_length_bytes.copy_from_slice(&buffer[offset..offset + 4]);\n\
                         let {0}_length = u32::from_le_bytes({0}_length_bytes);\n\
                         offset += 4;\n\
                         let {0} = String::from_utf8(buffer[offset..offset + {0}_length as usize].to_vec())\n\
                             .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, \"Invalid UTF-8 string\"))?;\n\
                         offset += {0}_length as usize;\n",
                        field_name
                    ));
                }
                "ArrayInteger" => {
                    struct_fields.push_str(&format!("    {}: Vec<i32>,\n", field_name));
                    struct_fields.push_str(&format!("    {}_length: u32,\n", field_name));
                    constructor_params.push_str(&format!("{}: Vec<i32>, ", field_name));
                    constructor_assignments.push_str(&format!("            {},\n", field_name));
                    serialization_code.push_str(&format!(
                        "        buffer.extend(&self.{}.len().to_le_bytes());\n\
                         for num in &self.{} {{\n\
                             buffer.extend(&num.to_le_bytes());\n\
                         }}\n",
                        field_name, field_name
                    ));
                    deserialization_code.push_str(&format!(
                        "        let mut {0}_length_bytes = [0u8; 4];\n\
                         {0}_length_bytes.copy_from_slice(&buffer[offset..offset + 4]);\n\
                         let {0}_length = u32::from_le_bytes({0}_length_bytes);\n\
                         offset += 4;\n\
                         let mut {0} = Vec::new();\n\
                         for _ in 0..{0}_length {{\n\
                             let mut num_bytes = [0u8; 4];\n\
                             num_bytes.copy_from_slice(&buffer[offset..offset + 4]);\n\
                             let num = i32::from_le_bytes(num_bytes);\n\
                             {0}.push(num);\n\
                             offset += 4;\n\
                         }}\n",
                        field_name
                    ));
                }
                _ => {}
            }
        }
    
        // 최종 Rust 코드 생성
        format!(
            "// 자동 생성된 구조체 및 관련 메서드
        #[repr(C)]
        #[derive(Debug, Clone)]
        pub struct {struct_name} {{
        {struct_fields}}}
    
        impl {struct_name} {{
            pub fn new({constructor_params}) -> Self {{
                Self {{
        {constructor_assignments}        }}
            }}
    
            pub fn serialize(&self) -> Vec<u8> {{
                let mut buffer = Vec::new();
        {serialization_code}
                buffer
            }}
    
            pub fn deserialize(buffer: &[u8]) -> io::Result<Self> {{
                let mut offset = 0;
        {deserialization_code}
                Ok(Self {{
                    {constructor_assignments}
                }})
            }}
        }}",
            struct_name = struct_name,
            struct_fields = struct_fields.trim_end(),
            constructor_params = constructor_params.trim_end_matches(", "),
            constructor_assignments = constructor_assignments.trim_end(),
            serialization_code = serialization_code.trim_end(),
            deserialization_code = deserialization_code.trim_end()
        )
    }
    
        


}


impl CodeGenerator for RustGenerator {
    fn generate(&mut self) {

        println!("================= Generate Option =================");
        let mut _source = CodeGenOptionManager::get_generated_source_code();
        let mut _file_path = CodeGenOptionManager::get_file_name();
        println!("Generate File Name : {}", _file_path.clone());
        let mut _directory = CodeGenOptionManager::get_generate_directory();
        println!("Generate File Path : {}", _directory.clone());
        let mut _gen_mode = CodeGenOptionManager::get_gen_laungauge_mode();

        let mut _gen_mode_str = CodeGenOptionManager::get_code_gen_lang_option_by_string();
        println!("Generate Language : {}", _gen_mode_str.clone());
        println!("================= Generate Option =================");

        self.write(_directory, 
            _file_path, 
            _source,
            _gen_mode);
    }

    fn parse(&mut self) {

        println!("========== Parse Option ==========");
        let directory_name = CodeGenOptionManager::get_target_file_directory();
        println!("Tareget Directory : {}", directory_name.clone());
        let file_name = CodeGenOptionManager::get_file_name();
        println!("Tareget File Name : {}", file_name.clone());
        let _fileds_name = Self::get_first_part(file_name.as_str());
        println!("Data Fileds Name : {}", _fileds_name);
        println!("========== Parse Option ==========");

        let fields = read_parse_struct(directory_name, file_name.clone());

        println!("Data Fileds Result");

        for (key, value) in &fields {
            println!("Key: {}, Value: {}", key, value);
        }

        let rust_code = self.format_rust_code(_fileds_name.to_string(), fields);
        
        println!("{}", rust_code);
        CodeGenOptionManager::set_generated_source_code(rust_code);
    }

}