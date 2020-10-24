pub fn find_matches(content: &str, pattern: &str, mut writer: impl std::io::Write) {
  for line in content.lines() {
    if line.contains(pattern) {
      if let Err(e) = writeln!(writer, "{}", line) {
        println!("Writing error: {}", e.to_string());
      }
    }
  }
}