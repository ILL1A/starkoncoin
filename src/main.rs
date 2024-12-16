/// main.rs is an entry point to starkoncoin.
use clap::{Parser, Subcommand};
mod wallet;
mod crypto;
mod node;

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
        /// Sender's account public key
        #[arg(short, long)]
        public_key: String,
        /// Sender's account secret key
        #[arg(short, long)]
        secret_key: String,
        /// Recipient's account public key
        #[arg(short, long)]
        recipient_account: String,
        /// Node's IP address
        #[arg(short, long)]
        node_ip: String,
        /// Node's port
        #[arg(short, long)]
        node_port: u16, 
    },
    /// Command to run a node with account's public and secret keys
    Run {
        /// Account's public key
        #[arg(long)]
        public_key: String,
        /// Genesis node
        #[arg(short, long)]
        genesis: bool,
        /// Node's IP address
        #[arg(short, long)]
        addr: String,
        /// Node's port
        #[arg(short, long)]
        port: u16,
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
            node_ip,
            node_port,
        } => {}
        Commands::Run { public_key , genesis, addr, port} => {
            let options = node::NodeOptions {
                public_key: public_key,
                is_genesis: genesis,
                addr: addr,
                port: port,
            };
            let mut node = std::sync::Arc::new(Box::new(node::Node::new(options)));
            node.run();
        }
    }
}
