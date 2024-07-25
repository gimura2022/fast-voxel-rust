use std::{fs::File, io::Write};

#[allow(unused_imports)]
use log::*;
#[allow(unused_imports)]
use env_logger::*;

use clap::Parser;

use fast_voxel_perprocessor::*;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    path: String,

    #[arg(short, long)]
    out: String
}

fn main() -> Result<(), ()> {
    let env = env_logger::Env::default()
        .filter_or("RUST_LOG", "fast_voxel_perprocessor=info");
    env_logger::init_from_env(env);

    let args = Args::parse();
    trace!("{:#?}", args);

    let preprocessed = preprocess_dir(args.path);

    match preprocessed {
        Ok(source) => {
            let mut file = File::create(args.out)
                .expect("Error to open file!");

            file.write_all(source.as_bytes())
                .expect("Error to write file!");

            info!("Done :)");

            Ok(())
        },
        Err(error) => {
            error!("{}", error.to_string());
            Err(())
        },
    }
}