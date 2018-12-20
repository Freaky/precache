use std::fs::File;
use std::io::{copy, sink};

use ignore::WalkBuilder;

fn main() -> Result<(), String> {
    let mut builder = WalkBuilder::new(std::env::args_os().nth(1).ok_or_else(|| {
        format!(
            "Usage: {} [path [path [...]]]",
            std::env::args().nth(0).expect("WHAT AM I?!")
        )
    })?);
    builder.standard_filters(false);

    for path in std::env::args_os().skip(2) {
        builder.add(path);
    }

    builder.build_parallel().run(|| {
        Box::new(|entry| {
            if let Ok(entry) = entry {
                if let Ok(mut file) = File::open(entry.path()) {
                    let _ = copy(&mut file, &mut sink());
                }
            }
            ignore::WalkState::Continue
        })
    });

    Ok(())
}
