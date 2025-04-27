#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
pub mod kudosjar {

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    pub enum WithdrawError {
        NotCreator,
        TransferFailed,
    }

    #[ink(storage)]
    pub struct KudosJar {
        creator: AccountId,
        balance: Balance,
    }

    impl KudosJar {
        #[ink(constructor)]
        pub fn new(creator: AccountId) -> Self {
            Self {
                creator,
                balance: 0,
            }
        }

        #[ink(message, payable)]
        pub fn tip(&mut self) {
            let payment = self.env().transferred_value();
            self.balance += payment;
            self.env().emit_event(Tipped {
                from: self.env().caller(),
                amount: payment,
            });
        }

        #[ink(message)]
            pub fn withdraw(&mut self) -> Result<(), WithdrawError> {
            let caller = self.env().caller();
            if caller != self.creator {
                return Err(WithdrawError::NotCreator);
            }
            let amount = self.balance;
            self.balance = 0;
            self.env()
                .transfer(self.creator, amount)
                .map_err(|_| WithdrawError::TransferFailed)
        }


        #[ink(message)]
        pub fn get_balance(&self) -> Balance {
            self.balance
        }
    }

    #[ink(event)]
    pub struct Tipped {
        #[ink(topic)]
        from: AccountId,
        amount: Balance,
    }
}