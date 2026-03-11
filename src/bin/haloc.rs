// The Halo Programming Language
// Compiler: converts .halo files to executable binaries
// Version: 0.2.0
// License: MPL 2.0
// SPDX-License-Identifier: MPL-2.0

use std::env;
use std::fs;
use std::process::{exit, Command};

use halo::compiler::Compilation;
use halo::lexer::Lexer;
use halo::parser::Parser;

struct CompilerConfig {
    input_file: String,
    output_file: String,
    emit_llvm: bool,
    verbose: bool,
    optimize: bool,
}

fn parse_args() -> Result<CompilerConfig, String> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        return Err("Usage: haloc <input.halo> [options]".to_string());
    }

    let mut config = CompilerConfig {
        input_file: String::new(),
        output_file: "a.out".to_string(),
        emit_llvm: false,
        verbose: false,
        optimize: false,
    };

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "-o" => {
                if i + 1 < args.len() {
                    config.output_file = args[i + 1].clone();
                    i += 2;
                } else {
                    return Err("Expected output filename after -o".to_string());
                }
            }
            "-emit-llvm" => {
                config.emit_llvm = true;
                i += 1;
            }
            "-v" | "--verbose" => {
                config.verbose = true;
                i += 1;
            }
            "-O" => {
                config.optimize = true;
                i += 1;
            }
            "-h" | "--help" => {
                print_help();
                exit(0);
            }
            arg if !arg.starts_with('-') => {
                config.input_file = arg.to_string();
                i += 1;
            }
            arg => {
                return Err(format!("Unknown option: {}", arg));
            }
        }
    }

    if config.input_file.is_empty() {
        return Err("No input file specified".to_string());
    }

    Ok(config)
}

fn print_help() {
    println!("╔════════════════════════════════════════════════╗");
    println!("║    🌟 Halo Compiler v0.2.0 🌟                ║");
    println!("╚════════════════════════════════════════════════╝\n");

    println!("USAGE:");
    println!("    haloc [OPTIONS] <INPUT.halo>\n");

    println!("OPTIONS:");
    println!("    -o <output>      Output filename (default: a.out)");
    println!("    -emit-llvm       Emit LLVM IR (.ll file)");
    println!("    -O               Enable optimizations");
    println!("    -v, --verbose    Verbose output");
    println!("    -h, --help       Print this help message\n");

    println!("EXAMPLES:");
    println!("    haloc program.halo              Compile to a.out");
    println!("    haloc program.halo -o program   Compile to 'program'");
    println!("    haloc program.halo -emit-llvm   Emit LLVM IR\n");
}

