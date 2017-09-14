use clap;

use canvas;
use config;


pub fn subcommand(matches: &clap::ArgMatches) -> Result<(), String> {
    let config = config::get_config()?;
    let client = config::get_client()?;
    match matches.subcommand() {
        ("ls", Some(_ls_matches)) => {
            let courses = canvas::get_course_list(&config, &client)?;
            for course in courses {
                println!("{:<10} {}", format!("({})", course.id), course.name);
            }
        }
        ("info", Some(_info_matches)) => unimplemented!(),
        _ => unreachable!(),
    }
    Ok(())
}
