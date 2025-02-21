mod value;
mod bytecode;
mod lex;
mod parse;
mod vm;



fn main() {
    let mut args = std::env::args();
    let project_name = args.next().unwrap(); // Get the name of the project

    let filename = match args.next() {
        Some(filename) => filename,
        None => {
            eprintln!("Usage: {} <filename>", project_name);
            std::process::exit(1);
        }
    };

    let lex = lex::Lex::new(&filename);  // 词法分析, 生成tokens
    let mut proto = parse::ParseProto::new(lex);   
    proto.compile();        // 语法分析，生成字节码
    let mut state = vm::ExeState::new();  
    state.run(&proto);  // 执行

    
}
