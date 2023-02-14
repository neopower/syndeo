#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
mod syndeo {
    use ink::storage::Mapping;
    use ink_prelude::vec::Vec;

    #[ink(event)]
    pub struct Contribution {
        sender: AccountId,
        recipient: AccountId,
        amount: u64,
    }

    #[ink(event)]
    pub struct NewRecipient {
        recipient: AccountId,
    }

    #[ink(storage)]
    pub struct Syndeo {
        contributions: Mapping<AccountId, u64>,
        recipients: Vec<AccountId>,
        total_contributions: u64,
    }

    impl Syndeo {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                contributions: Mapping::default(),
                recipients: Vec::new(),
                total_contributions: 0,
            }
        }

        #[ink(message)]
        pub fn contribute(&mut self, recipient: AccountId, amount: u64) {
            let sender = self.env().caller();
            let mut balance = self.contributions.get(recipient).unwrap_or(0);
            balance = balance.checked_add(amount).unwrap();
            self.contributions.insert(recipient, &balance);

            self.total_contributions = self.total_contributions.checked_add(amount).unwrap();

            self.env().emit_event(Contribution {
                sender,
                recipient,
                amount,
            });

            if !self.recipients.contains(&recipient) {
                self.recipients.push(recipient);

                self.env().emit_event(NewRecipient { recipient });
            }
        }

        #[ink(message)]
        pub fn distribute_rewards(&mut self) {
            let total_reward: Balance = self.env().balance();
            for recipient in &self.recipients {
                let recipient_points = self.contributions.get(recipient).unwrap();

                // ToDo: Check the math operation
                let reward: Balance = (recipient_points as u128)
                    .checked_mul(total_reward)
                    .unwrap()
                    .checked_div(self.total_contributions as u128)
                    .unwrap();

                // ToDo: Test the expect
                self.env()
                    .transfer(*recipient, reward)
                    .expect("failed to transfer tokens");
            }

            self.clear_contributions();
        }

        #[ink(message)]
        pub fn get_funds(&self) -> Balance {
            self.env().balance()
        }

        #[ink(message)]
        pub fn get_total_contributions(&self) -> u64 {
            self.total_contributions
        }

        #[ink(message)]
        pub fn get_number_of_recipients(&self) -> u64 {
            self.recipients.len() as u64
        }

        fn clear_contributions(&mut self) {
            self.contributions = Mapping::default();
            self.recipients = Vec::new();
            self.total_contributions = 0;
        }
    }
}
