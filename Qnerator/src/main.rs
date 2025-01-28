mod code_generator;

use code_generator::gen_prompt::*;
use std::env;

fn main() {

    let mut _prompt = GenPrompt::new();

    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        println!("Prompt Run . . .");
        _prompt.run(args);
    } else {
        _prompt.print_help();
    }



}
