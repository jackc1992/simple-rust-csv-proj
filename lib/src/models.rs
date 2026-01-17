use rust_decimal::Decimal;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum Transaction {
    Deposit {
        client: u16,
        #[serde(rename = "tx")]
        tx_id: u32,
        amount: Decimal,
    },
    Withdrawal {
        client: u16,
        #[serde(rename = "tx")]
        tx_id: u32,
        amount: Decimal,
    },
    // I'm honestly not sure if the `amount` field is included for the following three transaction
    // variants
    Dispute {
        client: u16,
        #[serde(rename = "tx")]
        tx_id: u32,
    },
    Resolve {
        client: u16,
        #[serde(rename = "tx")]
        tx_id: u32,
    },
    Chargeback {
        client: u16,
        #[serde(rename = "tx")]
        tx_id: u32,
    },
}
