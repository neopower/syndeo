use crate::errors::ContractError;
use crate::syndeo::Syndeo;

use ink::env::test::{callee, default_accounts, set_account_balance, set_caller, DefaultAccounts};
use ink::env::DefaultEnvironment;
use ink::primitives::AccountId;

fn get_contract_account_id() -> AccountId {
    callee::<DefaultEnvironment>()
}

fn get_default_accounts() -> DefaultAccounts<DefaultEnvironment> {
    default_accounts::<DefaultEnvironment>()
}

fn set_sender(sender: AccountId) {
    set_caller::<DefaultEnvironment>(sender);
}

fn set_balance(balance: u128) {
    set_account_balance::<DefaultEnvironment>(get_contract_account_id(), balance)
}

fn init(max_points_per_sender: Option<u64>) -> (Syndeo, DefaultAccounts<DefaultEnvironment>) {
    (Syndeo::new(max_points_per_sender), get_default_accounts())
}

#[ink::test]
fn init_works() {
    // Arrange
    let max_points_per_sender = 5;

    // Act
    let (contract, _) = init(Some(max_points_per_sender));

    // Assert
    assert_eq!(contract.max_points_per_sender, max_points_per_sender);
}

#[ink::test]
fn is_member_works() {
    // Arrange
    let (mut contract, accounts) = init(None);
    contract.members.push(accounts.alice);

    // Act
    let result_true = contract.is_member(&accounts.alice);
    let result_false = contract.is_member(&accounts.bob);

    // Assert
    assert_eq!(result_true, true);
    assert_eq!(result_false, false);
}

#[ink::test]
fn check_admin() {
    // Arrange
    let (contract, accounts) = init(None);

    // Act
    set_sender(accounts.alice);
    let result_ok = contract.check_admin();

    set_sender(accounts.bob);
    let result_fails = contract.check_admin();

    // Assert
    assert_eq!(result_ok, Ok(()));
    assert_eq!(result_fails, Err(ContractError::AdminRequired))
}

#[ink::test]
fn add_member() {
    // Arrange
    let (mut contract, accounts) = init(None);
    set_sender(accounts.alice);

    // Act
    let result_ok = contract.add_member(accounts.bob);
    let result_fails = contract.add_member(accounts.bob);

    // Assert
    assert_eq!(result_ok, Ok(()));
    assert_eq!(result_fails, Err(ContractError::MemberAlreadyExists));
}

#[ink::test]
fn award_works() {
    // Arrange
    let (mut contract, accounts) = init(None);
    set_sender(accounts.alice);
    contract.members.push(accounts.bob);
    contract.members.push(accounts.frank);

    // Act
    let result_award_bob_ok = contract.award(accounts.bob, 2);
    let result_award_frank_ok = contract.award(accounts.frank, 3);

    // Assert
    assert_eq!(result_award_bob_ok, Ok(()));
    assert_eq!(result_award_frank_ok, Ok(()));
    assert_eq!(contract.total_points, 5);
    assert_eq!(contract.recipients.len(), 2);
    assert_eq!(contract.senders.len(), 1);
}

#[ink::test]
fn award_with_null_points_amount_fails() {
    // Arrange
    let (mut contract, accounts) = init(None);
    set_sender(accounts.alice);

    // Act
    let result_fails = contract.award(accounts.bob, 0);

    // Assert
    assert_eq!(
        result_fails,
        Err(ContractError::AwardPointsMustBeGreaterThanZero)
    );
}

#[ink::test]
fn award_with_same_sender_and_recipient_fails() {
    // Arrange
    let (mut contract, accounts) = init(None);
    set_sender(accounts.alice);

    // Act
    let result_fails = contract.award(accounts.alice, 1);

    // Assert
    assert_eq!(result_fails, Err(ContractError::CannotAwardYourself));
}

#[ink::test]
fn award_with_points_amount_exceeding_the_maximum_fails() {
    // Arrange
    let (mut contract, accounts) = init(Some(5));
    set_sender(accounts.alice);
    contract.members.push(accounts.bob);

    // Act
    let result_fails = contract.award(accounts.bob, 10);

    // Assert
    assert_eq!(result_fails, Err(ContractError::MaxPointsPerSenderExceeded));
}

#[ink::test]
fn distribute_rewards_works() {
    // Arrange
    let (mut contract, accounts) = init(None);
    let total_reward = 30_000_000_000_000 as u128;
    let bob_points = 5 as u64;
    let frank_points = 10 as u64;

    contract
        .points_by_recipient
        .insert(accounts.bob, &bob_points);
    contract
        .points_by_recipient
        .insert(accounts.frank, &frank_points);
    contract.recipients.push(accounts.bob);
    contract.recipients.push(accounts.frank);
    contract.total_points = 15;

    set_balance(total_reward);
    set_sender(accounts.alice);

    // Act
    let result_ok = contract.distribute_rewards(None);

    // Assert
    assert_eq!(result_ok, Ok(()));
}

#[ink::test]
fn distribute_rewards_with_null_balance_fails() {
    // Arrange
    let (mut contract, accounts) = init(None);
    set_sender(accounts.alice);
    set_balance(0);
    contract.recipients.push(accounts.bob);

    // Act
    let result_fails = contract.distribute_rewards(None);

    // Assert
    assert_eq!(result_fails, Err(ContractError::NullFunds));
}

#[ink::test]
fn distribute_rewards_with_null_amount_input_fails() {
    // Arrange
    let (mut contract, accounts) = init(None);
    set_sender(accounts.alice);
    contract.recipients.push(accounts.bob);

    // Act
    let result_fails = contract.distribute_rewards(Some(0));

    // Assert
    assert_eq!(result_fails, Err(ContractError::NullFunds));
}

#[ink::test]
fn distribute_rewards_with_no_recipients_loaded() {
    // Arrange
    let (mut contract, accounts) = init(None);
    set_sender(accounts.alice);

    // Act
    let result_fails = contract.distribute_rewards(Some(1));

    // Assert
    assert_eq!(result_fails, Err(ContractError::NoRecipients));
}
