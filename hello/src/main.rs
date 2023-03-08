//A command-line tool to play Marco Polo
use clap::Parser;

#[derive(Parser)]
#[clap(
    version = "1.0",
    author = "Yikai Liu",
    about = "Check if the input is palindrome"
)]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Parser)]
enum Commands {
    #[clap(version = "1.0", author = "Yikai Liu")]
    Check {
        #[clap(short, long)]
        input: String,
    },
}

fn main() {
    let args = Cli::parse();
    match args.command {
        Some(Commands::Check { input    }) => {
            let result = hello::restore_ip_addresses(input);
            // print result which is a Vec<Vec<i32>>
            println!("{:?}", result);
            
        }
        None => println!("No subcommand was used"),
    }
}