use crate::code_generator::read_parse_struct;

use super::CodeGenerator;
use super::code_gen_option::*;

use std::fs;
use std::io::{self, Write};

use std::fs::File;
use std::io::{BufRead, BufReader};
use super::gen_trait::*;

pub struct CPPGenerator{

}

impl CPPGenerator {
    pub fn new() -> Self {
        return CPPGenerator{}
    }

        pub fn format_cpp_code(&mut self, struct_name: &str, fields: &[(String, String)]) -> String {
            let mut cpp_code = String::new();
    
            // Structure definition
            cpp_code.push_str(&format!("#pragma pack(push, 1)\nstruct {} {{\n", struct_name));
    
            // Field definitions
            for (typ, name) in fields {
                let cpp_type = match typ.as_str() {
                    "Integer" => "uint32_t".to_string(),
                    "Long" => "uint64_t".to_string(),
                    "Float" => "float".to_string(),
                    "String" => "std::string".to_string(),
                    typ if typ.starts_with("Array") => {
                        let element_type = &typ[5..]; // "Array" 이후 타입 추출
                        let cpp_element_type = match element_type {
                            "Integer" => "int32_t",
                            "Long" => "int64_t",
                            "Float" => "float",
                            "String" => "std::string",
                            _ => panic!("Unsupported array type: {}", element_type),
                        };
                        format!("std::vector<{}>", cpp_element_type)
                    }
                    _ => panic!("Unsupported type: {}", typ),
                };
                cpp_code.push_str(&format!("    {} {};\n", cpp_type, name));
            }
    
            let constructor_params: Vec<String> = fields
            .iter()
            .map(|(typ, name)| {
                let cpp_type = match typ.as_str() {
                    "Integer" => "uint32_t".to_string(),
                    "Long" => "uint64_t".to_string(),
                    "Float" => "float".to_string(),
                    "String" => "std::string".to_string(),
                    typ if typ.starts_with("Array") => {
                        let element_type = match &typ[5..] {
                            "Integer" => "int32_t",
                            "Long" => "int64_t",
                            "Float" => "float",
                            "String" => "std::string",
                            _ => panic!("Unsupported array type: {}", &typ[5..]),
                        };
                        format!("std::vector<{}>", element_type) // String 반환
                    }
                    _ => panic!("Unsupported type: {}", typ),
                };
                format!("{} {}", cpp_type, name) // 최종적으로 String 반환
            })
            .collect();
        

    
            let init_list: Vec<String> = fields.iter().map(|(_, name)| format!("{}({})", name, name)).collect();
            cpp_code.push_str(&format!(
                "\n    {}({}) : {} {{}}\n\n",
                struct_name,
                constructor_params.join(", "),
                init_list.join(", ")
            ));
    
            // Serialize function
            cpp_code.push_str("    std::vector<uint8_t> serialize() const {\n");
            cpp_code.push_str("        std::vector<uint8_t> buffer;\n");
    
            for (typ, name) in fields {
                match typ.as_str() {
                    "Integer" | "Long" | "Float" => {
                        cpp_code.push_str(&format!(
                            "        buffer.insert(buffer.end(), reinterpret_cast<const uint8_t*>(&{}), reinterpret_cast<const uint8_t*>(&{} + 1));\n",
                            name, name
                        ));
                    }
                    "String" => {
                        cpp_code.push_str(&format!(
                            "        uint32_t {0}_length = {0}.size();\n",
                            name
                        ));
                        cpp_code.push_str(&format!(
                            "        buffer.insert(buffer.end(), reinterpret_cast<const uint8_t*>(&{0}_length), reinterpret_cast<const uint8_t*>(&{0}_length + 1));\n",
                            name
                        ));
                        cpp_code.push_str(&format!(
                            "        buffer.insert(buffer.end(), {0}.begin(), {0}.end());\n",
                            name
                        ));
                    }
                    typ if typ.starts_with("Array") => {
                        cpp_code.push_str(&format!(
                            "        uint32_t {0}_size = {0}.size();\n",
                            name
                        ));
                        cpp_code.push_str(&format!(
                            "        buffer.insert(buffer.end(), reinterpret_cast<const uint8_t*>(&{0}_size), reinterpret_cast<const uint8_t*>(&{0}_size + 1));\n",
                            name
                        ));
                        cpp_code.push_str(&format!(
                            "        buffer.insert(buffer.end(), reinterpret_cast<const uint8_t*>({0}.data()), reinterpret_cast<const uint8_t*>({0}.data() + {0}_size));\n",
                            name
                        ));
                    }
                    _ => panic!("Unsupported type: {}", typ),
                }
            }
            cpp_code.push_str("        return buffer;\n    }\n\n");
    
            // Deserialize function
            cpp_code.push_str(&format!(
                "    static {} deserialize(const std::vector<uint8_t>& buffer) {{\n",
                struct_name
            ));
            cpp_code.push_str("        size_t offset = 0;\n");
    
            let mut deserialized_fields = Vec::new();
            for (typ, name) in fields {
                match typ.as_str() {
                    "Integer" => cpp_code.push_str(&format!(
                        "        uint32_t {} = *reinterpret_cast<const uint32_t*>(buffer.data() + offset);\n        offset += sizeof(uint32_t);\n",
                        name
                    )),
                    "Long" => cpp_code.push_str(&format!(
                        "        uint64_t {} = *reinterpret_cast<const uint64_t*>(buffer.data() + offset);\n        offset += sizeof(uint64_t);\n",
                        name
                    )),
                    "Float" => cpp_code.push_str(&format!(
                        "        float {} = *reinterpret_cast<const float*>(buffer.data() + offset);\n        offset += sizeof(float);\n",
                        name
                    )),
                    "String" => {
                        cpp_code.push_str(&format!(
                            "        uint32_t {0}_length = *reinterpret_cast<const uint32_t*>(buffer.data() + offset);\n        offset += sizeof(uint32_t);\n",
                            name
                        ));
                        cpp_code.push_str(&format!(
                            "        std::string {0}(buffer.begin() + offset, buffer.begin() + offset + {0}_length);\n        offset += {0}_length;\n",
                            name
                        ));
                    }
                    typ if typ.starts_with("Array") => {
                        cpp_code.push_str(&format!(
                            "        uint32_t {0}_size = *reinterpret_cast<const uint32_t*>(buffer.data() + offset);\n        offset += sizeof(uint32_t);\n",
                            name
                        ));
                        let element_type = match &typ[5..] {
                            "Integer" => "int32_t",
                            "Long" => "int64_t",
                            "Float" => "float",
                            _ => panic!("Unsupported array type: {}", &typ[5..]),
                        };
                        cpp_code.push_str(&format!(
                            "        std::vector<{}> {}({}_size);\n",
                            element_type, name, name
                        ));
                        cpp_code.push_str(&format!(
                            "        std::memcpy({}.data(), buffer.data() + offset, {}_size * sizeof({}));\n        offset += {}_size * sizeof({});\n",
                            name, name, element_type, name, element_type
                        ));
                    }
                    _ => panic!("Unsupported type: {}", typ),
                }
                deserialized_fields.push(name.to_string());
            }
    
            cpp_code.push_str(&format!(
                "        return {}({});\n    }}\n",
                struct_name,
                deserialized_fields.join(", ")
            ));
    
            cpp_code.push_str("};\n#pragma pack(pop)\n");
    
            cpp_code
        }
    
    
}


