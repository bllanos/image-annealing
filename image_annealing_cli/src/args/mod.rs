use crate::config::{self, Config};
pub use bpaf::ParseFailure;
use bpaf::{Bpaf, ShellComp};

#[derive(Debug, Bpaf)]
#[bpaf(generate(make_option_parser), options, version)]
/// Run individual operations
struct Options {
    /// Path of the configuration file describing the operation to run
    #[bpaf(long, short, argument::<String>("CONFIG_FILE"), complete_shell(ShellComp::File { mask: Some("*.json") }), parse(config::parse_config_file))]
    config: Config,
}

pub fn parse_args<T>(args: T) -> Result<Config, ParseFailure>
where
    T: IntoIterator<Item = String>,
{
    let mut args_iter = args.into_iter();
    args_iter
        .next()
        .expect("no arguments (not even the program name)");
    let args_vector = args_iter.collect::<Vec<String>>();
    make_option_parser()
        .run_inner(
            args_vector
                .iter()
                .map(|s| s.as_ref())
                .collect::<Vec<&str>>()
                .as_slice()
                .into(),
        )
        .map(|opts| opts.config)
}

#[cfg(test)]
mod tests;
