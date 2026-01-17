use std::{error::Error, io::Read};

use crate::models::Transaction;

mod models;

pub fn process_transactions<R: Read>(input: R) -> Result<(), Box<dyn Error>> {
    let mut reader = csv::Reader::from_reader(input);

    for row in reader.deserialize::<Transaction>() {
        match row? {
            Transaction::Deposit {
                client,
                tx_id,
                amount,
            } => todo!(),
            Transaction::Withdrawal {
                client,
                tx_id,
                amount,
            } => todo!(),
            Transaction::Dispute { client, tx_id } => todo!(),
            Transaction::Resolve { client, tx_id } => todo!(),
            Transaction::Chargeback { client, tx_id } => todo!(),
        }
    }
    Ok(())
}
