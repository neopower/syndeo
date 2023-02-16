#[derive(PartialEq, Debug, Eq, Clone, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum ContractError {
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
}
