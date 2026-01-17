use thiserror::Error;

use crate::models::{ClientID, TransactionID};

#[derive(Debug, Error)]
pub enum AccountError {
    #[error("insufficient funds")]
    InsufficientFunds,

    #[error("Transaction does not exist")]
    MissingTransaction,

    #[error("Transaction is already disputed")]
    AlreadyDisputed,

    #[error("Transaction is not disputed")]
    NotDisputed,

    #[error("Invalid client id for associated transaction id")]
    IncorrectTransactionId,
}

#[derive(Error, Debug)]
pub enum PaymentError {
    #[error("Invalid transaction format in row {row}: {details}")]
    InvalidTransaction { row: usize, details: String },

    #[error("Attempted to operate on locked account: client {0}")]
    AccountLocked(ClientID),

    #[error("Transaction {tx_id} failed for client {client_id}: {source}")]
    TransactionFailed {
        tx_id: TransactionID,
        client_id: ClientID,
        #[source]
        source: AccountError,
    },

    #[error("Dispute for {tx_id} failed for client {client_id}: {source}")]
    DisputeFailed {
        tx_id: TransactionID,
        client_id: ClientID,
        #[source]
        source: AccountError,
    },

    #[error("Resolving{tx_id} failed for client {client_id}")]
    ResolveFailed {
        tx_id: TransactionID,
        client_id: ClientID,
        #[source]
        source: AccountError,
    },
}
