mod cli;
use structopt::StructOpt;

fn main() {
    pringln!("{:#?}", cli::CommandLineArgs::from_args());
}