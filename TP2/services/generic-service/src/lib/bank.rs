use crate::{
    paths::BANK_ACCOUNTS,
    types::{
        common::BoxResult,
        data::{Action, Transaction},
    },
};
use serde::Deserialize;
use std::{collections::HashMap, convert::TryInto};

type Account = u32;

#[derive(Debug, Deserialize)]
struct AccountRecord {
    cbu: Account,
    money: i32,
}

pub struct Bank {
    accounts_money: HashMap<Account, i32>,
}

impl Bank {
    pub fn new() -> BoxResult<Self> {
        let mut bank = Bank {
            accounts_money: HashMap::new(),
        };

        bank.parse_accounts()?;

        Ok(bank)
    }

    fn parse_accounts(&mut self) -> BoxResult<()> {
        let mut accounts_file = csv::Reader::from_path(BANK_ACCOUNTS)?;

        for result in accounts_file.deserialize() {
            let record: AccountRecord = result?;
            self.accounts_money.insert(record.cbu, record.money);
        }

        Ok(())
    }

    pub fn reserve_resources(&mut self, tx: &Transaction) -> BoxResult<Action> {
        let account_money = self.accounts_money.get(&tx.cbu).unwrap_or(&0);
        let required_money = (tx.airline_cost + tx.hotel_cost).try_into()?;

        if *account_money < required_money {
            println!(
                "[tx {}] insufficient money => bank loan - required: {} available: {}",
                tx.id, required_money, *account_money
            );
            // WONTDO: Reject transaction [we must add another message]
            // for now, we allow "bank loans" (negative money)
        }

        let account_left_money = *account_money - required_money;
        println!(
            "[tx {}] taking {} from cbu {} - money left: {}",
            tx.id, required_money, tx.cbu, account_left_money
        );

        self.accounts_money.insert(tx.cbu, account_left_money);

        Ok(Action::Prepare)
    }

    pub fn release_resources(&mut self, tx: &Transaction) -> BoxResult<()> {
        let account_money = self.accounts_money.get_mut(&tx.cbu).ok_or(format!(
            "[tx {}] ERROR: account with cbu {} should have been created",
            tx.id, tx.cbu
        ))?;

        // restore money to account (it should have been reserved -substracted-)
        let money_to_release: i32 = (tx.airline_cost + tx.hotel_cost).try_into()?;
        *account_money += money_to_release;
        println!(
            "[tx {}] releasing {} to cbu {} - money available: {}",
            tx.id, money_to_release, tx.cbu, account_money
        );

        Ok(())
    }

    pub fn consume_resources(&mut self, tx: &Transaction) -> BoxResult<()> {
        // do nothing, since resources were already substracted from account
        println!(
            "[tx {}] confirming extraction of {} from cbu {}",
            tx.id,
            tx.airline_cost + tx.hotel_cost,
            tx.cbu
        );
        Ok(())
    }
}
