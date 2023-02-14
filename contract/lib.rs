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
        points_by_recipient: Mapping<AccountId, u64>,
        recipients: Vec<AccountId>,
        total_points: u64,
    }

    impl Syndeo {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                points_by_recipient: Mapping::default(),
                recipients: Vec::new(),
                total_points: 0,
            }
        }

        #[ink(message)]
        pub fn award(&mut self, recipient: AccountId, amount: u64) {
            let sender = self.env().caller();
            let mut recipient_points = self.points_by_recipient.get(recipient).unwrap_or(0);
            recipient_points = recipient_points.checked_add(amount).unwrap();
            self.points_by_recipient
                .insert(recipient, &recipient_points);

            self.total_points = self.total_points.checked_add(amount).unwrap();

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
                let recipient_points = self.points_by_recipient.get(recipient).unwrap();

                // ToDo: Check the math operation
                let reward: Balance = (recipient_points as u128)
                    .checked_mul(total_reward)
                    .unwrap()
                    .checked_div(self.total_points as u128)
                    .unwrap();

                // ToDo: Test the expect
                self.env()
                    .transfer(*recipient, reward)
                    .expect("failed to transfer tokens");
            }

            self.reset_points();
        }

        #[ink(message)]
        pub fn get_funds(&self) -> Balance {
            self.env().balance()
        }

        #[ink(message)]
        pub fn get_total_points(&self) -> u64 {
            self.total_points
        }

        #[ink(message)]
        pub fn get_number_of_recipients(&self) -> u64 {
            self.recipients.len() as u64
        }

        fn reset_points(&mut self) {
            self.points_by_recipient = Mapping::default();
            self.recipients = Vec::new();
            self.total_points = 0;
        }
    }
}