fn main() {
    let config = match parse_args() {
        Ok(cfg) => cfg,
        Err(e) => {
            eprintln!("❌ Error: {}", e);
            eprintln!("Run 'haloc --help' for usage information");
            exit(1);
        }
    };

    if config.verbose {
        println!("╔════════════════════════════════════════╗");
        println!("║    🌟 Halo Compiler v0.2.0 🌟        ║");
        println!("╚════════════════════════════════════════╝\n");
    }

    // Step 1: Read source file
    if config.verbose {
        println!("📂 Reading file: {}", config.input_file);
    }

    let source = match fs::read_to_string(&config.input_file) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("❌ Cannot read file '{}': {}", config.input_file, e);
            exit(1);
        }
    };

    if config.verbose {
        println!("✅ File read successfully ({} bytes)\n", source.len());
    }

    // Step 2: Tokenize
    if config.verbose {
        println!("🔍 Tokenizing...");
    }

    let mut lexer = Lexer::new(source);
    let mut tokens = Vec::new();
    loop {
        let token = lexer.next_token();
        let is_eof = token.token_type == halo::lexer::TokenType::EOF;
        tokens.push(token);
        if is_eof {
            break;
        }
    }

    if config.verbose {
        println!("✅ Tokenization successful ({} tokens)\n", tokens.len());
    }

    // Step 3: Parse
    if config.verbose {
        println!("📝 Parsing...");
    }

    let mut parser = Parser::new(tokens);
    let program = match parser.parse() {
        Ok(prog) => prog,
        Err(errors) => {
            eprintln!("❌ Parsing errors:");
            for error in errors {
                eprintln!("   {}", error);
            }
            exit(1);
        }
    };

    if config.verbose {
        println!("✅ Parsing successful\n");
    }

    // Step 4: Generate LLVM IR
    if config.verbose {
        println!("⚙️  Generating LLVM IR...");
    }

    let mut comp = Compilation::new("halo_program");
    if let Err(e) = comp.codegen().compile(&program) {
        eprintln!("❌ Compilation error: {}", e);
        exit(1);
    }

    if config.verbose {
        println!("✅ LLVM IR generation successful\n");
    }

    // Step 5: Emit LLVM IR if requested
    let ll_file = format!("{}.ll", config.output_file);
    if config.verbose || config.emit_llvm {
        if config.verbose {
            println!("💾 Emitting LLVM IR to: {}", ll_file);
        }

        if let Err(e) = comp.emit_llvm(&ll_file) {
            eprintln!("❌ Failed to emit LLVM IR: {}", e);
            exit(1);
        }

        if config.verbose {
            println!("✅ LLVM IR saved\n");
        }
    }

    // Step 5b: Always write the .ll so llc has something to compile.
    if !config.emit_llvm && !config.verbose {
        if let Err(e) = comp.emit_llvm(&ll_file) {
            eprintln!("❌ Failed to write temporary IR: {}", e);
            exit(1);
        }
    }

    // Step 6: Compile LLVM IR to assembly
    if config.verbose {
        println!("🔨 Compiling LLVM IR to assembly...");
    }

    let asm_file = format!("{}.s", config.output_file);
    let mut llc_cmd = Command::new("llc");
    llc_cmd.arg(&ll_file).arg("-o").arg(&asm_file);

    if config.optimize {
        llc_cmd.arg("-O2");
    }

    match llc_cmd.output() {
        Ok(output) => {
            if !output.status.success() {
                eprintln!("❌ LLC compilation failed");
                eprintln!("{}", String::from_utf8_lossy(&output.stderr));
                exit(1);
            }
        }
        Err(e) => {
            eprintln!("❌ Failed to run llc: {}", e);
            eprintln!("Make sure LLVM is installed and llc is in PATH");
            exit(1);
        }
    }

    if config.verbose {
        println!("✅ Assembly generated: {}\n", asm_file);
    }

    // Step 7: Link assembly to executable
    if config.verbose {
        println!("🔗 Linking executable...");
    }

    let mut cc_cmd = Command::new("cc");
    cc_cmd.arg(&asm_file).arg("-o").arg(&config.output_file);

    match cc_cmd.output() {
        Ok(output) => {
            if !output.status.success() {
                eprintln!("❌ Linking failed");
                eprintln!("{}", String::from_utf8_lossy(&output.stderr));
                exit(1);
            }
        }
        Err(e) => {
            eprintln!("❌ Failed to run cc: {}", e);
            eprintln!("Make sure a C compiler is installed (gcc, clang, etc)");
            exit(1);
        }
    }

    if config.verbose {
        println!("✅ Executable created: {}\n", config.output_file);
    }

    // Step 8: Cleanup (optional - keep .ll and .s files for debugging)
    if !config.emit_llvm && !config.verbose {
        let _ = fs::remove_file(&ll_file);
        let _ = fs::remove_file(&asm_file);
    }

    if config.verbose {
        println!("╔════════════════════════════════════════╗");
        println!("║      ✅ Compilation successful!        ║");
        println!("╚════════════════════════════════════════╝");
        println!("\n🚀 Run your program with: ./{}", config.output_file);
    } else {
        println!("✅ Compiled to: {}", config.output_file);
    }
}
