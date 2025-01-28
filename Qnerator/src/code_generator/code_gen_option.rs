use std::sync::{RwLock, Arc};
use lazy_static::lazy_static;

lazy_static! {
    static ref INSTANCE: Arc<RwLock<CodeGenOptionManager>> = Arc::new(RwLock::new(CodeGenOptionManager::new()));
}

pub struct CodeGenOptionManager {

    gen_option : CodeGenProperty

}

impl CodeGenOptionManager {
    pub fn new() -> Self {
        CodeGenOptionManager {
            gen_option: CodeGenProperty::new(),
        }
    }

    pub fn get_instance() -> &'static Arc<RwLock<CodeGenOptionManager>> {
        &INSTANCE
    }

    pub fn get_target_file_directory() -> String {
        let mut _inst = CodeGenOptionManager::get_instance().write().unwrap();
        let mut _ret = _inst.gen_option.get_target_file_directory();
        return _ret
    }

    pub fn set_target_file_direcotry(_directory : String) {
        let mut _inst = CodeGenOptionManager::get_instance().write().unwrap();
        _inst.gen_option.set_target_file_directory(_directory.clone());
    }

    pub fn get_generate_directory() -> String {
        let mut _inst = CodeGenOptionManager::get_instance().write().unwrap();
        let mut _ret = _inst.gen_option.get_generate_directory();
        return _ret
    }

    pub fn set_generate_direcotry(_directory : String) {
        let mut _inst = CodeGenOptionManager::get_instance().write().unwrap();
        _inst.gen_option.set_generate_directory(_directory.clone());
    }

    pub fn get_file_name() -> String {
        let mut _inst = CodeGenOptionManager::get_instance().write().unwrap();
        let mut _ret = _inst.gen_option.get_file_name();
        return _ret
    }

    pub fn set_file_name(_file_name : String) {
        let mut _inst = CodeGenOptionManager::get_instance().write().unwrap();
        _inst.gen_option.set_file_name(_file_name.clone());
    }

    pub fn get_gen_laungauge_mode() -> GenType {
        let mut _inst = CodeGenOptionManager::get_instance().write().unwrap();
        let mut _ret = _inst.gen_option.get_mode();
        return _ret
    }

    pub fn set_gen_laungauge_mode(_type : GenType) {
        let mut _inst = CodeGenOptionManager::get_instance().write().unwrap();
        _inst.gen_option.set_mode(_type.clone());
    }

    pub fn get_generated_source_code() -> String {
        let mut _inst = CodeGenOptionManager::get_instance().write().unwrap();
        let mut _ret = _inst.gen_option.get_generated_source_code();
        return _ret
    }

    pub fn set_generated_source_code(_src : String) {
        let mut _inst = CodeGenOptionManager::get_instance().write().unwrap();
        _inst.gen_option.set_generated_source_code(_src.clone());
    }

    pub fn get_code_gen_lang_option_by_string() -> String {
        let mut _inst = CodeGenOptionManager::get_instance().write().unwrap();
        let mut _result = _inst.gen_option.get_language_option_as_string();

        return _result
    }

}


#[derive(Clone)]
pub enum GenType {
    NONE,
    CPP,
    RUST,
    GO,
    PYTHON,
    CSHARP
}

#[derive(Clone)]
pub struct CodeGenProperty {
    target_file_directory: String,
    generate_directory: String,
    file_name: String,
    gen_mode: GenType,
    generated_source_code : String
}

impl CodeGenProperty {
    pub fn new() -> Self {
        return CodeGenProperty{
            target_file_directory : "".to_string(),
            generate_directory : "".to_string(),
             file_name : "".to_string(),
              gen_mode : GenType::NONE,
              generated_source_code : "".to_string()
            }
    }

    pub fn get_generate_directory(&mut self) -> String {
        return self.generate_directory.clone()
    }

    pub fn get_file_name(&mut self) -> String {
        return self.file_name.clone()
    }

    pub fn get_mode(&mut self) -> GenType {
        return self.gen_mode.clone()
    }

    pub fn get_target_file_directory(&mut self) -> String {
        return self.target_file_directory.clone()
    }

    pub fn get_generated_source_code(&mut self) -> String {
        return self.generated_source_code.clone()
    }

    pub fn set_generated_source_code(&mut self, _src: String) {
        self.generated_source_code = _src;
    }

    pub fn set_target_file_directory(&mut self, _directory: String) {
        self.target_file_directory = _directory;
    }

    pub fn set_generate_directory(&mut self, _direcotry : String) {
        self.generate_directory = _direcotry;
    }

    pub fn set_file_name(&mut self, _file_name : String) {
        self.file_name = _file_name;
    }

    pub fn set_mode(&mut self, _type : GenType) {
        self.gen_mode = _type;
    }

    pub fn get_language_option_as_string(&self) -> String {

        let mut result = "".to_string();

        match self.gen_mode {
            GenType::CPP => {
                result = "cpp".to_string();
            },
            GenType::RUST => {
                result = "rust".to_string();
            },
            GenType::PYTHON => {
                result = "python".to_string();
            },
            GenType::GO => {
                result = "go".to_string();
            },
            GenType::CSHARP => {
                result = "csharp".to_string();
            },
            GenType::NONE => {
                result = "none".to_string();
            },
            _ => {
                result = "none".to_string();
            }
        }        

        return result
    }

}