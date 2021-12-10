use std::error::Error;

fn main() {
    if let Err(err) = deseret::run() {
        eprintln!("Error: {}", err);

        let mut source = err.source();
        while let Some(err) = source {
            eprintln!("Note: caused by {}", err);
            source = err.source();
        }
    }
}
