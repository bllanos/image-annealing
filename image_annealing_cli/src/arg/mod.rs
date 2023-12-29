use bpaf::Bpaf;

#[derive(Bpaf, Clone, Debug)]
#[bpaf(options, generate(make_option_parser), version)]
/// Data processing command-line interface
pub struct Options {}

#[cfg(test)]
mod tests;
