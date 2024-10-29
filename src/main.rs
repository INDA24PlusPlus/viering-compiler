use ast::AstParser;
use lexer::Lexer;
use semantic::Semantic;
use std::fs::remove_file;
use std::io::Write;
use std::process::Command;
use std::{
    env,
    fs::{self, File},
};
use transpiler::Transpiler;

pub mod ast;
pub mod lexer;
pub mod semantic;
pub mod transpiler;

fn print_error() {
    println!(
        r#"
Usage: viering-compiler <command> <file>

Commands:
  ast         Generates the AST
  transpile   Generates C code
  compile     Compiles the program
  run         Compies and runs the program
"#
    );
}

#[derive(PartialEq)]
enum Mode {
    Ast,
    Transpile,
    Compile,
    Run,
}

fn compile_c_code(code: &str, file_name: &str) {
    let temp_file = "temp.c";
    let mut file = File::create(temp_file).unwrap();
    file.write_all(code.as_bytes()).unwrap();

    Command::new("gcc")
        .arg(temp_file)
        .arg("-o")
        .arg(file_name)
        .output()
        .expect("Failed to compile, is gcc installed correctly?");

    remove_file(temp_file).unwrap();
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        print_error();
        return;
    }

    let mode = match args[1].as_str() {
        "ast" => Mode::Ast,
        "transpile" => Mode::Transpile,
        "compile" => Mode::Compile,
        "run" => Mode::Run,
        _ => {
            print_error();
            return;
        }
    };

    match fs::read_to_string(args[2].clone()) {
        Ok(contents) => {
            let mut lexer = Lexer::new(contents);
            let tokens = lexer.tokenize();

            match tokens {
                Ok(tokens) => {
                    let mut ast_parser = AstParser::new(tokens.clone());
                    let ast = match ast_parser.parse() {
                        Ok(ast) => ast,
                        Err(err) => {
                            println!("Error during ast construction: {}", err);
                            return;
                        }
                    };

                    if mode == Mode::Ast {
                        ast.print();
                        return;
                    }

                    match Semantic::new(ast.clone()).check() {
                        Ok(()) => {}
                        Err(err) => {
                            println!("Error during semantic analysis: {}", err);
                            return;
                        }
                    }

                    let transpiler = Transpiler::new();
                    let c_code = transpiler.transpile(ast);

                    if mode == Mode::Transpile {
                        println!("{}", c_code);
                        return;
                    }

                    let file_name = "main";
                    compile_c_code(&c_code, file_name);

                    if mode == Mode::Compile {
                        println!("Compiled code to file: \"main\"");
                        return;
                    }

                    let output = Command::new(format!("./{}", file_name)).output().unwrap();
                    print!("{}", String::from_utf8_lossy(&output.stdout));
                    remove_file("main").unwrap();
                }
                Err(err) => {
                    println!("{}", err)
                }
            };
        }
        Err(_) => {
            println!("Invalid file path supplied");
        }
    }
}
