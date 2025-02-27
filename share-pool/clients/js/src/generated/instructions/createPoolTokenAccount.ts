/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/kinobi-so/kinobi
 */

import {
  combineCodec,
  getStructDecoder,
  getStructEncoder,
  getU8Decoder,
  getU8Encoder,
  transformEncoder,
  type Address,
  type Codec,
  type Decoder,
  type Encoder,
  type IAccountMeta,
  type IAccountSignerMeta,
  type IInstruction,
  type IInstructionWithAccounts,
  type IInstructionWithData,
  type ReadonlyAccount,
  type ReadonlySignerAccount,
  type TransactionSigner,
  type WritableAccount,
  type WritableSignerAccount,
} from '@solana/web3.js';
import { SHARE_POOL_PROGRAM_ADDRESS } from '../programs';
import { getAccountMetaFactory, type ResolvedAccount } from '../shared';

export type CreatePoolTokenAccountInstruction<
  TProgram extends string = typeof SHARE_POOL_PROGRAM_ADDRESS,
  TAccountPool extends string | IAccountMeta<string> = string,
  TAccountMint extends string | IAccountMeta<string> = string,
  TAccountPoolTokenAccount extends string | IAccountMeta<string> = string,
  TAccountAuthority extends string | IAccountMeta<string> = string,
  TAccountPayer extends string | IAccountMeta<string> = string,
  TAccountTokenProgram extends
    | string
    | IAccountMeta<string> = 'TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA',
  TAccountSystemProgram extends
    | string
    | IAccountMeta<string> = '11111111111111111111111111111111',
  TAccountRent extends
    | string
    | IAccountMeta<string> = 'SysvarRent111111111111111111111111111111111',
  TRemainingAccounts extends readonly IAccountMeta<string>[] = [],
> = IInstruction<TProgram> &
  IInstructionWithData<Uint8Array> &
  IInstructionWithAccounts<
    [
      TAccountPool extends string
        ? WritableAccount<TAccountPool>
        : TAccountPool,
      TAccountMint extends string
        ? ReadonlyAccount<TAccountMint>
        : TAccountMint,
      TAccountPoolTokenAccount extends string
        ? WritableAccount<TAccountPoolTokenAccount>
        : TAccountPoolTokenAccount,
      TAccountAuthority extends string
        ? ReadonlySignerAccount<TAccountAuthority> &
            IAccountSignerMeta<TAccountAuthority>
        : TAccountAuthority,
      TAccountPayer extends string
        ? WritableSignerAccount<TAccountPayer> &
            IAccountSignerMeta<TAccountPayer>
        : TAccountPayer,
      TAccountTokenProgram extends string
        ? ReadonlyAccount<TAccountTokenProgram>
        : TAccountTokenProgram,
      TAccountSystemProgram extends string
        ? ReadonlyAccount<TAccountSystemProgram>
        : TAccountSystemProgram,
      TAccountRent extends string
        ? ReadonlyAccount<TAccountRent>
        : TAccountRent,
      ...TRemainingAccounts,
    ]
  >;

export type CreatePoolTokenAccountInstructionData = { discriminator: number };

export type CreatePoolTokenAccountInstructionDataArgs = {};

export function getCreatePoolTokenAccountInstructionDataEncoder(): Encoder<CreatePoolTokenAccountInstructionDataArgs> {
  return transformEncoder(
    getStructEncoder([['discriminator', getU8Encoder()]]),
    (value) => ({ ...value, discriminator: 1 })
  );
}

export function getCreatePoolTokenAccountInstructionDataDecoder(): Decoder<CreatePoolTokenAccountInstructionData> {
  return getStructDecoder([['discriminator', getU8Decoder()]]);
}

export function getCreatePoolTokenAccountInstructionDataCodec(): Codec<
  CreatePoolTokenAccountInstructionDataArgs,
  CreatePoolTokenAccountInstructionData
> {
  return combineCodec(
    getCreatePoolTokenAccountInstructionDataEncoder(),
    getCreatePoolTokenAccountInstructionDataDecoder()
  );
}

export type CreatePoolTokenAccountInput<
  TAccountPool extends string = string,
  TAccountMint extends string = string,
  TAccountPoolTokenAccount extends string = string,
  TAccountAuthority extends string = string,
  TAccountPayer extends string = string,
  TAccountTokenProgram extends string = string,
  TAccountSystemProgram extends string = string,
  TAccountRent extends string = string,
> = {
  /** Pool account to add mint (seeds: ['pool', mint, pool_token_account, authority]) */
  pool: Address<TAccountPool>;
  /** Mint account to add */
  mint: Address<TAccountMint>;
  /** Pool token account to save (seeds: ['pool_token_account', pool, mint]) */
  poolTokenAccount: Address<TAccountPoolTokenAccount>;
  /** Authority account */
  authority: TransactionSigner<TAccountAuthority>;
  /** Payer account */
  payer: TransactionSigner<TAccountPayer>;
  /** Token program account */
  tokenProgram?: Address<TAccountTokenProgram>;
  /** System program account */
  systemProgram?: Address<TAccountSystemProgram>;
  /** Rent sysvar account */
  rent?: Address<TAccountRent>;
};

export function getCreatePoolTokenAccountInstruction<
  TAccountPool extends string,
  TAccountMint extends string,
  TAccountPoolTokenAccount extends string,
  TAccountAuthority extends string,
  TAccountPayer extends string,
  TAccountTokenProgram extends string,
  TAccountSystemProgram extends string,
  TAccountRent extends string,
