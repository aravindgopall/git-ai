use colored::*;

pub fn show_banner() {
    println!(
        "{}",
        "
   ____   _   ___    ___     _ 
  / ___| ( ) |___|  / __\\  (_) 
 | |  _  | |  | |  | |  | | | |
 | |_| | | |  | |  | |__| | | |
  \\___| |_|  |_|  |_|  |_| |_|
         git-ai
Clean diffs. Smart commits. AI magic.
    "
        .bright_cyan()
    );
}
