#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate human_panic;
extern crate clap;
extern crate csv;
extern crate failure;

use clap::{App, Arg};
use csv::Reader;
use failure::Error;
use std::fs::DirBuilder;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::time::{SystemTime, UNIX_EPOCH};

type Result<T> = std::result::Result<T, Error>;

#[derive(Deserialize, Debug)]
struct UserInfo {
    name: String,
    gender: String,
    home_address: String,
    mob_or_email: String,
}

#[inline]
pub fn get_data_from(path: &str) -> Result<Vec<String>> {
    let mut reader = Reader::from_path(path)?;
    let result: Vec<String> = reader
        .records()
        .filter_map(|record| record.ok())
        .map(|entity| {
            let data: UserInfo = entity.deserialize(None).unwrap();
            data.mob_or_email
        })
        .collect();
    Ok(result)
}

#[inline]
fn save_data_to(path: &str, data: &[String]) -> Result<()> {
    let f = File::create(path)?;
    let mut buffer = BufWriter::new(f);
    let new_line;
    if cfg!(target_os = "linux") {
        new_line = "\n";
    } else {
        // windows
        new_line = "\r\n";
    }
    for record in data {
        buffer.write_fmt(format_args!("{}{}", record, new_line))?;
    }
    Ok(())
}

fn main() -> Result<()> {
    setup_panic!(Metadata {
        name: env!("CARGO_PKG_NAME").into(),
        version: env!("CARGO_PKG_VERSION").into(),
        authors: "Shady Khalifa <shekohex@gmail.com>".into(),
        homepage: "shekohex.github.io".into(),
    });
    let matches = App::new("FB CSV Data Extractor")
        .version("0.1.0")
        .author("Shady Khalifa <shekohex@gmail.com>")
        .about("Extract Data from CSV files")
        .after_help("if you don't undersand anything, just ping me on my email .")
        .arg(
            Arg::with_name("email-csv")
                .short("e")
                .long("email-csv")
                .value_name("FILE")
                .required(true)
                .help("Sets the path to the csv file that contains emails")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("mob-csv")
                .short("m")
                .long("mob-csv")
                .value_name("FILE")
                .required(true)
                .help("Sets the path to the csv file that contains mobile numbers")
                .takes_value(true),
        )
        .get_matches();
    DirBuilder::new().recursive(true).create("./Extracted")?;
    println!(":: Starting Reading files");
    let emails_file_path = matches.value_of("email-csv").unwrap();
    let mobs_file_path = matches.value_of("mob-csv").unwrap();
    let sys_time = SystemTime::now();
    let now = sys_time.duration_since(UNIX_EPOCH)?;
    let emails = get_data_from(emails_file_path)?;
    let mobile_numbers = get_data_from(mobs_file_path)?;
    println!(":: Got {} Emails", emails.len());
    println!(":: Got {} Mobile Numbers", mobile_numbers.len());
    let format_name = |name| format!("{}_{}.txt", name, now.subsec_nanos());
    save_data_to(&format_name("./Extracted/EMails"), &emails)?;
    save_data_to(&format_name("./Extracted/MobileNumbers"), &mobile_numbers)?;
    let duration = sys_time.elapsed()?;
    println!(
        ":: Done in {}s and {}ms !",
        duration.as_secs(),
        duration.subsec_millis(),
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_get_emails_from() {
        let path = "./test-files/Mails.csv";
        let emails = get_data_from(path).unwrap();
        println!("Emails {:#?}", emails);
        assert!(emails.len() > 1);
    }

    #[test]
    fn test_get_mob_from() {
        let path = "./test-files/MobileNumbers.csv";
        let mob = get_data_from(path).unwrap();
        println!("mob {:#?}", mob);
        assert!(mob.len() > 1);
    }
}
