#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
mod syndeo {
    use ink::storage::Mapping;
    use ink_prelude::vec::Vec;

    #[derive(PartialEq, Debug, Eq, Clone, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum ContractError {
        MemberAlreadyExists,
        MemberDoesNotExist,
        MaxPointsPerSenderCannotBeZero,
    }

    #[ink(event)]
    pub struct NewMember {
        member: AccountId,
    }

    #[ink(event)]
    pub struct MemberDeleted {
        member: AccountId,
    }

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
        admin: AccountId,
        members: Vec<AccountId>,
        points_by_sender: Mapping<AccountId, u64>,
        points_by_recipient: Mapping<AccountId, u64>,
        recipients: Vec<AccountId>,
        total_points: u64,
        max_points_per_sender: u64,
    }

    impl Syndeo {
        #[ink(constructor)]
        pub fn new(max_points_per_sender: Option<u64>) -> Self {
            Self {
                admin: Self::env().caller(),
                members: Vec::new(),
                points_by_sender: Mapping::default(),
                points_by_recipient: Mapping::default(),
                recipients: Vec::new(),
                total_points: 0,
                max_points_per_sender: max_points_per_sender.unwrap_or(10),
            }
        }

        // ONLY ADMIN
        #[ink(message)]
        pub fn add_member(&mut self, new_member: AccountId) -> Result<(), ContractError> {
            if self.members.contains(&new_member) {
                return Err(ContractError::MemberAlreadyExists);
            }

            self.members.push(new_member);
            self.env().emit_event(NewMember { member: new_member });

            Ok(())
        }

        // ONLY ADMIN
        #[ink(message)]
        pub fn remove_member(&mut self, member_to_remove: AccountId) -> Result<(), ContractError> {
            match self.members.iter().position(|m| *m == member_to_remove) {
                Some(member_index) => {
                    self.members.remove(member_index);
                    self.env().emit_event(MemberDeleted {
                        member: member_to_remove,
                    });
                }
                None => return Err(ContractError::MemberDoesNotExist),
            };

            Ok(())
        }

        // ONLY ADMIN
        #[ink(message)]
        pub fn set_max_points_per_sender(
            &mut self,
            max_points_per_sender: u64,
        ) -> Result<(), ContractError> {
            if self.max_points_per_sender == 0 {
                return Err(ContractError::MaxPointsPerSenderCannotBeZero);
            }

            self.max_points_per_sender = max_points_per_sender;

            Ok(())
        }

        // ONLY MEMBERS (Sender & Recipient)
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

        // ONLY ADMIN
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
