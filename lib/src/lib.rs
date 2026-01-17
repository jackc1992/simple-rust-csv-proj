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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_deposit_withdrawal() {
        let input = "\
type,       client, tx, amount
deposit,    1,      1,  10.0
deposit,    2,      2,  20.0
withdrawal, 1,      3,  5.0
withdrawal, 2,      4,  30.0
";
        let mut output = Vec::new();

        process_transactions(input.as_bytes(), &mut output).expect("Should not fail");

        let result = String::from_utf8(output).expect("Output should be valid UTF-8");

        assert!(result.contains("1,5,0,5,false"));
        assert!(result.contains("2,20,0,20,false"));
    }

    #[test]
    fn test_dispute_and_chargeback() {
        let input = "\
type,       client, tx, amount
deposit,    1,      1,  100.0
dispute,    1,      1,
chargeback, 1,      1,
";
        let mut output = Vec::new();

        process_transactions(input.as_bytes(), &mut output).expect("Should not fail");

        let result = String::from_utf8(output).unwrap();

        assert!(result.contains("1,0,0,0,true"));
    }

    #[test]
    fn test_security_cross_client_dispute() {
        let input = "\
type,       client, tx, amount
deposit,    1,      1,  100.0
dispute,    2,      1,
";
        let mut output = Vec::new();

        process_transactions(input.as_bytes(), &mut output).unwrap();

        let result = String::from_utf8(output).unwrap();

        assert!(result.contains("1,100,0,100,false"));
    }
}
