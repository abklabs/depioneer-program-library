//! This code was AUTOGENERATED using the kinobi library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun kinobi to update it.
//!
//! <https://github.com/kinobi-so/kinobi>
//!

use borsh::BorshDeserialize;
use borsh::BorshSerialize;

/// Accounts.
pub struct Increment {
    /// The program derived address of the counter account to increment (seeds: ['counter', authority])
    pub counter: solana_program::pubkey::Pubkey,
    /// The authority of the counter
    pub authority: solana_program::pubkey::Pubkey,
}

impl Increment {
    pub fn instruction(
        &self,
        args: IncrementInstructionArgs,
    ) -> solana_program::instruction::Instruction {
        self.instruction_with_remaining_accounts(args, &[])
    }
    #[allow(clippy::vec_init_then_push)]
    pub fn instruction_with_remaining_accounts(
        &self,
        args: IncrementInstructionArgs,
        remaining_accounts: &[solana_program::instruction::AccountMeta],
    ) -> solana_program::instruction::Instruction {
        let mut accounts = Vec::with_capacity(2 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.counter,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.authority,
            true,
        ));
        accounts.extend_from_slice(remaining_accounts);
        let mut data = IncrementInstructionData::new().try_to_vec().unwrap();
        let mut args = args.try_to_vec().unwrap();
        data.append(&mut args);

        solana_program::instruction::Instruction {
            program_id: crate::SHARE_POOL_ID,
            accounts,
            data,
        }
    }
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct IncrementInstructionData {
    discriminator: u8,
}

impl IncrementInstructionData {
    pub fn new() -> Self {
        Self { discriminator: 1 }
    }
}

impl Default for IncrementInstructionData {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct IncrementInstructionArgs {
    pub amount: Option<u32>,
}

/// Instruction builder for `Increment`.
///
/// ### Accounts:
///
///   0. `[writable]` counter
///   1. `[signer]` authority
#[derive(Clone, Debug, Default)]
pub struct IncrementBuilder {
    counter: Option<solana_program::pubkey::Pubkey>,
    authority: Option<solana_program::pubkey::Pubkey>,
    amount: Option<u32>,
    __remaining_accounts: Vec<solana_program::instruction::AccountMeta>,
}

impl IncrementBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    /// The program derived address of the counter account to increment (seeds: ['counter', authority])
    #[inline(always)]
    pub fn counter(&mut self, counter: solana_program::pubkey::Pubkey) -> &mut Self {
        self.counter = Some(counter);
        self
    }
    /// The authority of the counter
    #[inline(always)]
    pub fn authority(&mut self, authority: solana_program::pubkey::Pubkey) -> &mut Self {
        self.authority = Some(authority);
        self
    }
    /// `[optional argument]`
    #[inline(always)]
    pub fn amount(&mut self, amount: u32) -> &mut Self {
        self.amount = Some(amount);
        self
    }
    /// Add an aditional account to the instruction.
    #[inline(always)]
    pub fn add_remaining_account(
        &mut self,
        account: solana_program::instruction::AccountMeta,
    ) -> &mut Self {
        self.__remaining_accounts.push(account);
        self
    }
    /// Add additional accounts to the instruction.
    #[inline(always)]
    pub fn add_remaining_accounts(
        &mut self,
        accounts: &[solana_program::instruction::AccountMeta],
    ) -> &mut Self {
        self.__remaining_accounts.extend_from_slice(accounts);
        self
    }
    #[allow(clippy::clone_on_copy)]
    pub fn instruction(&self) -> solana_program::instruction::Instruction {
        let accounts = Increment {
            counter: self.counter.expect("counter is not set"),
            authority: self.authority.expect("authority is not set"),
        };
        let args = IncrementInstructionArgs {
            amount: self.amount.clone(),
        };

        accounts.instruction_with_remaining_accounts(args, &self.__remaining_accounts)
    }
}

/// `increment` CPI accounts.
pub struct IncrementCpiAccounts<'a, 'b> {
    /// The program derived address of the counter account to increment (seeds: ['counter', authority])
    pub counter: &'b solana_program::account_info::AccountInfo<'a>,
    /// The authority of the counter
    pub authority: &'b solana_program::account_info::AccountInfo<'a>,
}

/// `increment` CPI instruction.
pub struct IncrementCpi<'a, 'b> {
    /// The program to invoke.
    pub __program: &'b solana_program::account_info::AccountInfo<'a>,
    /// The program derived address of the counter account to increment (seeds: ['counter', authority])
    pub counter: &'b solana_program::account_info::AccountInfo<'a>,
    /// The authority of the counter
    pub authority: &'b solana_program::account_info::AccountInfo<'a>,
    /// The arguments for the instruction.
    pub __args: IncrementInstructionArgs,
}

