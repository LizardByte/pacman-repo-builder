use argh::FromArgs;

#[derive(Debug, FromArgs)]
#[argh(subcommand, name = "copyright", description = "Show license")]
pub struct CopyrightArgs {}
