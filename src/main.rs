use structopt::StructOpt;
// use anyhow::{Context, Result};
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
    // let args = Mfcc::from_args();
    // println!("uid: {}", &args.uid);
    // let content = std::fs::read_to_string(&args.path)
    //     .with_context(|| format!("could not read file `{}`", args.path.display()))?;

    // mfcc::find_matches(&content, &args.pattern, &mut std::io::stdout());

    match Mfcc::from_args() {
        Mfcc::WriteBlank { uid, path} => {
            println!("uid: {}", uid);
            println!("source file: {}", path.display());

            // first we want to set the blank card to have the proper uid
            let uid_set_output = Command::new("nfc-mfsetuid")
                                .arg(&uid)
                                .output()
                                .expect("nfc-mfsetuid command failed");

            if uid_set_output.status.success() {
                println!("Successfully set card to have UID: {}", &uid);
            } else {
                eprintln!("Error: couldn't find card");
                std::process::exit(exitcode::USAGE);
            }


            // Next we want to dump the blank card
            let dump_blank = Command::new("mfoc")
                            .arg("-O")
                            .arg("blank_with_uid.mfd")
                            .output()
                            .expect("dumping of blank card failed");
            
            if dump_blank.status.success() {
                println!("Successfully dumped blank card");
            } else {
                eprintln!("Error: couldn't output blank card dump");
                std::process::exit(exitcode::USAGE);
            }

            // Now we want to write the given source file onto the blank card
            let write_dump = Command::new("nfc-mfclassic")
                            .arg("W")
                            .arg("a")
                            .arg(&path)
                            .arg("blank_with_uid.mfd")
                            .output()
                            .expect("writing to blank card failed");
            
            
            if write_dump.status.success() {
                println!("Successfully wrote to blank card");
            } else {
                eprintln!("Error: couldn't write to blank card");
                std::process::exit(exitcode::USAGE);
            }

            // remove any generated files
            match std::fs::remove_file("blank_with_uid.mfd") {
                Ok(_) => { println!("Cleaning up"); }
                Err(error) => {
                    eprintln!("Error: {}", error);
                    std::process::exit(exitcode::USAGE);
                }
            }

            // println!("{}", output.status.success());
            // println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
            // println!("stderr: {}", String::from_utf8_lossy(&output.stderr));

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
