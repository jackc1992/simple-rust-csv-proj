use std::collections::HashMap;

use rust_decimal::Decimal;

use crate::{
    client::Client,
    errors::{AccountError, PaymentError},
    models::{ClientID, ClientOutput, Deposit, Transaction, TransactionID},
};

pub trait TransactionStore {
    fn record_deposit(&mut self, tx_id: TransactionID, client_id: ClientID, value: Decimal);
    fn get_tx_info(&mut self, tx_id: TransactionID) -> Result<&mut Deposit, AccountError>;
}

#[derive(Default)]
pub struct InMemoryStore {
    data: HashMap<TransactionID, Deposit>,
}

impl TransactionStore for InMemoryStore {
    fn record_deposit(&mut self, tx_id: TransactionID, client_id: ClientID, value: Decimal) {
        let deposit = Deposit {
            client_id,
            value,
            is_disputed: false,
        };

        self.data.insert(tx_id, deposit);
    }

    fn get_tx_info(&mut self, tx_id: TransactionID) -> Result<&mut Deposit, AccountError> {
        self.data
            .get_mut(&tx_id)
            .ok_or(AccountError::MissingTransaction)
    }
}

pub struct BankEngine<Store: TransactionStore> {
    clients: HashMap<ClientID, Client>,
    transaction_store: Store,
}

fn get_or_insert_client(
    client_id: ClientID,
    clients: &mut HashMap<ClientID, Client>,
) -> Result<&mut Client, PaymentError> {
    let client = clients.entry(client_id).or_default();
    if client.locked {
        return Err(PaymentError::AccountLocked(client_id));
    }
    Ok(client)
}

impl<Store: TransactionStore> BankEngine<Store> {
    pub fn new(store: Store) -> Self {
        Self {
            clients: HashMap::new(),
            transaction_store: store,
        }
    }

    fn get_transaction_info(
        &mut self,
        tx_id: TransactionID,
        client_id: ClientID,
    ) -> Result<(&mut Client, &mut Deposit), PaymentError> {
        let client = get_or_insert_client(client_id, &mut self.clients)?;
        let deposit = self
            .transaction_store
            .get_tx_info(tx_id)
            .map_err(|source| PaymentError::TransactionFailed {
                tx_id,
                client_id,
                source,
            })?;

        if client_id != deposit.client_id {
            Err(PaymentError::TransactionFailed {
                tx_id,
                client_id,
                source: AccountError::IncorrectTransactionId,
            })
        } else {
            Ok((client, deposit))
        }
    }

    pub fn process_transaction(&mut self, transaction: Transaction) -> Result<(), PaymentError> {
        match transaction {
            Transaction::Deposit {
                client_id,
                tx_id,
                amount,
            } => {
                get_or_insert_client(client_id, &mut self.clients)?.make_deposit(amount);

                self.transaction_store
                    .record_deposit(tx_id, client_id, amount);

                Ok(())
            }
            Transaction::Withdrawal {
                client_id,
                tx_id,
                amount,
            } => {
                let client = get_or_insert_client(client_id, &mut self.clients)?;

                client
                    .make_withdrawal(amount)
                    .map_err(|source| PaymentError::TransactionFailed {
                        tx_id,
                        client_id,
                        source,
                    })
            }
            Transaction::Dispute { client_id, tx_id } => {
                let (client, transaction_info) = self.get_transaction_info(tx_id, client_id)?;

                if transaction_info.is_disputed {
                    return Err(PaymentError::DisputeFailed {
                        tx_id,
                        client_id,
                        source: AccountError::AlreadyDisputed,
                    });
                }

                client.make_dispute(transaction_info.value);
                transaction_info.is_disputed = true;
                Ok(())
            }
            Transaction::Resolve { client_id, tx_id } => {
                let (client, transaction_info) = self.get_transaction_info(tx_id, client_id)?;
                if !transaction_info.is_disputed {
                    return Err(PaymentError::ResolveFailed {
                        tx_id,
                        client_id,
                        source: AccountError::NotDisputed,
                    });
                }

                let transaction_amount = transaction_info.value;

                client.resolve_transaction(transaction_amount);
                transaction_info.is_disputed = false;
                Ok(())
            }
            Transaction::Chargeback { client_id, tx_id } => {
                let (client, transaction_info) = self.get_transaction_info(tx_id, client_id)?;

                if !transaction_info.is_disputed {
                    return Err(PaymentError::ResolveFailed {
                        tx_id,
                        client_id,
                        source: AccountError::NotDisputed,
                    });
                }

                client.charge_back(transaction_info.value);

                Ok(())
            }
        }
    }
    pub fn generate_output(&self) -> impl Iterator<Item = ClientOutput> {
        self.clients.iter().map(|(id, c)| ClientOutput {
            client: *id,
            available: c.total - c.held,
            held: c.held,
            total: c.total,
            locked: c.locked,
        })
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_resolve_logic() {
        let store = InMemoryStore::default();
        let mut bank_engine = BankEngine::new(store);
        let t1 = Transaction::Deposit {
            client_id: ClientID(1),
            tx_id: TransactionID(1),
            amount: 100.into(),
        };

        let t2 = Transaction::Dispute {
            client_id: ClientID(1),
            tx_id: TransactionID(1),
        };

        let t3 = Transaction::Resolve {
            client_id: ClientID(1),
            tx_id: TransactionID(1),
        };

        let _ = bank_engine.process_transaction(t1);

        let _ = bank_engine.process_transaction(t2);

        {
            let client = bank_engine.clients.get(&ClientID(1)).unwrap();
            let client_target = Client {
                held: 100.into(),
                total: 100.into(),
                locked: false,
            };
            assert_eq!(client, &client_target);
        }

        let _ = bank_engine.process_transaction(t3);

        let client = bank_engine.clients.get(&ClientID(1)).unwrap();
        let client_target = Client {
            held: 0.into(),
            total: 100.into(),
            locked: false,
        };
        assert_eq!(client, &client_target);
    }
}
