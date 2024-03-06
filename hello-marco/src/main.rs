#[derive(Parser)]
#[clap(version = "1.0", author = "Doug Copeland", about = "A Marco Polo Game")]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,    
}

#[derive(Parser)]
enum Commands {
    #[clap(version = "1.0", author = "Doug Copeland")]
    Play {
        #[clap(short, long)]
        name: String,
    }
}

fn main() {
    let args = Cli::parse();
    match args.command {
        Some(Commands::Play { name }) => {
            let result = marco_polo(&name);
            println!("{}", result);
        }
        None => {
            println!("What is your name?");
        }
    }
}