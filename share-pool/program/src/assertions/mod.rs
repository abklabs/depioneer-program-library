pub mod spl;

pub use spl::*;

use crate::{error::SharePoolError, state::Key};
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, msg, program_error::ProgramError,
    pubkey::Pubkey,
};

/// Assert that the given account is owned by the given program.
pub fn assert_program_owner(
    account_name: &str,
    account: &AccountInfo,
    owner: &Pubkey,
) -> ProgramResult {
    if account.owner != owner {
        msg!(
            "Account \"{}\" [{}] expected program owner [{}], got [{}]",
            account_name,
            account.key,
            owner,
            account.owner
        );
        Err(SharePoolError::InvalidProgramOwner.into())
    } else {
        Ok(())
    }
}

/// Assert that the given account is owned by one of the given programs.
pub fn assert_program_owners(
    account_name: &str,
    account: &AccountInfo,
    owners: &[Pubkey],
) -> ProgramResult {
    if !owners.contains(account.owner) {
        msg!(
            "Account \"{}\" [{}] expected program owner in {:?}, got [{}]",
            account_name,
            account.key,
            owners,
            account.owner
        );
        Err(SharePoolError::InvalidProgramOwner.into())
    } else {
        Ok(())
    }
}

/// Assert the derivation of the seeds against the given account and return the bump seed.
pub fn assert_pda(
    account_name: &str,
    account: &AccountInfo,
    program_id: &Pubkey,
    seeds: &[&[u8]],
) -> Result<u8, ProgramError> {
    let (key, bump) = Pubkey::find_program_address(seeds, program_id);
    if *account.key != key {
        msg!(
            "Account \"{}\" [{}] is an invalid PDA. Expected the following valid PDA [{}]",
            account_name,
            account.key,
            key,
        );
        return Err(SharePoolError::InvalidPda.into());
    }
    Ok(bump)
}

/// Assert the derivation of the seeds and bump against the given account.
pub fn assert_pda_with_bump(
    account_name: &str,
    account: &AccountInfo,
    program_id: &Pubkey,
    seeds_with_bump: &[&[u8]],
) -> Result<(), ProgramError> {
    let key = Pubkey::create_program_address(seeds_with_bump, program_id).map_err(|_| {
        msg!(
            "The seeds and bump passed for \"{}\" [{}] don't yield a valid PDA",
            account_name,
            account.key,
        );
        SharePoolError::InvalidSeeds
    })?;
    if *account.key != key {
        msg!(
            "Account \"{}\" [{}] is an invalid PDA. Expected the following valid PDA [{}]",
            account_name,
            account.key,
            key,
        );
        return Err(SharePoolError::InvalidPda.into());
    }
    Ok(())
}

/// Assert that the given account is empty.
pub fn assert_empty(account_name: &str, account: &AccountInfo) -> ProgramResult {
    if !account.data_is_empty() {
        msg!(
            "Account \"{}\" [{}] must be empty",
            account_name,
            account.key,
        );
        Err(SharePoolError::ExpectedEmptyAccount.into())
    } else {
        Ok(())
    }
}

/// Assert that the given account is non empty.
pub fn assert_non_empty(account_name: &str, account: &AccountInfo) -> ProgramResult {
    if account.data_is_empty() {
        msg!(
            "Account \"{}\" [{}] must not be empty",
            account_name,
            account.key,
        );
        Err(SharePoolError::ExpectedNonEmptyAccount.into())
    } else {
        Ok(())
    }
}

/// Assert that the given account is a signer.
pub fn assert_signer(account_name: &str, account: &AccountInfo) -> ProgramResult {
    if !account.is_signer {
        msg!(
            "Account \"{}\" [{}] must be a signer",
            account_name,
            account.key,
        );
        Err(SharePoolError::ExpectedSignerAccount.into())
    } else {
        Ok(())
    }
}

/// Assert that the given account is writable.
pub fn assert_writable(account_name: &str, account: &AccountInfo) -> ProgramResult {
    if !account.is_writable {
        msg!(
            "Account \"{}\" [{}] must be writable",
            account_name,
            account.key,
        );
        Err(SharePoolError::ExpectedWritableAccount.into())
    } else {
        Ok(())
    }
}

/// Assert that the given account matches the given public key.
pub fn assert_same_pubkeys(
    account_name: &str,
    account: &AccountInfo,
    expected: &Pubkey,
) -> ProgramResult {
    if account.key != expected {
        msg!(
            "Account \"{}\" [{}] must match the following public key [{}]",
            account_name,
            account.key,
            expected
        );
        Err(SharePoolError::AccountMismatch.into())
    } else {
        Ok(())
    }
}

/// Assert that the given account has the expected account key.
pub fn assert_account_key(account_name: &str, account: &AccountInfo, key: Key) -> ProgramResult {
    let key_number = key as u8;
    if account.data_len() <= 1 || account.try_borrow_data()?[0] != key_number {
        msg!(
            "Account \"{}\" [{}] expected account key [{}], got [{}]",
            account_name,
            account.key,
            key_number,
            account.try_borrow_data()?[0]
        );
        Err(SharePoolError::InvalidAccountKey.into())
    } else {
        Ok(())
    }
}
