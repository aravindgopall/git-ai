use rand::prelude::*;

pub fn suggest_commit_message() -> String {
    let options = vec![
        "[git-ai] cleaned config spaghetti 🍝",
        "[git-ai] squashed timestamps like a boss 🕒",
        "[git-ai] localhost chaos managed 🚀",
        "[git-ai] peace restored to your diffs ✌️",
        "[git-ai] tiny tweaks, big vibes 🎯",
        "[git-ai] silenced noisy paths 🔇",
    ];

    options.choose(&mut rand::rng()).unwrap().to_string()
}
