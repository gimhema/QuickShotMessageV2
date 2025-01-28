use std::fs::File;
use std::io::{BufRead, BufReader};
use super::code_gen_option::*;

use std::io::{self, Write};
use std::path::Path;




pub fn read_parse_struct(directory_name: String, file_name: String) -> Vec<(String, String)> {
    // 안전하게 파일 경로 생성
    let file_path = Path::new(&directory_name).join(&file_name);

    let mut fields = Vec::new();

    // 파일 열기
    match File::open(&file_path) {
        Ok(file) => {
            let reader = BufReader::new(file);

            for (line_num, line) in reader.lines().enumerate() {
                if let Ok(trimmed_line) = line {
                    let trimmed_line = trimmed_line.trim();

                    // 빈 줄이나 주석 무시
                    if trimmed_line.is_empty() || trimmed_line.starts_with("//") {
                        continue;
                    }

                    // 필드 타입과 이름 추출
                    let parts: Vec<&str> = trimmed_line.split_whitespace().collect();
                    if parts.len() == 2 {
                        let field_type = match parts[0] {
                            "Integer" | "Long" | "Float" | "String" | "ArrayInteger" | "ArrayFloat" => {
                                parts[0].to_string()
                            }
                            _ => {
                                println!(
                                    "Warning: Unrecognized type '{}' on line {}",
                                    parts[0], line_num + 1
                                );
                                continue;
                            }
                        };

                        let field_name = parts[1].to_string();
                        fields.push((field_type, field_name));
                    } else {
                        println!("Warning: Invalid line format on line {}", line_num + 1);
                    }
                }
            }
        }
        Err(e) => {
            println!("Error: Failed to open file '{}'. Reason: {}", file_path.display(), e);
        }
    }

    fields
}

pub trait CodeGenerator {
    
    fn parse(&mut self) {

    }
    
    fn generate(&mut self) {

    }

    fn init_code_generator(&mut self, _source : String) {

    }

    fn change_file_format_by_gen_mode(&mut self, _file_name: String, _gen_mode: GenType) -> Option<String> {
        println!("Start change file name step");
        println!("Original file name: {}", _file_name);
    
        // Split the file name by '.' and handle the case where no '.' exists
        let mut parts = _file_name.rsplitn(2, '.'); // Split from the right, max 2 parts
        let extension = parts.next();
        let base_name = parts.next().unwrap_or(""); // Default to empty if no base name exists
    
        // Check if base_name is empty
        if base_name.is_empty() {
            println!("Invalid file name: no base name found.");
            return None;
        }
    
        // Determine the new file extension based on _gen_mode
        let _file_format = match _gen_mode {
            GenType::CPP => ".cpp",
            GenType::RUST => ".rs",
            GenType::PYTHON => ".py",
            GenType::CSHARP => ".cs",
            GenType::GO => ".go",
            _ => {
                println!("Unsupported type.");
                return None;
            }
        };
    
        // Create the new file name
        let result = format!("{}{}", base_name, _file_format);
        println!("Changed file name: {}", result);
    
        Some(result)
    }
    

    fn write(&mut self, _directory: String, _file_name: String, _source: String, _gen_mode : GenType) {
        
        let mut _generate_file_name = self.change_file_format_by_gen_mode(_file_name, _gen_mode);

        // Create the full path by combining directory and file path
        let full_path = Path::new(&_directory).join(&_generate_file_name.unwrap());

        
        // Try to open the file for writing
        match File::create(&full_path) {
            Ok(mut file) => {
                if let Err(err) = file.write_all(_source.as_bytes()) {
                    eprintln!("Failed to write to file: {}", err);
                } else {
                    println!("Code generation completed. File written to: {}", full_path.display());
                }
            }
            Err(err) => {
                eprintln!("Failed to create file: {}", err);
            }
        }
    }

    fn get_first_part(input: &str) -> &str {
        input.split('.').next().unwrap_or("")
    }
}
