use colored::*;

pub fn show_banner() {
    println!(
        "{}",
        "
   ____ _ _    _      
  / ___(_) | _(_)_ __ 
 | |  _| | |/ / | '_ \\
 | |_| | |   <| | | | |
  \\____|_|_|\\_\\_|_| |_|
         git-ai
Clean diffs. Smart commits. AI magic.
    "
        .bright_cyan()
    );
}
