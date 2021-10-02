extern crate rust_massa_tracker;
extern crate diesel;

#[macro_use]
extern crate diesel_migrations;

use clap;

use self::rust_massa_tracker::*;
use self::diesel::prelude::*;
use self::models;

use chrono::{offset::TimeZone, DateTime, Local};

embed_migrations!("./migrations");

fn main() {
    use rust_massa_tracker::schema::massaa::dsl::*;

    let conn = establish_connection();
    
    embedded_migrations::run(&conn).unwrap();

    let matches = clap::App::new("Tool for recording daily weight")
        .version("0.1")
        .setting(clap::AppSettings::SubcommandRequiredElseHelp)
        .subcommand(clap::SubCommand::with_name("add")
                .about("Add daily weight")
                .arg(clap::Arg::with_name("kilos")
                     .help("Päivän paino kilogrammoina")
                     .required(true)
                )
        )
        .subcommand(clap::SubCommand::with_name("show")
        )
        .get_matches();

    if let Some(sub_matches) = matches.subcommand_matches("add") {
        let kilos = clap::value_t!(sub_matches.value_of("kilos"),f32).unwrap();
        /*
         * We trust the argument value was given as required by the CLI parser,
         * and unrwap() directly. Otherwise we should do something like this:
        match kilos {
            Err(e) => println!("Nothing of value was lost: {}.", e),
            Ok(v) => println!("Value of massa: {:.2}", v + 10.0)
        }
        */

        diesel::insert_into(massaa).values((&kg.eq(kilos), &note_txt.eq("Omg!"))).execute(&conn).unwrap();

    }
    if let Some(_sub_matches) = matches.subcommand_matches("show") {

        let res = massaa
            .load::<self::models::Massa>(&conn)
            .expect("FUCK");

        println!("Displaying {} results", res.len());
        for r in res {
            let fuck = r.ts.map_or("-".to_string(), |v| {
                let loc : DateTime<Local> = Local.from_utc_datetime(&v);
                loc.format("%Y-%m-%d %H:%M:%S").to_string() 
            });
            let this = r.id.unwrap();
            let kgv  = r.kg.unwrap();
            let txt  = r.note_txt.unwrap_or("-".to_string());
            println!("{:03} | {} | {:.1} | {}", this, fuck, kgv, txt);
        }
    }

}