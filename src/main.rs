use std::fs::File;

use clap::Parser;
use tone_lang::{Args, frontend::parsing::audio_parsing::parse_mp3_to_frecuency_array};

fn main() {
    // need something for first grabing the audio notes and then passing it to the lexer (the syntax
    // analysis won't be that complex as checking strings)
    // Will construct the dfa and the parser burnt for just going faster

    // turn audio file in to num array

    // do frontend loop for passing lexeme through lexeme

    // then backend transform

    let args = Args::parse();

    let frecuencies = parse_mp3_to_frecuency_array(&args.src_code_path);

    println!("{frecuencies:?}");
}
