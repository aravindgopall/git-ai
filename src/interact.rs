use crate::hunk::split_diff_into_hunks;
use crate::utils::show_in_pager;

pub fn start_interactive_review(diff: String) {
    let (_header, hunks) = split_diff_into_hunks(&diff);

    for (_, hunk) in hunks.iter().enumerate() {
        show_in_pager(hunk);
    }
}
