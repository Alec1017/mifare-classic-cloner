use std::process::Command;
use std::path::PathBuf;

extern crate exitcode;


/// Writes the given .mfd file to the destinatin .mfd file
pub fn write_card(is_blank: bool, source_file: &PathBuf, dest_file: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {  

  // Need to store Command::new into its own variable first
  let mut command = Command::new("nfc-mfclassic");

  // Determine write option depending on type of card received
  if is_blank {
      command.arg("W");
  } else {
      command.arg("w");
  }

  // Add remaining arguments
  let write_command = command
                      .arg("a")
                      .arg(source_file)
                      .arg(dest_file)
                      .output();

  // Check for errors
  if let Err(_) = write_command {
      eprintln!("Error: couldn't write to card");
      std::process::exit(exitcode::USAGE);
  }

  Ok(())
}


/// Dumps the contents of a card into a .mfd with the given output name
pub fn dump_card(key_file: Option<&PathBuf>, output_file_name: &str) -> Result<(), Box<dyn std::error::Error>> {   
    
  // Need to store Command::new into its own variable first
  let mut command = Command::new("mfoc");
  command.arg("-O")
         .arg(format!("{}", output_file_name));

  // Add the key file option, if it exists
  if let Some(file) = key_file {
      command.arg("-k").arg(file);
  }

  let dump_command = command.output();

  // Check for errors
  if let Err(_) = dump_command {
      eprintln!("Error: couldn't dump contents of card");
      std::process::exit(exitcode::USAGE);
  }

  Ok(())
}