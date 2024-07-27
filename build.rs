use std::{fs::File, io::Write};

use fast_voxel_perprocessor::preprocess_dir;

fn main() -> Result<(), ()> {
    println!("cargo::rerun-if-changed=src/shaders");

    let preprocessed = preprocess_dir("src/shaders".to_string());

    match preprocessed {
        Ok(source) => {
            let mut file = File::create("target/compiled.wgsl")
                .expect("Error to open file!");

            file.write_all(source.as_bytes())
                .expect("Error to write file!");

            Ok(())
        },
        Err(error) => {
            panic!("{}", error.to_string());
        },
    }
}