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
        CPPGenerator {}
    }

    pub fn format_cpp_code(&mut self, struct_name: &str, fields: &[(String, String)]) -> String {
        use std::fmt::Write as _;

        // 타입 매핑: 스칼라와 배열 원소 타입
        let map_scalar_cpp = |typ: &str| -> &'static str {
            match typ {
                "Integer" => "uint32_t",
                "Long"    => "uint64_t",
                "Float"   => "float",
                "String"  => "std::string",
                _ => panic!("Unsupported type: {}", typ),
            }
        };

        let map_array_elem_cpp = |elem: &str| -> &'static str {
            match elem {
                "Integer" => "uint32_t",
                "Long"    => "uint64_t",
                "Float"   => "float",
                "String"  => "std::string",
                _ => panic!("Unsupported array type: {}", elem),
            }
        };

        // 어떤 타입이 쓰이는지 미리 체크해서 헬퍼 람다들을 조건부로 생성해도 되지만,
        // 간결성을 위해 항상 공통 헬퍼를 생성한다.
        let mut out = String::new();

        // 헤더
        writeln!(out, "#pragma once").unwrap();
        writeln!(out, "#include <cstdint>").unwrap();
        writeln!(out, "#include <cstring>").unwrap();
        writeln!(out, "#include <string>").unwrap();
        writeln!(out, "#include <vector>\n").unwrap();

        // struct 헤더
        writeln!(out, "struct {} {{", struct_name).unwrap();

        // 멤버 선언(숫자형 기본값 0, 문자열/벡터는 기본 생성)
        for (typ, name) in fields {
            if typ.starts_with("Array") {
                let elem = &typ[5..];
                let cpp_elem = map_array_elem_cpp(elem);
                writeln!(out, "    std::vector<{}> {};", cpp_elem, name).unwrap();
            } else {
                let cpp_t = map_scalar_cpp(typ);
                match *typ {
                    ref t if t == "Integer" || t == "Long" || t == "Float" => {
                        writeln!(out, "    {} {} = 0;", cpp_t, name).unwrap();
                    }
                    _ => {
                        // String
                        writeln!(out, "    {} {};", cpp_t, name).unwrap();
                    }
                }
            }
        }
        writeln!(out).unwrap();

        // serialize() 시작
        writeln!(out, "    std::vector<uint8_t> serialize() const {{").unwrap();
        writeln!(out, "        std::vector<uint8_t> buffer;").unwrap();
        // 대략적 reserve(스칼라 4/8/4 + 문자열 길이)
        // 안전을 위해 간단히 문자열 길이 합산만 반영
        writeln!(out, "        size_t approx = 0;").unwrap();
        for (typ, name) in fields {
            match typ.as_str() {
                "Integer" => writeln!(out, "        approx += 4;").unwrap(),
                "Long"    => writeln!(out, "        approx += 8;").unwrap(),
                "Float"   => writeln!(out, "        approx += 4;").unwrap(),
                "String"  => writeln!(out, "        approx += 4 + {}.size();", name).unwrap(),
                t if t.starts_with("Array") => {
                    let elem = &t[5..];
                    match elem {
                        "Integer" => writeln!(out, "        approx += 4 + {}.size()*4;", name).unwrap(),
                        "Long"    => writeln!(out, "        approx += 4 + {}.size()*8;", name).unwrap(),
                        "Float"   => writeln!(out, "        approx += 4 + {}.size()*4;", name).unwrap(),
                        "String"  => writeln!(out, "        {{ approx += 4; for (auto &s: {}) approx += 4 + s.size(); }}", name).unwrap(),
                        _ => {}
                    }
                }
                _ => {}
            }
        }
        writeln!(out, "        buffer.reserve(approx);").unwrap();
        writeln!(out).unwrap();

        // put_* 헬퍼
        writeln!(out, "        auto put_u32 = [&](uint32_t v) {{").unwrap();
        writeln!(out, "            uint8_t b[4]; std::memcpy(b, &v, 4); buffer.insert(buffer.end(), b, b+4);").unwrap();
        writeln!(out, "        }};").unwrap();

        writeln!(out, "        auto put_u64 = [&](uint64_t v) {{").unwrap();
        writeln!(out, "            uint8_t b[8]; std::memcpy(b, &v, 8); buffer.insert(buffer.end(), b, b+8);").unwrap();
        writeln!(out, "        }};").unwrap();

        writeln!(out, "        auto put_f32 = [&](float v) {{").unwrap();
        writeln!(out, "            uint8_t b[4]; std::memcpy(b, &v, 4); buffer.insert(buffer.end(), b, b+4);").unwrap();
        writeln!(out, "        }};").unwrap();

        writeln!(out, "        auto put_str = [&](const std::string& s) {{").unwrap();
        writeln!(out, "            put_u32(static_cast<uint32_t>(s.size()));").unwrap();
        writeln!(out, "            buffer.insert(buffer.end(), reinterpret_cast<const uint8_t*>(s.data()), reinterpret_cast<const uint8_t*>(s.data()) + s.size());").unwrap();
        writeln!(out, "        }};").unwrap();

        writeln!(out, "        auto put_vec_u32 = [&](const std::vector<uint32_t>& v) {{").unwrap();
        writeln!(out, "            put_u32(static_cast<uint32_t>(v.size()));").unwrap();
        writeln!(out, "            for (auto x : v) put_u32(x);").unwrap();
        writeln!(out, "        }};").unwrap();

        writeln!(out, "        auto put_vec_u64 = [&](const std::vector<uint64_t>& v) {{").unwrap();
        writeln!(out, "            put_u32(static_cast<uint32_t>(v.size()));").unwrap();
        writeln!(out, "            for (auto x : v) put_u64(x);").unwrap();
        writeln!(out, "        }};").unwrap();

        writeln!(out, "        auto put_vec_f32 = [&](const std::vector<float>& v) {{").unwrap();
        writeln!(out, "            put_u32(static_cast<uint32_t>(v.size()));").unwrap();
        writeln!(out, "            for (auto x : v) put_f32(x);").unwrap();
        writeln!(out, "        }};").unwrap();

        writeln!(out, "        auto put_vec_str = [&](const std::vector<std::string>& v) {{").unwrap();
        writeln!(out, "            put_u32(static_cast<uint32_t>(v.size()));").unwrap();
        writeln!(out, "            for (auto &s : v) put_str(s);").unwrap();
        writeln!(out, "        }};").unwrap();

        writeln!(out).unwrap();

        // 필드 직렬화
        for (typ, name) in fields {
            match typ.as_str() {
                "Integer" => writeln!(out, "        put_u32({});", name).unwrap(),
                "Long"    => writeln!(out, "        put_u64({});", name).unwrap(),
                "Float"   => writeln!(out, "        put_f32({});", name).unwrap(),
                "String"  => writeln!(out, "        put_str({});", name).unwrap(),
                t if t.starts_with("Array") => {
                    let elem = &t[5..];
                    match elem {
                        "Integer" => writeln!(out, "        put_vec_u32({});", name).unwrap(),
                        "Long"    => writeln!(out, "        put_vec_u64({});", name).unwrap(),
                        "Float"   => writeln!(out, "        put_vec_f32({});", name).unwrap(),
                        "String"  => writeln!(out, "        put_vec_str({});", name).unwrap(),
                        _ => {}
                    }
                }
                _ => {}
            }
        }

        writeln!(out, "        return buffer;").unwrap();
        writeln!(out, "    }}\n").unwrap();

        // try_deserialize_at
        writeln!(out, "private:").unwrap();
        writeln!(out, "    static bool try_deserialize_at(const std::vector<uint8_t>& buf, size_t start, {}& out) {{", struct_name).unwrap();
        writeln!(out, "        size_t off = start;").unwrap();
        writeln!(out, "        auto need = [&](size_t n)->bool {{ return buf.size() - off >= n; }};").unwrap();

        writeln!(out, "        auto get_u32 = [&](uint32_t& v)->bool {{ if (!need(4)) return false; std::memcpy(&v, buf.data()+off, 4); off += 4; return true; }};").unwrap();
        writeln!(out, "        auto get_u64 = [&](uint64_t& v)->bool {{ if (!need(8)) return false; std::memcpy(&v, buf.data()+off, 8); off += 8; return true; }};").unwrap();
        writeln!(out, "        auto get_f32 = [&](float& v)->bool {{ if (!need(4)) return false; std::memcpy(&v, buf.data()+off, 4); off += 4; return true; }};").unwrap();

        writeln!(out, "        auto get_str = [&](std::string& s)->bool {{").unwrap();
        writeln!(out, "            uint32_t len = 0; if (!get_u32(len)) return false;").unwrap();
        writeln!(out, "            constexpr uint32_t MAX_FIELD = 1u << 20; if (len > MAX_FIELD) return false;").unwrap();
        writeln!(out, "            if (!need(len)) return false; s.assign(reinterpret_cast<const char*>(buf.data()+off), len); off += len; return true;").unwrap();
        writeln!(out, "        }};").unwrap();

        writeln!(out, "        auto get_vec_u32 = [&](std::vector<uint32_t>& v)->bool {{ uint32_t n=0; if(!get_u32(n)) return false; v.clear(); v.reserve(n); for(uint32_t i=0;i<n;++i){{ uint32_t t; if(!get_u32(t)) return false; v.push_back(t); }} return true; }};").unwrap();
        writeln!(out, "        auto get_vec_u64 = [&](std::vector<uint64_t>& v)->bool {{ uint32_t n=0; if(!get_u32(n)) return false; v.clear(); v.reserve(n); for(uint32_t i=0;i<n;++i){{ uint64_t t; if(!get_u64(t)) return false; v.push_back(t); }} return true; }};").unwrap();
        writeln!(out, "        auto get_vec_f32 = [&](std::vector<float>& v)->bool {{ uint32_t n=0; if(!get_u32(n)) return false; v.clear(); v.reserve(n); for(uint32_t i=0;i<n;++i){{ float t; if(!get_f32(t)) return false; v.push_back(t); }} return true; }};").unwrap();
        writeln!(out, "        auto get_vec_str = [&](std::vector<std::string>& v)->bool {{ uint32_t n=0; if(!get_u32(n)) return false; v.clear(); v.reserve(n); for(uint32_t i=0;i<n;++i){{ std::string s; if(!get_str(s)) return false; v.push_back(std::move(s)); }} return true; }};").unwrap();

        // 로컬 변수들 선언 및 읽기
        for (typ, name) in fields {
            if typ.starts_with("Array") {
                let elem = &typ[5..];
                let cpp_elem = map_array_elem_cpp(elem);
                writeln!(out, "        std::vector<{}> __{};", cpp_elem, name).unwrap();
            } else {
                let cpp_t = map_scalar_cpp(typ);
                match typ.as_str() {
                    "Integer" | "Long" | "Float" => writeln!(out, "        {} __{} = 0;", cpp_t, name).unwrap(),
                    "String" => writeln!(out, "        {} __{};", cpp_t, name).unwrap(),
                    _ => {}
                }
            }
        }
        writeln!(out).unwrap();

        // 읽기 수행
        for (typ, name) in fields {
            match typ.as_str() {
                "Integer" => writeln!(out, "        if (!get_u32(__{})) return false;", name).unwrap(),
                "Long"    => writeln!(out, "        if (!get_u64(__{})) return false;", name).unwrap(),
                "Float"   => writeln!(out, "        if (!get_f32(__{})) return false;", name).unwrap(),
                "String"  => writeln!(out, "        if (!get_str(__{})) return false;", name).unwrap(),
                t if t.starts_with("Array") => {
                    let elem = &t[5..];
                    match elem {
                        "Integer" => writeln!(out, "        if (!get_vec_u32(__{})) return false;", name).unwrap(),
                        "Long"    => writeln!(out, "        if (!get_vec_u64(__{})) return false;", name).unwrap(),
                        "Float"   => writeln!(out, "        if (!get_vec_f32(__{})) return false;", name).unwrap(),
                        "String"  => writeln!(out, "        if (!get_vec_str(__{})) return false;", name).unwrap(),
                        _ => {}
                    }
                }
                _ => {}
            }
        }
        writeln!(out).unwrap();

        // out에 할당(move)
        for (typ, name) in fields {
            if typ.starts_with("Array") || typ == "String" {
                writeln!(out, "        out.{0} = std::move(__{0});", name).unwrap();
            } else {
                writeln!(out, "        out.{0} = __{0};", name).unwrap();
            }
        }
        writeln!(out, "        return true;").unwrap();
        writeln!(out, "    }}\n").unwrap();

        // public deserialize
        writeln!(out, "public:").unwrap();
        writeln!(out, "    static {} deserialize(const std::vector<uint8_t>& buffer) {{", struct_name).unwrap();
        writeln!(out, "        {} out;", struct_name).unwrap();
        writeln!(out, "        size_t off0 = 0;").unwrap();
        writeln!(out, "        if (buffer.size() >= 8) {{").unwrap();
        writeln!(out, "            uint32_t len = 0; std::memcpy(&len, buffer.data(), 4);").unwrap();
        writeln!(out, "            if (len == buffer.size() || len + 4 == buffer.size()) off0 = 4;").unwrap();
        writeln!(out, "        }}").unwrap();
        writeln!(out, "        if (try_deserialize_at(buffer, off0, out)) return out;").unwrap();
        writeln!(out, "        if (off0 == 4 && try_deserialize_at(buffer, 0, out)) return out;").unwrap();
        writeln!(out, "        return out;").unwrap();
        writeln!(out, "    }}").unwrap();

        writeln!(out, "}};").unwrap();

        out
    }

    // parse(), generate()는 기존 로직 그대로 사용
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
