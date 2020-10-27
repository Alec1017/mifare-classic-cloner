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

/// Writes the given .mfd file to the destinatin .mfd file
fn write_card(is_blank: bool, source_file: &PathBuf, dest_file: &PathBuf) -> Result<(), Box<dyn std::error::Error>>  {  

    // Need to store Command::new into its own variable first
    let mut command = Command::new("nfc-mfclassic");

    // Determine write option depending on type of card received
    if is_blank {
        command.arg("W");
    } else {
        command.arg("w");
    }

    // Add remaining arguments
    command.arg("a")
           .arg(source_file)
           .arg(dest_file)
           .output()?;
    Ok(())
}


/// Dumps the contents of a card into a .mfd with the given output name
fn dump_card(key_file: Option<&PathBuf>, output_name: &str) -> Result<(), Box<dyn std::error::Error>>  {   
    
    // Need to store Command::new into its own variable first
    let mut command = Command::new("mfoc");
    command.arg("-O")
           .arg(format!("{}.mfd", output_name));

    // Add the key file option, if it exists
    if let Some(file) = key_file {
        command.arg("-k").arg(file);
    }

    command.output()?;
    Ok(())
}



fn main() {
    // let args = Mfcc::from_args();
    // println!("uid: {}", &args.uid);
    // let content = std::fs::read_to_string(&args.path)
    //     .with_context(|| format!("could not read file `{}`", args.path.display()))?;

    // mfcc::find_matches(&content, &args.pattern, &mut std::io::stdout());

    match Mfcc::from_args() {
        Mfcc::WriteBlank { uid, path } => {
            println!("uid: {}", uid);
            println!("source file: {}", path.display());

            // first we want to set the blank card to have the proper uid
            match Command::new("nfc-mfsetuid").arg(&uid).output() {
                Ok(_) => println!("Successfully set card to have UID: {}", &uid),
                Err(_) => {
                    println!("Error: can't set uid onto card");
                    std::process::exit(exitcode::USAGE);
                }
            }

            // Next we want to dump the blank card
            match dump_card(None, "blank_with_uid") {
                Ok(_) => println!("Successfully dumped blank card"),
                Err(_) => {
                    println!("Error: couldn't output blank card dump");
                    std::process::exit(exitcode::USAGE);
                }
            }

            let dumped_card = PathBuf::from("blank_with_uid.mfd");

            // Next we want to dump the blank card
            match write_card(true, &path, &dumped_card) {
                Ok(_) => println!("Successfully wrote blank card"),
                Err(_) => {
                    println!("Error: couldn't write to blank card");
                    std::process::exit(exitcode::USAGE);
                }
            }

            // remove any generated files
            match std::fs::remove_file("blank_with_uid.mfd") {
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
