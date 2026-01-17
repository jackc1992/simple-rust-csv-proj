use std::fmt::Display;

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Hash, PartialEq, Eq, Clone, Copy, Serialize)]
pub struct ClientID(pub u16);

impl Display for ClientID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Deserialize, Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub struct TransactionID(pub u32);

impl Display for TransactionID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Deserialize)]
pub struct TransactionRecord {
    #[serde(rename = "type")]
    pub r#type: String,

    #[serde(rename = "client")]
    pub client_id: ClientID,

    #[serde(rename = "tx")]
    pub tx_id: TransactionID,

    // Use Option because 'dispute' rows might have empty amounts
    pub amount: Option<Decimal>,
}

#[derive(Debug)]
pub enum Transaction {
    Deposit {
        client_id: ClientID,
        tx_id: TransactionID,
        amount: Decimal,
    },
    Withdrawal {
        client_id: ClientID,
        tx_id: TransactionID,
        amount: Decimal,
    },
    Dispute {
        client_id: ClientID,
        tx_id: TransactionID,
    },
    Resolve {
        client_id: ClientID,
        tx_id: TransactionID,
    },
    Chargeback {
        client_id: ClientID,
        tx_id: TransactionID,
    },
}

impl TryFrom<TransactionRecord> for Transaction {
    type Error = String;

    fn try_from(record: TransactionRecord) -> Result<Self, Self::Error> {
        match record.r#type.as_str() {
            "deposit" => Ok(Transaction::Deposit {
                client_id: record.client_id,
                tx_id: record.tx_id,
                // Ensure amount exists for deposits
                amount: record.amount.ok_or("Missing amount for deposit")?,
            }),
            "withdrawal" => Ok(Transaction::Withdrawal {
                client_id: record.client_id,
                tx_id: record.tx_id,
                amount: record.amount.ok_or("Missing amount for withdrawal")?,
            }),
            "dispute" => Ok(Transaction::Dispute {
                client_id: record.client_id,
                tx_id: record.tx_id,
            }),
            "resolve" => Ok(Transaction::Resolve {
                client_id: record.client_id,
                tx_id: record.tx_id,
            }),
            "chargeback" => Ok(Transaction::Chargeback {
                client_id: record.client_id,
                tx_id: record.tx_id,
            }),
            other => Err(format!("Unknown transaction type: {}", other)),
        }
    }
}

pub struct Deposit {
    pub client_id: ClientID,
    pub value: Decimal,
    pub is_disputed: bool,
}

#[derive(Debug, Serialize)]
pub struct ClientOutput {
    pub client: ClientID,
    pub available: Decimal,
    pub held: Decimal,
    pub total: Decimal,
    pub locked: bool,
}
