use std::{
    error::Error,
    io::{Read, Write},
};

use csv::{ReaderBuilder, Trim};

use crate::{
    bank_engine::{BankEngine, InMemoryStore},
    errors::PaymentError,
    models::{Transaction, TransactionRecord},
};

mod bank_engine;
mod client;
mod errors;
mod models;

pub fn process_transactions<R: Read, O: Write>(input: R, output: O) -> Result<(), Box<dyn Error>> {
    let mut reader = ReaderBuilder::new()
        .has_headers(true)
        .trim(Trim::All)
        .from_reader(input);
    let store = InMemoryStore::default();

    let mut bank_engine = BankEngine::new(store);

    for (index, row) in reader.deserialize::<TransactionRecord>().enumerate() {
        let record = match row {
            Err(e) => {
                let err = PaymentError::InvalidTransaction {
                    row: index,
                    details: e.to_string(),
                };
                eprintln!("Skipping invalid transaction row: {}", err);
                continue;
            }
            Ok(t) => t,
        };

        let transaction = match Transaction::try_from(record) {
            Ok(t) => t,
            Err(e) => {
                eprintln!("Skipping invalid row {}: Data error: {}", index + 1, e);
                continue;
            }
        };

        if let Err(e) = bank_engine.process_transaction(transaction) {
            eprintln!("Transaction failed: {}", e);
        }
    }

    let mut writer = csv::WriterBuilder::new()
        .has_headers(true)
        .from_writer(output);

    for row in bank_engine.generate_output() {
        if let Err(e) = writer.serialize(row) {
            eprintln!("Error writing output: {}", e);
        }
    }

    writer.flush()?;

    Ok(())
}
