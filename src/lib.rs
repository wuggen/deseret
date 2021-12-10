#[macro_use]
extern crate snafu;

use std::fs;
use std::fs::File;
use std::io;
use std::io::BufWriter;
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;

use snafu::IntoError;
use snafu::ResultExt;
use structopt::StructOpt;

#[doc(inline)]
pub use error::{Error, Result};

pub mod deseret_ipa;

pub fn run() -> Result<()> {
    let options = Options::from_args();
    let input = options.get_input()?;
    options.write_output(deseret_ipa::translate_iter(&input))
}

/// A simple translator from the Deseret script to IPA.
#[derive(Debug, Clone, StructOpt)]
pub struct Options {
    /// The file in which to store the output. If absent, writes the output to standard output.
    #[structopt(short, long)]
    pub out: Option<PathBuf>,

    /// The file from which to read input. If absent, reads from standard input.
    #[structopt(short, long = "in")]
    pub in_: Option<PathBuf>,

    /// Input text. Cannot be given if '--in' is specified.
    pub text: Option<String>,
}

impl Options {
    fn get_input(&self) -> Result<String> {
        if self.in_.is_some() && self.text.is_some() {
            Err(Error::ConflictingInput)
        } else if let Some(text) = &self.text {
            Ok(text.clone())
        } else if let Some(file) = &self.in_ {
            fs::read_to_string(&file).context(error::FileRead { file })
        } else {
            let mut s = String::new();
            io::stdin()
                .read_to_string(&mut s)
                .context(error::StdinRead)?;
            Ok(s)
        }
    }

    fn write_output<'a>(&self, fragments: impl IntoIterator<Item = &'a str>) -> Result<()> {
        let mut stream: Box<dyn Write> = if let Some(file) = &self.out {
            Box::new(BufWriter::new(
                File::create(file).context(error::OpenOutput { file })?,
            ))
        } else {
            Box::new(io::stdout())
        };

        for fragment in fragments.into_iter() {
            stream.write(fragment.as_bytes()).map_err(|e| {
                error::WriteOutput {
                    file: self.out.clone(),
                }
                .into_error(e)
            })?;
        }

        Ok(())
    }
}

pub mod error {
    use std::io;
    use std::path::PathBuf;

    #[derive(Debug, Snafu)]
    #[snafu(visibility = "pub(crate)")]
    pub enum Error {
        #[snafu(display("both input file and in-line text were given; choose one"))]
        ConflictingInput,

        #[snafu(display("error reading input file {}", file.display()))]
        FileRead { file: PathBuf, source: io::Error },

        #[snafu(display("error reading from standard input"))]
        StdinRead { source: io::Error },

        #[snafu(display("error writing to {}", if let Some(path) = file {
            path.display().to_string()
        } else {
            "standard output".to_string()
        }))]
        WriteOutput {
            file: Option<PathBuf>,
            source: io::Error,
        },

        #[snafu(display("error opening output file {}", file.display()))]
        OpenOutput { file: PathBuf, source: io::Error },
    }

    pub type Result<T> = std::result::Result<T, Error>;
}
