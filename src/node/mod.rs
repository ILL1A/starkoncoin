use std::future::IntoFuture;

pub mod blockchain;
pub mod transaction;
pub mod net;

pub struct NodeOptions {
  pub public_key: String,
  pub is_genesis: bool,
  pub addr: String,
  pub port: u16,
}

pub struct Node {
  options: NodeOptions,
  blockchain: blockchain::Blockchain,
  listener: tokio::sync::RwLock<Option<tokio::task::JoinHandle<()>>>,
}

impl Node {
  pub fn new(options: NodeOptions) -> Self {
    Self {
      options: options,
      blockchain: blockchain::Blockchain::new(),
      listener: tokio::sync::RwLock::new(None),
    }
  }

  pub async fn setup(self: &mut std::sync::Arc<Box<Self>>) {
    println!("Setting up node with public key: {}", self.options.public_key);

    match self.options.is_genesis {
      true => {
        println!("Setting up genesis node...");
        std::sync::Arc::<Box<Self>>::get_mut( self).unwrap().blockchain.create_genesis_block();
      },
      false => {
        println!("Getting nodes from the network...");
        std::sync::Arc::<Box<Self>>::get_mut(self).unwrap().blockchain.get_blockchain();
      }
    }

    self.run_node_listener().await;
    println!("Node setup complete!");
  }

  async fn wait_for_end(self: &mut std::sync::Arc<Box<Self>>) {
    if let Some(handle) = self.listener.write().await.take() {
      if let Err(e) = handle.into_future().await {
          eprintln!("Error awaiting future: {}", e);
      }
    }
  }

  async fn run_node_listener(self: &mut std::sync::Arc<Box<Self>>) {
    let self_clone = self.clone();
    let handle = tokio::spawn(async move {
      if let Err(e) = self_clone.run_server().await {
        eprintln!("Error running server: {}", e);
      }
    });
    *self.listener.write().await = Some(handle);
    println!("Node listener running!");
  }

  #[tokio::main]
  pub async fn run(self: &mut std::sync::Arc<Box<Self>>) {
    self.setup().await;

    self.wait_for_end().await;
  }
}