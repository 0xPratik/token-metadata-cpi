use anchor_lang::prelude::*;
use anchor_lang::{solana_program, Result};
use mpl_token_metadata::instruction;
use mpl_token_metadata::state::Collection;
use mpl_token_metadata::state::Creator;
use mpl_token_metadata::state::Uses;
pub use mpl_token_metadata::ID;
// use std::ops::Deref;
pub fn create_metadata_accounts_v2<'a, 'b, 'c, 'info>(
    ctx: CpiContext<'a, 'b, 'c, 'info, CreateMetadataAccountV2<'info>>,
    name: String,
    symbol: String,
    uri: String,
    seller_fee_basis_points: u16,
    update_authority_is_signer: bool,
    is_mutable: bool,
    uses: Uses,
    collection: Collection,
    creators: Vec<Creator>,
) -> Result<()> {
    let ix = instruction::create_metadata_accounts_v2(
        mpl_token_metadata::ID,
        ctx.accounts.metadata_account.key(),
        ctx.accounts.mint.key(),
        ctx.accounts.mint_authority.key(),
        ctx.accounts.payer.key(),
        ctx.accounts.update_authority.key(),
        name,
        symbol,
        uri,
        Some(creators),
        seller_fee_basis_points,
        update_authority_is_signer,
        is_mutable,
        Some(collection),
        Some(uses),
    );
    solana_program::program::invoke_signed(
        &ix,
        &[
            ctx.accounts.metadata_account,
            ctx.accounts.mint,
            ctx.accounts.mint_authority,
            ctx.accounts.payer,
            ctx.accounts.update_authority,
            ctx.accounts.system_program.to_account_info(),
            ctx.accounts.rent.to_account_info(),
        ],
        ctx.signer_seeds,
    )
    .map_err(Into::into)
}

#[derive(Accounts)]
pub struct CreateMetadataAccountV2<'info> {
    pub metadata_account: AccountInfo<'info>,
    pub mint: AccountInfo<'info>,
    pub mint_authority: AccountInfo<'info>,
    pub payer: AccountInfo<'info>,
    pub update_authority: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}
