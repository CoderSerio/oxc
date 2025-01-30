use std::process::{ExitCode, Termination};

#[derive(Debug)]
pub enum CliRunResult {
    None,
    InvalidOptions {
        message: String,
    },
    /// The exit unix code for, in general 0 or 1 (from `--deny-warnings` or `--max-warnings` for example)
    LintResult(ExitCode),
    PrintConfigResult,
    ConfigFileInitFailed,
    ConfigFileInitSucceeded,
}

impl Termination for CliRunResult {
    #[allow(clippy::print_stdout, clippy::print_stderr)]
    fn report(self) -> ExitCode {
        match self {
            Self::None | Self::PrintConfigResult | Self::ConfigFileInitSucceeded => {
                ExitCode::SUCCESS
            }
            Self::ConfigFileInitFailed => ExitCode::FAILURE,
            Self::InvalidOptions { message } => {
                println!("Invalid Options: {message}");
                ExitCode::FAILURE
            }
            Self::LintResult(exit_code) => exit_code,
        }
    }
}
