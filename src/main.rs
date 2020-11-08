use std::path::PathBuf;
use std::time::{Instant};
use std::io::stdout;

use structopt::StructOpt;
use console::{style, Emoji};
use indicatif::{HumanDuration};
use log_update::LogUpdate;

extern crate exitcode;



#[derive(StructOpt)]
#[structopt(about = "Mifare Classic Card Cloner\n\nA CLI to facilitate easy cloning of mifare classic cards")]
enum Mfcc {

    /// Writes a source file to a blank card
    WriteBlank {

        /// The UID that will be put on the cloned card
        uid: String,

        /// The source file to clone onto the new card
        #[structopt(parse(from_os_str))]
        path: PathBuf,

    },

    /// Overwrites content on card with given source file
    Overwrite {

        /// Optional .txt key file
        #[structopt(short = "k", long = "key")]
        key_file: Option<PathBuf>,

        /// The source file to overwrite the card
        #[structopt(parse(from_os_str))]
        path: PathBuf
    }
}

static DUMP:    Emoji<'_, '_> = Emoji("🔍  ", "");
static TARGET:  Emoji<'_, '_> = Emoji("🎯  ", "");
static FORMAT:  Emoji<'_, '_> = Emoji("🚚  ", "");
static CLEAN:   Emoji<'_, '_> = Emoji("🧹  ", "");
static WRITE:   Emoji<'_, '_> = Emoji("📝  ", "");
static SUCCESS: Emoji<'_, '_> = Emoji("✔️  ", "");
static DONE:    Emoji<'_, '_> = Emoji("✨  ", "");


fn main() {

    let time_started = Instant::now();

    // Construct the log updater
    let mut log_update = LogUpdate::new(stdout()).unwrap();


    match Mfcc::from_args() {
        Mfcc::WriteBlank { uid, path } => {

            let output_file_name = "blank_with_uid.mfd";

            log_update.render(&format!(
                "{} {}Setting card with new UID...",
                style("[1/3]").bold().dim(),
                TARGET
            )).unwrap();

            // Set the blank card to have the proper uid
            if let Ok(_) = mfc_cloner::set_card_uid(&uid) {
                log_update.render(&format!(
                    "{} {}Card UID set",
                    style("[1/3]").bold().dim(),
                    SUCCESS
                )).unwrap();
                println!("");
            }

            log_update.render(&format!(
                "{} {}Dumping given card...",
                style("[2/3]").bold().dim(),
                DUMP
            )).unwrap();

            // Dump the blank card
            if let Ok(_) = mfc_cloner::dump_card(None, &output_file_name) {
                log_update.render(&format!(
                    "{} {}Card dumped",
                    style("[2/3]").bold().dim(),
                    SUCCESS
                )).unwrap();
                println!("");
            }

            let dumped_card = PathBuf::from(&output_file_name);

            log_update.render(&format!(
                "{} {}Writing to blank card...",
                style("[3/3]").bold().dim(),
                WRITE
            )).unwrap();

            // Write to blank card
            if let Ok(_) = mfc_cloner::write_card(true, &path, &dumped_card) {
                log_update.render(&format!(
                    "{} {}Wrote to card",
                    style("[3/3]").bold().dim(),
                    SUCCESS
                )).unwrap();
                println!("");
            }

            // Remove any generated files
            if let Ok(_) = mfc_cloner::remove_generated_file(&dumped_card) {
                println!("{}Cleaning up card dump", CLEAN);
            }

            println!("\n\n{}Done in {}", DONE, HumanDuration(time_started.elapsed()));
        }

        Mfcc::Overwrite { key_file, path} => {
            let initial_card_state   = "initial_card_state.mfd";
            let formatted_card_state = "formatted_card_state.mfd";

            log_update.render(&format!(
                "{} {}Dumping given card...",
                style("[1/4]").bold().dim(),
                DUMP
            )).unwrap();

            // first we want to dump the card (using the keys or not)
            if let Ok(_) = mfc_cloner::dump_card(key_file.as_ref(), &initial_card_state) {
                log_update.render(&format!(
                    "{} {}Card dumped",
                    style("[1/4]").bold().dim(),
                    SUCCESS
                )).unwrap();
                println!("");
            }

            log_update.render(&format!(
                "{} {}Formatting card...",
                style("[2/4]").bold().dim(),
                FORMAT
            )).unwrap();

            let initial_dumped_card = PathBuf::from(&initial_card_state);

            // then we want to format the card
            if let Ok(_) = mfc_cloner::format_card(&initial_dumped_card) {
                log_update.render(&format!(
                    "{} {}Formatted card",
                    style("[2/4]").bold().dim(),
                    SUCCESS
                )).unwrap();
                println!("");
            }

            log_update.render(&format!(
                "{} {}Dumping formatted card...",
                style("[3/4]").bold().dim(),
                DUMP
            )).unwrap();

            // now dump formatted card
            if let Ok(_) = mfc_cloner::dump_card(None, &formatted_card_state) {
                log_update.render(&format!(
                    "{} {}Dumped formatted card",
                    style("[3/4]").bold().dim(),
                    SUCCESS
                )).unwrap();
                println!("");
            }

            let formatted_card = PathBuf::from(&formatted_card_state);

            log_update.render(&format!(
                "{} {}Writing to formatted card...",
                style("[4/4]").bold().dim(),
                WRITE
            )).unwrap();

            // Write to formatted card
            if let Ok(_) = mfc_cloner::write_card(false, &path, &formatted_card) {
                log_update.render(&format!(
                    "{} {}Wrote to card",
                    style("[4/4]").bold().dim(),
                    SUCCESS
                )).unwrap();
                println!("");
            }

            // Remove initial card dump
            if let Ok(_) = mfc_cloner::remove_generated_file(&initial_dumped_card) {
                println!("{}Cleaning up initial card dump", CLEAN);
            }

            // Remove formatted card dump
            if let Ok(_) = mfc_cloner::remove_generated_file(&formatted_card) {
                println!("{}Cleaning up formatted card dump", CLEAN);
            }

            println!("\n\n{}Done in {}", DONE, HumanDuration(time_started.elapsed()));
        }
    }
}
