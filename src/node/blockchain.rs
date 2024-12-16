use super::transaction;

#[derive(Hash, Clone)]
struct Block {
    index: u64,
    timestamp: u64,
    transactions: Vec<transaction::Transaction>,
    prev_hash: Vec<u8>,
    hash: Vec<u8>,
    nonce: u64,
}

pub(crate) struct Blockchain {
    // TODO: Handle multiple temporary chains
    chain: Vec<Block>,
    current_transactions: Vec<transaction::Transaction>,
}

impl Blockchain {
	pub(crate) fn new() -> Self {
		let chain = Vec::new();
		let current_transactions = Vec::new();
		Blockchain {
			chain,
			current_transactions,
		}
	}

	pub(crate) fn create_genesis_block(&mut self) {
		let block = Block {
			index: 1,
			timestamp: 0,
			transactions: Vec::new(),
			prev_hash: Vec::new(),
			hash: Vec::new(),
			nonce: 0,
		};
		self.chain.push(block);
	}

	pub(crate) fn get_blockchain(&mut self) {
		// TODO: Get blockchain from the network
	}
}
