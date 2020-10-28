use structopt::StructOpt;
use std::process::Command;
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
            println!("confirming uid: {}", uid);
            println!("confirming source file: {}", path.display());

            let output_file_name = "blank_with_uid.mfd";

            // first we want to set the blank card to have the proper uid
            match Command::new("nfc-mfsetuid").arg(&uid).output() {
                Ok(_) => println!("Successfully set card to have UID: {}", &uid),
                Err(_) => {
                    println!("Error: can't set uid onto card");
                    std::process::exit(exitcode::USAGE);
                }
            }

            // Next we want to dump the blank card
            if let Ok(_) = mfcc::dump_card(None, &output_file_name) {
                println!("Successfully dumped blank card");
            }

            let dumped_card = PathBuf::from(&output_file_name);

            if let Ok(_) = mfcc::write_card(true, &path, &dumped_card) {
                println!("Successfully wrote blank card");
            }

            // remove any generated files
            match std::fs::remove_file(&output_file_name) {
                Ok(_) => { println!("Cleaning up"); }
                Err(error) => {
                    eprintln!("Error: {}", error);
                    std::process::exit(exitcode::USAGE);
                }
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
