#![recursion_limit = "1024"]
#![feature(plugin)]
#![plugin(rocket_codegen)]
extern crate rocket;
#[macro_use]
extern crate error_chain;
extern crate clap;
extern crate walkdir;
extern crate rayon;
extern crate notify;
extern crate slug;
extern crate chrono;


mod errors {
    // Create the Error, ErrorKind, ResultExt, and Result types
    error_chain!{}
}
use errors::*;
use chrono::prelude::*;

mod nanobe;
use nanobe::*; // frontend, api, renderer

// Cargo.toml variables for default info
const VERSION: Option<&'static str> = option_env!("CARGO_PKG_VERSION");
const AUTHORS: Option<&'static str> = option_env!("CARGO_PKG_AUTHORS");
const ABOUT: Option<&'static str> = option_env!("CARGO_PKG_DESCRIPTION");


fn main() {
    if let Err(ref e) = run() {
        println!("error: {}", e);

        for e in e.iter().skip(1) {
            println!("caused by: {}", e);
        }
        if let Some(backtrace) = e.backtrace() {
            println!("backtrace: {:?}", backtrace);
        }

        ::std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let matches = clap::App::new("NANOBE")
        .version(VERSION.unwrap_or_else(|| "dev"))
        .author(AUTHORS.unwrap_or_else(|| "koehr <n@koehr.in>"))
        .about(ABOUT.unwrap_or_else(|| "a blog engine"))
        .subcommand(
            clap::SubCommand::with_name("prebuild")
                .about("precompile templates manually")
                .arg(
                    clap::Arg::with_name("source")
                        .short("s")
                        .long("source")
                        .takes_value(true)
                        .value_name("SOURCEDIR")
                        .help("markdown input directory"),
                )
                .arg(
                    clap::Arg::with_name("dest")
                        .short("d")
                        .long("dest")
                        .takes_value(true)
                        .value_name("DESTDIR")
                        .help("html output directory"),
                ),
        )
        .get_matches_safe()
        .unwrap_or_else(|e| {
            println!("{}", e);
            ::std::process::exit(0);
        });

    match matches.subcommand_name() {
        Some("prebuild") => matches
            .subcommand_matches("prebuild")
            .ok_or("Missing parameters".into())
            .and_then(|matches| {
                prebuild::run(
                    &matches.value_of("SOURCEDIR").unwrap(),
                    &matches.value_of("DESTDIR").unwrap(),
                )
            }),
        _ => {
            rocket::ignite()
                .mount("/", routes![frontend::index])
                .mount("/api", routes![api::index])
                .launch();
        },
    }

    Ok(())
}
