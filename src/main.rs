use std::{fs::{read_to_string, write}, collections::HashMap};

use regex::Regex;

#[derive(Debug)]
struct Pfn {
    ret_type: String,
    args: String
}
impl Pfn {
    fn new(ret_type: &str, args: &str) -> Pfn {
        Pfn {ret_type: ret_type.to_owned(), args: args.to_owned()}
    }
}

fn main() {
    let source = read_to_string("gl.h").unwrap();
    let regex = Regex::new(r#"typedef (.+) \(GLAD_API_PTR \*(\w+)\)\((.*)\);"#).unwrap();
    let api_regex = Regex::new(r#"GLAD_API_CALL (\w+) (\w+)"#).unwrap();
    let def_regex = Regex::new(r#"#define (\w+) (glad\w+)"#).unwrap();
    let mut pfns: HashMap<&str, Pfn> = HashMap::new();
    let mut pfn_glad_map: HashMap<&str, &str> = HashMap::new();
    let mut glad_gl_map: HashMap<&str, &str> = HashMap::new();
    for line in source.lines() {
        match regex.captures(line) {
            Some(captures) => {
                pfns.insert(captures.get(2).unwrap().as_str(), Pfn::new(
                captures.get(1).unwrap().as_str(),
                captures.get(3).unwrap().as_str()));
            },
            None => {},
        };

        match api_regex.captures(line) {
            Some(captures) => {
                pfn_glad_map.insert(captures.get(2).unwrap().as_str(), captures.get(1).unwrap().as_str());
            },
            None => {},
        };

        match def_regex.captures(line) {
            Some(captures) => {
                glad_gl_map.insert(captures.get(2).unwrap().as_str(), captures.get(1).unwrap().as_str());
            },
            None => {},
        };
    }

    let mut output = String::from("#include \"glad/gl.h\"\n\n");
    for (glad_fn, gl_fn) in glad_gl_map.iter() {
        let pfn_name = pfn_glad_map.get(glad_fn).expect(format!("glad function is missing for {}", gl_fn).as_str());
        let pfn = pfns.get(pfn_name).expect(format!("pfn is missing for {}", glad_fn).as_str());
        let args: Vec<&str> = pfn.args.split(&[',']).map(|f| f.trim().split_whitespace().last().unwrap_or_default()).filter(|f| f != &"void").collect();
        output += format!("#undef {}\n", gl_fn).as_str();
        output += format!("inline {} {}({}) {{\n", pfn.ret_type, gl_fn, pfn.args).as_str();
        output += format!("\treturn {}({});\n", glad_fn, args.join(", ")).as_str();
        output += "}\n";
    }

    write("gl_fixed.h", output).unwrap();
}
