#![recursion_limit = "1024"]
// #![feature(plugin)]
// #![plugin(rocket_codegen)]
// extern crate rocket;
// extern crate rocket_contrib;
// extern crate notify;
// extern crate slug;

#[macro_use] extern crate bart_derive;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate slog;
extern crate sloggers;
extern crate serde_yaml;
extern crate serde_json;
extern crate clap;
extern crate comrak;
extern crate glob;
extern crate chrono;
extern crate rayon;


// use rocket_contrib::Template;
// use chrono::prelude::*;

mod logger;
mod nanobe {
  pub mod website;
  pub mod article;
  pub mod build;
  pub mod template;
}

use nanobe::build::build;
use nanobe::template::test;

// Cargo.toml variables for default info
const VERSION: Option<&'static str> = option_env!("CARGO_PKG_VERSION");
const AUTHORS: Option<&'static str> = option_env!("CARGO_PKG_AUTHORS");
const ABOUT: Option<&'static str> = option_env!("CARGO_PKG_DESCRIPTION");


fn main() {
    let logger = logger::build();

    let source_arg = clap::Arg::with_name("SOURCEDIR")
        .index(1)
        .help("markdown input directory");

    let destination_arg = clap::Arg::with_name("DESTDIR")
        .index(2)
        .help("html output directory");

    let build_cmd = clap::SubCommand::with_name("build")
        .about("precompile templates manually")
        .arg(&source_arg)
        .arg(&destination_arg);

    let template_cmd = clap::SubCommand::with_name("template")
        .about("render template test");

    let matches = clap::App::new("NANOBE")
        .about(ABOUT.unwrap_or_else(|| "a blog engine"))
        .author(AUTHORS.unwrap_or_else(|| "koehr <n@koehr.in>"))
        .version(VERSION.unwrap_or_else(|| "dev"))
        .subcommand(build_cmd)
        .subcommand(template_cmd)
        .setting(clap::AppSettings::SubcommandRequiredElseHelp)
        .setting(clap::AppSettings::InferSubcommands)
        .setting(clap::AppSettings::VersionlessSubcommands)
        .get_matches_safe()
        .unwrap_or_else(|e| {
            println!("{}", e);
            ::std::process::exit(0);
        });


    match matches.subcommand() {
      ("build", Some(m)) => {
          let src = &m.value_of("SOURCEDIR").unwrap_or("articles");
          let dest = &m.value_of("DESTDIR").unwrap_or("html");
          info!(logger, "building from {} into {}", src, dest);
          build(src, dest)
      },
      ("template", Some(_)) => {
          test()
      },
      _ => {}
    }
}
