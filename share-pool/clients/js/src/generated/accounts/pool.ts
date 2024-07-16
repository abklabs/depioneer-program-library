/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/kinobi-so/kinobi
 */

import {
  assertAccountExists,
  assertAccountsExist,
  combineCodec,
  decodeAccount,
  fetchEncodedAccount,
  fetchEncodedAccounts,
  getAddressDecoder,
  getAddressEncoder,
  getArrayDecoder,
  getArrayEncoder,
  getStructDecoder,
  getStructEncoder,
  getTupleDecoder,
  getTupleEncoder,
  getU64Decoder,
  getU64Encoder,
  type Account,
  type Address,
  type Codec,
  type Decoder,
  type EncodedAccount,
  type Encoder,
  type FetchAccountConfig,
  type FetchAccountsConfig,
  type MaybeAccount,
  type MaybeEncodedAccount,
} from '@solana/web3.js';
import { getKeyDecoder, getKeyEncoder, type Key, type KeyArgs } from '../types';

export type Pool = {
  key: Key;
  collection: Address;
  authority: Address;
  sharesPerToken: bigint;
  poolNfts: Array<readonly [Address, Address]>;
};

export type PoolArgs = {
  key: KeyArgs;
  collection: Address;
  authority: Address;
  sharesPerToken: number | bigint;
  poolNfts: Array<readonly [Address, Address]>;
};

export function getPoolEncoder(): Encoder<PoolArgs> {
  return getStructEncoder([
    ['key', getKeyEncoder()],
    ['collection', getAddressEncoder()],
    ['authority', getAddressEncoder()],
    ['sharesPerToken', getU64Encoder()],
    [
      'poolNfts',
      getArrayEncoder(
        getTupleEncoder([getAddressEncoder(), getAddressEncoder()])
      ),
    ],
  ]);
}

export function getPoolDecoder(): Decoder<Pool> {
  return getStructDecoder([
    ['key', getKeyDecoder()],
    ['collection', getAddressDecoder()],
    ['authority', getAddressDecoder()],
    ['sharesPerToken', getU64Decoder()],
    [
      'poolNfts',
      getArrayDecoder(
        getTupleDecoder([getAddressDecoder(), getAddressDecoder()])
      ),
    ],
  ]);
}

export function getPoolCodec(): Codec<PoolArgs, Pool> {
  return combineCodec(getPoolEncoder(), getPoolDecoder());
}

export function decodePool<TAddress extends string = string>(
  encodedAccount: EncodedAccount<TAddress>
): Account<Pool, TAddress>;
export function decodePool<TAddress extends string = string>(
  encodedAccount: MaybeEncodedAccount<TAddress>
): MaybeAccount<Pool, TAddress>;
export function decodePool<TAddress extends string = string>(
  encodedAccount: EncodedAccount<TAddress> | MaybeEncodedAccount<TAddress>
): Account<Pool, TAddress> | MaybeAccount<Pool, TAddress> {
  return decodeAccount(
    encodedAccount as MaybeEncodedAccount<TAddress>,
    getPoolDecoder()
  );
}

export async function fetchPool<TAddress extends string = string>(
  rpc: Parameters<typeof fetchEncodedAccount>[0],
  address: Address<TAddress>,
  config?: FetchAccountConfig
): Promise<Account<Pool, TAddress>> {
  const maybeAccount = await fetchMaybePool(rpc, address, config);
  assertAccountExists(maybeAccount);
  return maybeAccount;
}

export async function fetchMaybePool<TAddress extends string = string>(
  rpc: Parameters<typeof fetchEncodedAccount>[0],
  address: Address<TAddress>,
  config?: FetchAccountConfig
): Promise<MaybeAccount<Pool, TAddress>> {
  const maybeAccount = await fetchEncodedAccount(rpc, address, config);
  return decodePool(maybeAccount);
}

export async function fetchAllPool(
  rpc: Parameters<typeof fetchEncodedAccounts>[0],
  addresses: Array<Address>,
  config?: FetchAccountsConfig
): Promise<Account<Pool>[]> {
  const maybeAccounts = await fetchAllMaybePool(rpc, addresses, config);
  assertAccountsExist(maybeAccounts);
  return maybeAccounts;
}

export async function fetchAllMaybePool(
  rpc: Parameters<typeof fetchEncodedAccounts>[0],
  addresses: Array<Address>,
  config?: FetchAccountsConfig
): Promise<MaybeAccount<Pool>[]> {
  const maybeAccounts = await fetchEncodedAccounts(rpc, addresses, config);
  return maybeAccounts.map((maybeAccount) => decodePool(maybeAccount));
}