impl<'a, 'b> IncrementCpi<'a, 'b> {
    pub fn new(
        program: &'b solana_program::account_info::AccountInfo<'a>,
        accounts: IncrementCpiAccounts<'a, 'b>,
        args: IncrementInstructionArgs,
    ) -> Self {
        Self {
            __program: program,
            counter: accounts.counter,
            authority: accounts.authority,
            __args: args,
        }
    }
    #[inline(always)]
    pub fn invoke(&self) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed_with_remaining_accounts(&[], &[])
    }
    #[inline(always)]
    pub fn invoke_with_remaining_accounts(
        &self,
        remaining_accounts: &[(
            &'b solana_program::account_info::AccountInfo<'a>,
            bool,
            bool,
        )],
    ) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed_with_remaining_accounts(&[], remaining_accounts)
    }
    #[inline(always)]
    pub fn invoke_signed(
        &self,
        signers_seeds: &[&[&[u8]]],
    ) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed_with_remaining_accounts(signers_seeds, &[])
    }
    #[allow(clippy::clone_on_copy)]
    #[allow(clippy::vec_init_then_push)]
    pub fn invoke_signed_with_remaining_accounts(
        &self,
        signers_seeds: &[&[&[u8]]],
        remaining_accounts: &[(
            &'b solana_program::account_info::AccountInfo<'a>,
            bool,
            bool,
        )],
    ) -> solana_program::entrypoint::ProgramResult {
        let mut accounts = Vec::with_capacity(2 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.counter.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.authority.key,
            true,
        ));
        remaining_accounts.iter().for_each(|remaining_account| {
            accounts.push(solana_program::instruction::AccountMeta {
                pubkey: *remaining_account.0.key,
                is_signer: remaining_account.1,
                is_writable: remaining_account.2,
            })
        });
        let mut data = IncrementInstructionData::new().try_to_vec().unwrap();
        let mut args = self.__args.try_to_vec().unwrap();
        data.append(&mut args);

        let instruction = solana_program::instruction::Instruction {
            program_id: crate::SHARE_POOL_ID,
            accounts,
            data,
        };
        let mut account_infos = Vec::with_capacity(2 + 1 + remaining_accounts.len());
        account_infos.push(self.__program.clone());
        account_infos.push(self.counter.clone());
        account_infos.push(self.authority.clone());
        remaining_accounts
            .iter()
            .for_each(|remaining_account| account_infos.push(remaining_account.0.clone()));

        if signers_seeds.is_empty() {
            solana_program::program::invoke(&instruction, &account_infos)
        } else {
            solana_program::program::invoke_signed(&instruction, &account_infos, signers_seeds)
        }
    }
}

/// Instruction builder for `Increment` via CPI.
///
/// ### Accounts:
///
///   0. `[writable]` counter
///   1. `[signer]` authority
#[derive(Clone, Debug)]
pub struct IncrementCpiBuilder<'a, 'b> {
    instruction: Box<IncrementCpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> IncrementCpiBuilder<'a, 'b> {
    pub fn new(program: &'b solana_program::account_info::AccountInfo<'a>) -> Self {
        let instruction = Box::new(IncrementCpiBuilderInstruction {
            __program: program,
            counter: None,
            authority: None,
            amount: None,
            __remaining_accounts: Vec::new(),
        });
        Self { instruction }
    }
    /// The program derived address of the counter account to increment (seeds: ['counter', authority])
    #[inline(always)]
    pub fn counter(
        &mut self,
        counter: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.counter = Some(counter);
        self
    }
    /// The authority of the counter
    #[inline(always)]
    pub fn authority(
        &mut self,
        authority: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.authority = Some(authority);
        self
    }
    /// `[optional argument]`
    #[inline(always)]
    pub fn amount(&mut self, amount: u32) -> &mut Self {
        self.instruction.amount = Some(amount);
        self
    }
    /// Add an additional account to the instruction.
    #[inline(always)]
    pub fn add_remaining_account(
        &mut self,
        account: &'b solana_program::account_info::AccountInfo<'a>,
        is_writable: bool,
        is_signer: bool,
    ) -> &mut Self {
        self.instruction
            .__remaining_accounts
            .push((account, is_writable, is_signer));
        self
    }
    /// Add additional accounts to the instruction.
    ///
    /// Each account is represented by a tuple of the `AccountInfo`, a `bool` indicating whether the account is writable or not,
    /// and a `bool` indicating whether the account is a signer or not.
    #[inline(always)]
    pub fn add_remaining_accounts(
        &mut self,
        accounts: &[(
            &'b solana_program::account_info::AccountInfo<'a>,
            bool,
            bool,
        )],
    ) -> &mut Self {
        self.instruction
            .__remaining_accounts
            .extend_from_slice(accounts);
        self
    }
    #[inline(always)]
    pub fn invoke(&self) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed(&[])
    }
    #[allow(clippy::clone_on_copy)]
    #[allow(clippy::vec_init_then_push)]
    pub fn invoke_signed(
        &self,
        signers_seeds: &[&[&[u8]]],
    ) -> solana_program::entrypoint::ProgramResult {
        let args = IncrementInstructionArgs {
            amount: self.instruction.amount.clone(),
        };
        let instruction = IncrementCpi {
            __program: self.instruction.__program,

            counter: self.instruction.counter.expect("counter is not set"),

            authority: self.instruction.authority.expect("authority is not set"),
            __args: args,
        };
        instruction.invoke_signed_with_remaining_accounts(
            signers_seeds,
            &self.instruction.__remaining_accounts,
        )
    }
}

#[derive(Clone, Debug)]
struct IncrementCpiBuilderInstruction<'a, 'b> {
    __program: &'b solana_program::account_info::AccountInfo<'a>,
    counter: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    authority: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    amount: Option<u32>,
    /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
    __remaining_accounts: Vec<(
        &'b solana_program::account_info::AccountInfo<'a>,
        bool,
        bool,
    )>,
}
