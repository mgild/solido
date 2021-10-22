// SPDX-FileCopyrightText: 2021 Chorus One AG
// SPDX-License-Identifier: GPL-3.0

use solana_program::pubkey::Pubkey;

mod logic;

#[cfg(not(feature = "no-entrypoint"))]
pub mod entrypoint;

pub mod error;
pub mod instruction;
pub mod processor;
pub mod state;
pub mod token;

/// Mint authority, mints bSOL.
pub const ANKER_MINT_AUTHORITY: &[u8] = b"mint_authority";

/// Anker's authority that will control the reserve account.
pub const ANKER_RESERVE_AUTHORITY: &[u8] = b"reserve_authority";

/// Anker's reserve account. Holds StSOL.
pub const ANKER_RESERVE_ACCOUNT: &[u8] = b"reserve_authority";

/// Return the address at which the Anker instance should live that belongs to
/// the given Solido instance.
pub fn find_instance_address(anker_program_id: &Pubkey, solido_instance: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[solido_instance.as_ref()], anker_program_id)
}

/// Return the owner of the stSOL reserve account, and bump seed.
pub fn find_reserve_authority(anker_program_id: &Pubkey, anker_instance: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[anker_instance.as_ref(), ANKER_RESERVE_AUTHORITY],
        anker_program_id,
    )
}

/// Return the address of the stSOL reserve account, and bump seed.
pub fn find_reserve_account(anker_program_id: &Pubkey, anker_instance: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[anker_instance.as_ref(), ANKER_RESERVE_ACCOUNT],
        anker_program_id,
    )
}

/// Return the mint authority for bSOL, and bump seed.
pub fn find_mint_authority(anker_program_id: &Pubkey, anker_instance: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[anker_instance.as_ref(), ANKER_MINT_AUTHORITY],
        anker_program_id,
    )
}
