use rust_decimal::Decimal;

use crate::errors::AccountError;

#[derive(Default, Debug, PartialEq)]
pub struct Client {
    pub held: Decimal,
    pub total: Decimal,
    pub locked: bool,
}

impl Client {
    pub fn make_deposit(&mut self, amount: Decimal) {
        self.total += amount;
    }

    pub fn make_withdrawal(&mut self, amount: Decimal) -> Result<(), AccountError> {
        let available = self.total - self.held;
        if available >= amount {
            self.total -= amount;
            Ok(())
        } else {
            Err(AccountError::InsufficientFunds)
        }
    }

    pub fn make_dispute(&mut self, amount: Decimal) {
        self.held += amount;
    }

    pub fn resolve_transaction(&mut self, amount: Decimal) {
        self.held -= amount;
    }

    pub fn charge_back(&mut self, amount: Decimal) {
        self.held -= amount;
        self.total -= amount;
        self.locked = true;
    }
}
