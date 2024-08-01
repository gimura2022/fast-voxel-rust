use std::{fs::File, io::Write};

use gimura_preprocessor_lib::prelude::*;

fn main() {
    println!("cargo::rerun-if-changed=./examples/example_shaders/");
    println!("cargo::rerun-if-changed=./shaders_std/");

    let preprocessor_options = PreprocessorOptions::default();
    let mut preprocessor = Preporcessor::new(preprocessor_options);
    
    preprocessor.add_source("main".to_string(), CodeSource::from_path("./examples/example_shaders/".to_string()));
    preprocessor.add_source("std".to_string(), CodeSource::from_path("./shader_std/".to_string()));

    let source = preprocessor.preprocess("main".to_string(), "main.wgsl".to_string());

    let mut file = File::create("target/compiled.wgsl")
        .expect("Error to open file!");

    file.write_all(source.as_bytes())
        .expect("Error to write file!");
}