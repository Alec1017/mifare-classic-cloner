use structopt::StructOpt;
use std::path::PathBuf;

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


fn main() {

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

            match key_file {
                Some(file) => { println!("{}", file.display()); }
                None => { println!("There is no key file given"); }
            }

            println!("{}", path.display());



            // First we want to dump the contents of the scanned card
            // Using the keyfile if it exists
            // let dump_blank = Command::new("mfoc")
            //                 .arg("-O")
            //                 .arg("blank_with_uid.mfd")
            //                 .output()
            //                 .expect("dumping of blank card failed");
            
            // if dump_blank.status.success() {
            //     println!("Successfully dumped blank card");
            // } else {
            //     eprintln!("Error: couldn't output blank card dump");
            //     std::process::exit(exitcode::USAGE);
            // }
        }
    }
}
