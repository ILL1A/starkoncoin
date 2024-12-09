/// main.rs is an entry point to starkoncoin.
use clap::{Parser, Subcommand};
mod wallet;
mod crypto;

#[derive(Parser)]
#[command(
    name = "starkon",
    version = "1.0",
    about = "A simple CLI tool for utilizing starkoncoin."
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Command to generate key pair for a new account
    GenAccount,
    /// Command to send coin's to recipient account
    Send {
        // Sender's account public key
        #[arg(short, long)]
        public_key: String,
        // Sender's account secret key
        #[arg(short, long)]
        secret_key: String,
        // Recipient's account public key
        #[arg(short, long)]
        recipient_account: String,
    },
    /// Command to run a node with account's public and secret keys
    Run {
        /// Account's public key
        #[arg(short, long)]
        public_key: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::GenAccount => {
            wallet::account::generate_account();
        }
        Commands::Send {
            public_key,
            secret_key,
            recipient_account,
        } => {}
        Commands::Run { public_key } => {}
    }
}