>(
  input: CreatePoolTokenAccountInput<
    TAccountPool,
    TAccountMint,
    TAccountPoolTokenAccount,
    TAccountAuthority,
    TAccountPayer,
    TAccountTokenProgram,
    TAccountSystemProgram,
    TAccountRent
  >
): CreatePoolTokenAccountInstruction<
  typeof SHARE_POOL_PROGRAM_ADDRESS,
  TAccountPool,
  TAccountMint,
  TAccountPoolTokenAccount,
  TAccountAuthority,
  TAccountPayer,
  TAccountTokenProgram,
  TAccountSystemProgram,
  TAccountRent
> {
  // Program address.
  const programAddress = SHARE_POOL_PROGRAM_ADDRESS;

  // Original accounts.
  const originalAccounts = {
    pool: { value: input.pool ?? null, isWritable: true },
    mint: { value: input.mint ?? null, isWritable: false },
    poolTokenAccount: {
      value: input.poolTokenAccount ?? null,
      isWritable: true,
    },
    authority: { value: input.authority ?? null, isWritable: false },
    payer: { value: input.payer ?? null, isWritable: true },
    tokenProgram: { value: input.tokenProgram ?? null, isWritable: false },
    systemProgram: { value: input.systemProgram ?? null, isWritable: false },
    rent: { value: input.rent ?? null, isWritable: false },
  };
  const accounts = originalAccounts as Record<
    keyof typeof originalAccounts,
    ResolvedAccount
  >;

  // Resolve default values.
  if (!accounts.tokenProgram.value) {
    accounts.tokenProgram.value =
      'TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA' as Address<'TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA'>;
  }
  if (!accounts.systemProgram.value) {
    accounts.systemProgram.value =
      '11111111111111111111111111111111' as Address<'11111111111111111111111111111111'>;
  }
  if (!accounts.rent.value) {
    accounts.rent.value =
      'SysvarRent111111111111111111111111111111111' as Address<'SysvarRent111111111111111111111111111111111'>;
  }

  const getAccountMeta = getAccountMetaFactory(programAddress, 'programId');
  const instruction = {
    accounts: [
      getAccountMeta(accounts.pool),
      getAccountMeta(accounts.mint),
      getAccountMeta(accounts.poolTokenAccount),
      getAccountMeta(accounts.authority),
      getAccountMeta(accounts.payer),
      getAccountMeta(accounts.tokenProgram),
      getAccountMeta(accounts.systemProgram),
      getAccountMeta(accounts.rent),
    ],
    programAddress,
    data: getCreatePoolTokenAccountInstructionDataEncoder().encode({}),
  } as CreatePoolTokenAccountInstruction<
    typeof SHARE_POOL_PROGRAM_ADDRESS,
    TAccountPool,
    TAccountMint,
    TAccountPoolTokenAccount,
    TAccountAuthority,
    TAccountPayer,
    TAccountTokenProgram,
    TAccountSystemProgram,
    TAccountRent
  >;

  return instruction;
}

export type ParsedCreatePoolTokenAccountInstruction<
  TProgram extends string = typeof SHARE_POOL_PROGRAM_ADDRESS,
  TAccountMetas extends readonly IAccountMeta[] = readonly IAccountMeta[],
> = {
  programAddress: Address<TProgram>;
  accounts: {
    /** Pool account to add mint (seeds: ['pool', mint, pool_token_account, authority]) */
    pool: TAccountMetas[0];
    /** Mint account to add */
    mint: TAccountMetas[1];
    /** Pool token account to save (seeds: ['pool_token_account', pool, mint]) */
    poolTokenAccount: TAccountMetas[2];
    /** Authority account */
    authority: TAccountMetas[3];
    /** Payer account */
    payer: TAccountMetas[4];
    /** Token program account */
    tokenProgram: TAccountMetas[5];
    /** System program account */
    systemProgram: TAccountMetas[6];
    /** Rent sysvar account */
    rent: TAccountMetas[7];
  };
  data: CreatePoolTokenAccountInstructionData;
};

export function parseCreatePoolTokenAccountInstruction<
  TProgram extends string,
  TAccountMetas extends readonly IAccountMeta[],
>(
  instruction: IInstruction<TProgram> &
    IInstructionWithAccounts<TAccountMetas> &
    IInstructionWithData<Uint8Array>
): ParsedCreatePoolTokenAccountInstruction<TProgram, TAccountMetas> {
  if (instruction.accounts.length < 8) {
    // TODO: Coded error.
    throw new Error('Not enough accounts');
  }
  let accountIndex = 0;
  const getNextAccount = () => {
    const accountMeta = instruction.accounts![accountIndex]!;
    accountIndex += 1;
    return accountMeta;
  };
  return {
    programAddress: instruction.programAddress,
    accounts: {
      pool: getNextAccount(),
      mint: getNextAccount(),
      poolTokenAccount: getNextAccount(),
      authority: getNextAccount(),
      payer: getNextAccount(),
      tokenProgram: getNextAccount(),
      systemProgram: getNextAccount(),
      rent: getNextAccount(),
    },
    data: getCreatePoolTokenAccountInstructionDataDecoder().decode(
      instruction.data
    ),
  };
}
