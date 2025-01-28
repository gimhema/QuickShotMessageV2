use super::gen_trait::*;
use super::code_gen_option::*;

use std::fs::{File, OpenOptions};
use std::io::{self, Write, Read};
use std::path::Path;

use std::fs;


use super::GenType;

use super::CPPGenerator;
use super::RustGenerator;

pub enum MODE {
    DEFAULT,
    TEST,
    DIRECTORY
}

pub struct GenPrompt {
    mode : MODE
}

impl GenPrompt {
    pub fn new() -> Self {
        return GenPrompt{mode : MODE::DEFAULT}
    }

    fn get_first_word<'a>(&mut self, s: &'a str) -> &'a str {
        s.split_whitespace().next().unwrap_or("")
    }

    pub fn set_mode_by_prefix(&mut self, argv: String) -> MODE {
        let first_word = self.get_first_word(&argv);

        match first_word.to_lowercase().as_str() {
            "-t" => MODE::TEST,
            "-d" => MODE::DIRECTORY,
            _ => MODE::DEFAULT,
        }
    }

    pub fn print_help(&mut self) {
        println!("===============================");
        println!("-d");
        println!("-f");
        println!("===============================");
    }

    pub fn find_file_from_directory(directory : String) -> Result<Vec<String>, io::Error> {
        let mut result = Vec::new();

        // 디렉토리 읽기
        let entries = fs::read_dir(directory)?;

        for entry in entries {
            let entry = entry?;
            let path = entry.path();

            // 파일인지 확인하고 ".qsmb" 확장자 확인
            if path.is_file() {
                if let Some(extension) = path.extension() {
                    if extension == "qsmb" {
                        if let Some(file_name) = path.file_name() {
                            if let Some(file_name_str) = file_name.to_str() {
                                result.push(file_name_str.to_string());
                            }
                        }
                    }
                }
            }
        }

        Ok(result)
    }

    pub fn parse_file(&mut self) -> String {

        // 1. check file format
        // 2. decide parse mode 

        match CodeGenOptionManager::get_gen_laungauge_mode() {
            GenType::CPP => {
                println!("checked cpp");
                let mut generator = CPPGenerator::new();

                generator.parse();
                generator.generate();
            }
            GenType::GO  => {
                println!("checked go");
            }
            GenType::PYTHON  => {
                println!("checked python");
            }
            GenType::RUST  => {
                println!("checked rust");
                let mut generator = RustGenerator::new();

                generator.parse();
                generator.generate();
            }
            GenType::NONE  => {
                println!("unexpected format . . .");
            }
            _ => {println!("unexpected format . . .");}
        }


        return "".to_string()
    }

    pub fn set_generate_lanugage_by_console_argv(&mut self, command : String) {

        match command.as_str() {
            "cpp" => {
                CodeGenOptionManager::set_gen_laungauge_mode(GenType::CPP);
                println!("Set cpp generate mode");
            },
            "rust" => { 
                CodeGenOptionManager::set_gen_laungauge_mode(GenType::RUST);
                println!("Set rust generate mode");
            },
            "go" => { 
                CodeGenOptionManager::set_gen_laungauge_mode(GenType::GO);
                println!("Set go generate mode"); 
            },
            "python" => { 
                CodeGenOptionManager::set_gen_laungauge_mode(GenType::PYTHON);
                println!("Set python generate mode");
            },
            "csharp" => { 
                CodeGenOptionManager::set_gen_laungauge_mode(GenType::CSHARP);
                println!("Set csharp generate mode");
            },
            _ => {println!("Unsupported type . . .");}
        }
    }


    pub fn parse(&mut self, argv: Vec<String>) {

    // use case :  qnerator -f ExampleMEssage.qsmb cpp Example
    // use case :  qnerator -d ExampleMEssages cpp Example

    println!("Start Setting Generate Options");
    
    let _target_file_dir = argv[2].clone();
    let _generate_lang_str = argv[3].clone();
    let _generate_file_dir = argv[4].clone();

    println!("Target File Dir : {}", _target_file_dir);
    println!("Generate Mode Str : {}", _generate_lang_str);
    println!("Generate File Dir : {}", _generate_file_dir);

    CodeGenOptionManager::set_target_file_direcotry(_target_file_dir);
    println!("Set Target  File Directory");
    self.set_generate_lanugage_by_console_argv(_generate_lang_str);
    println!("Set Generate Language Options");
    self.set_generate_directory_by_param(_generate_file_dir);
    println!("Set Generate Directory");

    // return;

        match self.mode {
            MODE::DIRECTORY => {
                // 
                // argv[1] : prompt mode
                // argv[2] : target file directory
                // argv[3] : generate language
                // argv[4] : generate directory

                // qnerator -d <TARGET_FILE_DIRECTORY> <GENERATE_LANGUAGE> <GENERATE_DIRECTORY>
                // Example 1: qnerator -d /targets rust - => will generate /gen directory
                // Example 2: qnerator -d /targets rust /custom => will generate /custom directory


                let directory = argv[2].clone();

                // 함수 호출 및 Vec 순회
                match Self::find_file_from_directory(directory) {
                    Ok(files) => {
                        if files.is_empty() {
                            println!("No .qsmb files found in the directory.");
                        } else {
                            println!("Found .qsmb files:");
                            for file in files {
                                println!("Start Parse {}", file);
                                CodeGenOptionManager::set_file_name(file);
                                let mut _parse_result = self.parse_file();
                            }
                        }
                    }
                    Err(e) => eprintln!("Error reading directory: {}", e),
                }
                
            },
            _ => {println!("Unexpected action . . . .");}
        }

    }

    pub fn set_generate_directory_by_param(&mut self, param : String) {
        
        match param.as_str() {
            "-" => {
                CodeGenOptionManager::set_generate_direcotry("gen/".to_string());
            }
            _ => { 
                println!("set generate directory : {}", param.clone());
                CodeGenOptionManager::set_generate_direcotry(param);
            }
        }
    }

    pub fn param_valid(&mut self, argv: Vec<String>) {

        print!("Param Validation . . .");
        let mut idx = 0;
        for param in argv {
            println!("[{}] : [{}]", idx, param);
            idx += 1;
        }

    }

    pub fn run(&mut self, argv: Vec<String>) {
        println!("Entering run function with arguments: {:?}", argv);
    
        if argv.len() < 2 {
            println!("Insufficient arguments.");
            self.print_help();
            return;
        }
    
        self.mode = self.set_mode_by_prefix(argv[1].clone());
    
        match self.mode {
            MODE::DEFAULT => {
                self.print_help();
            }
            MODE::TEST => {
                self.param_valid(argv.clone());
            }
            MODE::DIRECTORY => {
                println!("Directory Mode . . .");
                self.parse(argv.clone());
            }
        }
    }

}
