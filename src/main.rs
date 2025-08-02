use clap::Parser;
use rstool::{Opts, SubCommand, process_csv, process_genpass};

fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();

    match opts.cmd {
        SubCommand::Csv(opts) => {
            process_csv(&opts.input, &opts.output, opts.format)?;
        },
        SubCommand::GenPass(opts) => {
            process_genpass(opts.length, opts.uppercase, opts.lowercase, opts.numbers, opts.symbols)?;
        },
    }

    Ok(())
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
