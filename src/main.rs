use std::path::PathBuf;
use std::time::{Instant};

use structopt::StructOpt;
use console::{style, Emoji};
use indicatif::{HumanDuration};

extern crate exitcode;



#[derive(StructOpt)]
#[structopt(about = "Mifare Classic Card Cloner\n\nA CLI to facillitate easy cloning of mifare classic cards")]
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

static LOOKING_GLASS: Emoji<'_, '_> = Emoji("🔍  ", "");
static TRUCK: Emoji<'_, '_> = Emoji("🚚  ", "");
static CLIP: Emoji<'_, '_> = Emoji("🔗  ", "");
static PAPER: Emoji<'_, '_> = Emoji("📃  ", "");
static SPARKLE: Emoji<'_, '_> = Emoji("✨ ", ":-)");
static SUCCESS: Emoji<'_, '_> = Emoji("✔️  ", "");


fn main() {

    let time_started = Instant::now();

    // let spinner_style = ProgressStyle::default_spinner()
    //     .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ ")
    //     .template("{prefix:.bold.dim} {spinner} {wide_msg}");

    match Mfcc::from_args() {
        Mfcc::WriteBlank { uid, path } => {

            let output_file_name = "blank_with_uid.mfd";

            // Set the blank card to have the proper uid
            if let Ok(_) = mfcc::set_card_uid(&uid) {
                println!("Successfully set card to have UID: {}", &uid);
            }

            // Dump the blank card
            if let Ok(_) = mfcc::dump_card(None, &output_file_name) {
                println!("Successfully dumped blank card");
            }

            let dumped_card = PathBuf::from(&output_file_name);

            // Write to blank card
            if let Ok(_) = mfcc::write_card(true, &path, &dumped_card) {
                println!("Successfully wrote blank card");
            }

            // Remove any generated files
            if let Ok(_) = mfcc::remove_generated_file(&dumped_card) {
                println!("Cleaning up");
            }

            println!("Done!");
        }

        Mfcc::Overwrite { key_file, path} => {

            // Successfully dumped given card


            // Successfully formatted card


            // Successfully dumped formatted card


            // Successfully wrote onto card

            // Done!

            let initial_card_state   = "initial_card_state.mfd";
            let formatted_card_state = "formatted_card_state.mfd";

            println!(
                "{} {}Dumping given card...",
                style("[1/4]").bold().dim(),
                LOOKING_GLASS
            );

            // first we want to dump the card (using the keys or not)
            if let Ok(_) = mfcc::dump_card(key_file.as_ref(), &initial_card_state) {
                // println!("Successfully dumped given card");
                println!(
                    "{} {}Card dumped",
                    style("[1/4]").bold().dim(),
                    SUCCESS
                );
            }

            println!(
                "{} {}Formatting card...",
                style("[2/4]").bold().dim(),
                TRUCK
            );

            let initial_dumped_card = PathBuf::from(&initial_card_state);

            // then we want to format the card
            if let Ok(_) = mfcc::format_card(&initial_dumped_card) {
                // println!("Successfully formatted card");
                println!(
                    "\r{} {}Formatted card",
                    style("[2/4]").bold().dim(),
                    SUCCESS
                );
            }

            println!(
                "{} {}Dumping formatted card...",
                style("[3/4]").bold().dim(),
                CLIP
            );

            // now dump formatted card
            if let Ok(_) = mfcc::dump_card(None, &formatted_card_state) {
                // println!("Successfully dumped formatted card");
                println!(
                    "\r{} {}Dumped formatted card",
                    style("[3/4]").bold().dim(),
                    SUCCESS
                );
            }

            let formatted_card = PathBuf::from(&formatted_card_state);

            println!(
                "{} {}Writing to formatted card...",
                style("[4/4]").bold().dim(),
                PAPER
            );

            // Write to formatted card
            if let Ok(_) = mfcc::write_card(false, &path, &formatted_card) {
                // println!("Successfully wrote onto card");
                println!(
                    "\r{} {}Wrote to card...",
                    style("[4/4]").bold().dim(),
                    SUCCESS
                );
            }

            // Remove initial card dump
            if let Ok(_) = mfcc::remove_generated_file(&initial_dumped_card) {
                println!("Cleaning up initial card dump");
            }

            // Remove formatted card dump
            if let Ok(_) = mfcc::remove_generated_file(&formatted_card) {
                println!("Cleaning up formatted card dump");
            }

            println!("{} Done in {}", SPARKLE, HumanDuration(time_started.elapsed()));
        }
    }
}
