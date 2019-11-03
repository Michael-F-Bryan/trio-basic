use anyhow::{Context, Error};
use codespan::Files;
use slog::{Drain, Level, Logger};
use std::path::PathBuf;
use structopt::StructOpt;
use trio_basic::{Callback, Project};

fn main() {
    let args = Args::from_args();
    let logger = args.logger();

    slog::info!(logger, "Starting application";
        "args" => format_args!("{:?}", args));

    if let Err(e) = run(&args, &logger) {
        slog::error!(logger, "Unable to load the input files"; 
                "error" => e.to_string());

        for cause in e.chain().skip(1) {
            slog::warn!(logger, "Caused by: {}", cause);
        }
        drop(logger);

        let bt = e.backtrace().to_string();
        // HACK: workaround `std::backtrace::Backtrace::status()` is unstable
        if bt != "disabled backtrace" && bt != "unsupported backtrace" {
            eprintln!("{}", bt);
        }

        std::process::exit(1);
    }
}

fn run(args: &Args, logger: &Logger) -> Result<(), Error> {
    let project = args
        .parse_input_files()
        .context("Unable to load the input files")?;

    trio_basic::compile(
        project,
        logger.clone(),
        &mut Nop,
        &trio_basic::diagnostics::PanicReporter,
    );

    Ok(())
}

#[derive(Debug, StructOpt)]
pub struct Args {
    #[structopt(help = "The files to compile")]
    input_files: Vec<PathBuf>,
    #[structopt(
        short = "v",
        long = "verbose",
        parse(from_occurrences),
        help = "Emit more verbose output"
    )]
    verbosity: usize,
}

impl Args {
    pub fn logger(&self) -> Logger {
        let decorator = slog_term::TermDecorator::new().build();
        let drain = slog_term::CompactFormat::new(decorator).build().fuse();
        let drain = slog_async::Async::new(drain).build().fuse();

        let level = match self.verbosity {
            0 => Level::Info,
            1 => Level::Debug,
            _ => Level::Trace,
        };
        let drain = drain.filter_level(level).fuse();

        Logger::root(drain, slog::o!())
    }

    pub fn parse_input_files(&self) -> Result<Project, Error> {
        let mut source_code = Files::new();
        let mut files = Vec::new();

        for filename in &self.input_files {
            let src = std::fs::read_to_string(filename).with_context(|| {
                format!("Unable to read \"{}\"", filename.display())
            })?;
            let id = source_code.add(filename.display().to_string(), src);
            files.push(id);
        }

        Ok(Project { files, source_code })
    }
}

struct Nop;

impl Callback for Nop {}
