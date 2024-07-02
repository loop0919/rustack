pub mod rustack;

use crate::rustack::parse::{parse_batch, parse_interactive};
use std::io::BufReader;

fn main() {
    if let Some(f) = std::env::args()
        .nth(1)
        .and_then(|f| std::fs::File::open(f).ok())
        {
            parse_batch(BufReader::new(f));
        } else {
            parse_interactive();
        }
}
