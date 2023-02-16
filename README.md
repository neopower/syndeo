<div align="center">
<h1 align="center">Syndeo - Smart Contract</h1>

[![Built with ink!](https://raw.githubusercontent.com/paritytech/ink/master/.images/badge_flat.svg)](https://github.com/paritytech/ink)
![Twitter Follow](https://img.shields.io/twitter/follow/NeoPowerDigital?style=social)

</div>

> **Syndeo** is a decentralized platform that **motivates people to contribute to organizations by allowing members to award each other points**. The organization admins can then distribute treasury funds to members based on their points. This creates a **fair and transparent way to recognize people's contributions in organizations**, making the process **democratic and decentralized**.

## Environment setup

To compile the smart contract, Rust and Cargo are required. Here is an [installation guide](https://doc.rust-lang.org/cargo/getting-started/installation.html).

[cargo-contract](https://github.com/paritytech/cargo-contract) is required too. Install it using this command:

```shell
cargo install cargo-contract --force --locked
```

## Build smart contract

To build the smart contract and generates the optimized WebAssembly bytecode, the metadata and bundles, execute this command:

```shell
cargo contract build --release
```

## Run tests off-chain

To run the tests off-chain, execute this command:

```shell
cargo test
```

## Upload & instantiate

Open the [Substrate Contracts-UI](https://contracts-ui.substrate.io).

Choose a chain (E.g. `Contracts (Rococo)` or `Shibuya`) in the dropdown placed in the top section of the left menu.

Follow the [official ink! guide](https://use.ink/getting-started/deploy-your-contract/#using-the-contracts-ui) to upload and instantiate the smart contract.

## ink! version

`ink`: 4.0.0

https://github.com/paritytech/ink/tree/v4.0.0

## Data Model

### Storage

```rust
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
```

### Events
```rust
struct AdminChanged {
    admin: AccountId,
}

struct NewMember {
    member: AccountId,
}

struct MemberDeleted {
    member: AccountId,
}

struct Award {
    sender: AccountId,
    recipient: AccountId,
    points: u64,
}

struct RewardGranted {
    recipient: AccountId,
    reward: Balance,
    points: u64,
}
```

### RewardsSummary
```rust
struct RewardsSummary {
    assigned_points: u64,
    members_awarded: u64,
    funds: Balance,
}
```

### Errors
```rust
enum ContractError {
    MemberAlreadyExists,
    MemberDoesNotExist,
    AdminRequired,
    MaxPointsPerSenderCannotBeZero,
    MaxPointsPerSenderExceeded,
    AwardPointsMustBeGreaterThanZero,
    CannotAwardYourself,
    SenderIsNotMember,
    RecipientIsNotMember,
    NullFunds,
    RewardExceedsContractBalance,
    NoRecipients,
}
```

## Messages - Transactions

### set_admin
> Set a new admin. 
> Only the current admin can execute this message.

```rust
fn set_admin(&mut self, new_admin: AccountId) -> Result<(), ContractError>;
```

- Related errors: `AdminRequired` 

- Related events: `AdminChanged`

### add_member
> Add a new member. 
> Only the current admin can execute this message.

```rust
fn add_member(&mut self, new_member: AccountId) -> Result<(), ContractError>;
```

- Related errors: `MemberAlreadyExists` 

- Related events: `NewMember`

### remove_member
> Remove a member. 
> Only the current admin can execute this message.

```rust
fn remove_member(&mut self, member_to_remove: AccountId) -> Result<(), ContractError>;
```

- Related errors: `MemberDoesNotExist` 

- Related events: `MemberDeleted`

### set_max_points_per_sender
> Set the maximum number of points a member can totally use to award other members. 
> Only the current admin can execute this message.

```rust
fn set_max_points_per_sender(&mut self, max_points_per_sender: u64) -> Result<(), ContractError>;
```

- Related errors: `MaxPointsPerSenderCannotBeZero`

### award
> Award another member with a number of points. 
> A member can not award himself.
> The sender and the recipient must be members. 
> The maximum number of points used by a member must not exceed the value defined in `max_points_per_sender`.

```rust
fn award(&mut self, recipient: AccountId, points: u64) -> Result<(), ContractError>;
```

- Related errors: `AwardPointsMustBeGreaterThanZero`, `CannotAwardYourself`, `SenderIsNotMember`, `RecipientIsNotMember`, `MaxPointsPerSenderExceeded`
- Related events: `Award`

### distribute_rewards
> Distribute proportionally the rewards to the award recipients according to the received number of points.  
> The rewards to distribute can be the entire balance deposited in the contract or a specific quantity sent as a parameter (`amount_to_distribute`). This parameter value must not exceed the contract balance.  
> Only the current admin can execute this message.

```rust
fn distribute_rewards(&mut self, amount_to_distribute: Option<Balance>) -> Result<(), ContractError>;
```

- Related errors: `NoRecipients`, `NullFunds`, `RewardExceedsContractBalance` 
- Related events: `RewardGranted`

## Messages - Queries

### get_rewards_summary
> Returns the total assigned points, the number of members to be rewarded, and the funds deposited in the contract.   

```rust
fn get_rewards_summary(&self) -> RewardsSummary;
```

### get_sender_available_points
> Returns the sender's available points to award another member. 

```rust
fn get_sender_available_points(&self) -> u64;
```

### get_max_points_per_sender
> Returns the maximum number of points a member can totally use to award other members.

```rust
fn get_max_points_per_sender(&self) -> u64;
```