impl CodeGenerator for CPPGenerator {
    fn generate(&mut self) {

        // let mut _source = self.gen_common.get_generate_source();
        // let mut _file_path = self.gen_common.get_genrate_file_name();
        // let mut _directory = self.gen_common.get_generate_file_path();
        
        let mut _source = CodeGenOptionManager::get_generated_source_code();
        let mut _file_path = CodeGenOptionManager::get_file_name();
        let mut _directory = CodeGenOptionManager::get_generate_directory();
        let mut _gen_mode = CodeGenOptionManager::get_gen_laungauge_mode();

        self.write(_directory, 
            _file_path, 
            _source,
            _gen_mode);
    }

    fn parse(&mut self) {

        // let directory_name = self.gen_common.get_read_file_path().clone();
        let directory_name = CodeGenOptionManager::get_target_file_directory();

//      let file_name = self.gen_common.get_genrate_file_name().clone(); // source 값을 로컬 변수로 복사하여 빌림 해제
        let mut _file_path = CodeGenOptionManager::get_file_name();

        let _fileds_name = Self::get_first_part(_file_path.as_str());

        let fields = read_parse_struct(directory_name, _file_path.clone());

        let cpp_code = self.format_cpp_code(&_fileds_name, &fields);
    

        println!("{}", cpp_code);
        // self.gen_common.set_generate_source(cpp_code);
        CodeGenOptionManager::set_generated_source_code(cpp_code);
    }

}
