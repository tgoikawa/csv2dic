use async_std::task::block_on;
use structopt::{clap, StructOpt};

#[derive(Debug, StructOpt)]
#[structopt(name = "csv2dic", about = "Make word dictionary from csv file.")]
#[structopt(long_version(option_env!("LONG_VERSION").unwrap_or(env!("CARGO_PKG_VERSION"))))]
#[structopt(setting(clap::AppSettings::ColoredHelp))]
struct Opts {
    #[structopt(short = "i", long = "input")]
    input: String,

    #[structopt(short = "o", long = "output")]
    output: String,
}
fn main() {
    let args = Opts::from_args();
    match block_on(csv2dic::convert_to_file(args.input, args.output)) {
        Ok(_) => {}
        Err(err) => {
            println!("{}", err);
        }
    }
}
