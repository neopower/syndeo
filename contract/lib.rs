#![cfg_attr(not(feature = "std"), no_std)]

pub mod errors;

#[cfg(test)]
pub mod tests;

#[ink::contract]
mod syndeo {
    use crate::errors::ContractError;

    use ink::storage::Mapping;
    use ink_prelude::vec::Vec;

    #[derive(PartialEq, Debug, Eq, Clone, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub struct RewardsSummary {
        assigned_points: u64,
        members_awarded: u64,
        funds: Balance,
    }

    #[ink(event)]
    pub struct AdminChanged {
        admin: AccountId,
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
    pub struct Award {
        sender: AccountId,
        recipient: AccountId,
        points: u64,
    }

    #[ink(event)]
    pub struct RewardGranted {
        recipient: AccountId,
        reward: Balance,
        points: u64,
    }

    #[ink(storage)]
    pub struct Syndeo {
        pub admin: AccountId,
        pub members: Vec<AccountId>,
        pub points_by_sender: Mapping<AccountId, u64>,
        pub senders: Vec<AccountId>,
        pub points_by_recipient: Mapping<AccountId, u64>,
        pub recipients: Vec<AccountId>,
        pub total_points: u64,
        pub max_points_per_sender: u64,
    }

    impl Syndeo {
        #[ink(constructor)]
        pub fn new(max_points_per_sender: Option<u64>) -> Self {
            let caller = Self::env().caller();
            let mut members = Vec::new();
            members.push(caller);

            Self {
                admin: caller,
                members,
                points_by_sender: Mapping::default(),
                senders: Vec::new(),
                points_by_recipient: Mapping::default(),
                recipients: Vec::new(),
                total_points: 0,
                max_points_per_sender: max_points_per_sender.unwrap_or(10),
            }
        }

        #[ink(message)]
        pub fn set_admin(&mut self, new_admin: AccountId) -> Result<(), ContractError> {
            self.check_admin()?;

            if !self.is_member(&new_admin) {
                self.add_member(new_admin)?;
            }

            self.admin = new_admin;

            self.env().emit_event(AdminChanged { admin: new_admin });

            Ok(())
        }

        #[ink(message)]
        pub fn add_member(&mut self, new_member: AccountId) -> Result<(), ContractError> {
            self.check_admin()?;

            if self.members.contains(&new_member) {
                return Err(ContractError::MemberAlreadyExists);
            }

            self.members.push(new_member);
            self.env().emit_event(NewMember { member: new_member });

            Ok(())
        }

        #[ink(message)]
        pub fn remove_member(&mut self, member_to_remove: AccountId) -> Result<(), ContractError> {
            self.check_admin()?;

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

        #[ink(message)]
        pub fn set_max_points_per_sender(
            &mut self,
            max_points_per_sender: u64,
        ) -> Result<(), ContractError> {
            self.check_admin()?;

            if self.max_points_per_sender == 0 {
                return Err(ContractError::MaxPointsPerSenderCannotBeZero);
            }

            self.max_points_per_sender = max_points_per_sender;

            Ok(())
        }

        #[ink(message)]
        pub fn award(&mut self, recipient: AccountId, points: u64) -> Result<(), ContractError> {
            let sender = self.env().caller();
            self.validate_award_inputs(sender, recipient, points)?;

            let sender_used_points = self.points_by_sender.get(sender).unwrap_or(0);
            self.validate_sender_points(sender_used_points, points)?;
            let sender_used_points_result = sender_used_points.checked_add(points).unwrap();
            self.points_by_sender
                .insert(sender, &sender_used_points_result);

            let recipient_points = self.points_by_recipient.get(recipient).unwrap_or(0);
            let recipient_points_result = recipient_points.checked_add(points).unwrap();
            self.points_by_recipient
                .insert(recipient, &recipient_points_result);

            self.total_points = self.total_points.checked_add(points).unwrap();

            self.env().emit_event(Award {
                sender,
                recipient,
                points,
            });

            self.update_senders_and_recipients(sender, recipient);

            Ok(())
        }

        #[ink(message)]
        pub fn distribute_rewards(
            &mut self,
            amount_to_distribute: Option<Balance>,
        ) -> Result<(), ContractError> {
            self.check_admin()?;
            self.validate_recipients_exist()?;

            let total_rewards = self.validate_reward(amount_to_distribute)?;

            for recipient in &self.recipients {
                let recipient_points = self.points_by_recipient.get(recipient).unwrap();
                self.points_by_recipient.remove(recipient);

                let recipient_reward: Balance = (recipient_points as u128)
                    .checked_mul(total_rewards)
                    .unwrap()
                    .checked_div(self.total_points as u128)
                    .unwrap();

                self.env()
                    .transfer(*recipient, recipient_reward)
                    .expect("failed to transfer tokens");

                self.env().emit_event(RewardGranted {
                    recipient: *recipient,
                    reward: recipient_reward,
                    points: recipient_points,
                });
            }

            self.reset_points();

            Ok(())
        }

        #[ink(message)]
        pub fn get_rewards_summary(&self) -> RewardsSummary {
            RewardsSummary {
                assigned_points: self.total_points,
                members_awarded: self.recipients.len() as u64,
                funds: self.env().balance(),
            }
        }

        #[ink(message)]
        pub fn get_sender_available_points(&self) -> u64 {
            let sender_points = self.points_by_sender.get(self.env().caller()).unwrap_or(0);
            self.max_points_per_sender
                .checked_sub(sender_points)
                .unwrap_or(0)
        }

        #[ink(message)]
        pub fn get_max_points_per_sender(&self) -> u64 {
            self.max_points_per_sender
        }

        pub fn validate_sender_points(
            &self,
            sender_used_points: u64,
            new_amount: u64,
        ) -> Result<(), ContractError> {
            if sender_used_points.checked_add(new_amount).unwrap() > self.max_points_per_sender {
                return Err(ContractError::MaxPointsPerSenderExceeded);
            }

            Ok(())
        }

        pub fn update_senders_and_recipients(&mut self, sender: AccountId, recipient: AccountId) {
            if !self.senders.contains(&sender) {
                self.senders.push(sender);
            }

            if !self.recipients.contains(&recipient) {
                self.recipients.push(recipient);
            }
        }

        pub fn validate_award_inputs(
            &self,
            sender: AccountId,
            recipient: AccountId,
            amount: u64,
        ) -> Result<(), ContractError> {
            if amount == 0 {
                return Err(ContractError::AwardPointsMustBeGreaterThanZero);
            }

            if sender == recipient {
                return Err(ContractError::CannotAwardYourself);
            }

            self.check_valid_member(&sender, &recipient)?;

            Ok(())
        }

        pub fn validate_recipients_exist(&self) -> Result<(), ContractError> {
            if self.recipients.len() == 0 {
                return Err(ContractError::NoRecipients);
            }

            Ok(())
        }

        pub fn validate_reward(&self, reward: Option<Balance>) -> Result<Balance, ContractError> {
            let contract_balance = self.env().balance();
            let reward: Balance = reward.unwrap_or(contract_balance);

            if reward == 0 {
                return Err(ContractError::NullFunds);
            }

            if reward > contract_balance {
                return Err(ContractError::RewardExceedsContractBalance);
            }

            Ok(reward)
        }

        pub fn reset_points(&mut self) {
            for sender in &self.senders {
                self.points_by_sender.remove(sender);
            }

            self.senders = Vec::new();
            self.recipients = Vec::new();
            self.total_points = 0;
        }

        pub fn check_admin(&self) -> Result<(), ContractError> {
            if self.env().caller() != self.admin {
                return Err(ContractError::AdminRequired);
            }

            Ok(())
        }

        pub fn check_valid_member(
            &self,
            sender: &AccountId,
            recipient: &AccountId,
        ) -> Result<(), ContractError> {
            if !self.is_member(sender) {
                return Err(ContractError::SenderIsNotMember);
            }

            if !self.is_member(recipient) {
                return Err(ContractError::RecipientIsNotMember);
            }

            Ok(())
        }

        pub fn is_member(&self, account: &AccountId) -> bool {
            self.members.contains(account)
        }
    }
}
