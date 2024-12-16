#[derive(Hash, Clone)]
pub(crate) struct Transaction {
  sender: String,
  recipient: String,
  amount: u64,
  sender_index: u64,
}