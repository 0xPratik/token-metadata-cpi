use anchor_lang::prelude::*;
use anchor_lang::{solana_program, Result};
use anchor_spl::{
    associated_token::{self, AssociatedToken, Create},
    token::{self, Mint, MintTo, Token},
};
use mpl_token_metadata::instruction;
use mpl_token_metadata::state::Collection;
use mpl_token_metadata::state::Creator;
use mpl_token_metadata::state::Data;
use mpl_token_metadata::state::DataV2;

use mpl_token_metadata::state::Uses;
pub use mpl_token_metadata::ID;

pub const PREFIX: &str = "metadata";
pub const EDITION: &str = "edition";
pub const EDITION_MARKER_BIT_SIZE: u64 = 248;
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

pub fn create_metadata_accounts<'a, 'b, 'c, 'info>(
    ctx: CpiContext<'a, 'b, 'c, 'info, CreateMetadataAccounts<'info>>,
    name: String,
    symbol: String,
    uri: String,
    seller_fee_basis_points: u16,
    update_authority_is_signer: bool,
    is_mutable: bool,
    creators: Vec<Creator>,
) -> Result<()> {
    let ix = instruction::create_metadata_accounts(
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
pub struct CreateMetadataAccounts<'info> {
    pub metadata_account: AccountInfo<'info>,
    pub mint: AccountInfo<'info>,
    pub mint_authority: AccountInfo<'info>,
    pub payer: AccountInfo<'info>,
    pub update_authority: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn update_metadata_accounts<'a, 'b, 'c, 'info>(
    ctx: CpiContext<'a, 'b, 'c, 'info, UpdateMetadataAccounts<'info>>,
    primary_sale_happened: bool,
    data: Data,
) -> Result<()> {
    let ix = instruction::update_metadata_accounts(
        mpl_token_metadata::ID,
        ctx.accounts.metadata_account.key(),
        ctx.accounts.update_authority.key(),
        Some(ctx.accounts.new_update_authority.key()),
        Some(data),
        Some(primary_sale_happened),
    );
    solana_program::program::invoke_signed(
        &ix,
        &[ctx.accounts.metadata_account, ctx.accounts.update_authority],
        ctx.signer_seeds,
    )
    .map_err(Into::into)
}

#[derive(Accounts)]
pub struct UpdateMetadataAccounts<'info> {
    pub metadata_account: AccountInfo<'info>,
    pub update_authority: AccountInfo<'info>,
    pub new_update_authority: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn update_metadata_accounts_v2<'a, 'b, 'c, 'info>(
    ctx: CpiContext<'a, 'b, 'c, 'info, UpdateMetadataAccountsV2<'info>>,
    data: DataV2,
    primary_sale_happened: bool,
    is_mutable: bool,
) -> Result<()> {
    let ix = instruction::update_metadata_accounts_v2(
        mpl_token_metadata::ID,
        ctx.accounts.metadata_account.key(),
        ctx.accounts.update_authority.key(),
        Some(ctx.accounts.new_update_authority.key()),
        Some(data),
        Some(primary_sale_happened),
        Some(is_mutable),
    );
    solana_program::program::invoke_signed(
        &ix,
        &[ctx.accounts.metadata_account, ctx.accounts.update_authority],
        ctx.signer_seeds,
    )
    .map_err(Into::into)
}

#[derive(Accounts)]
pub struct UpdateMetadataAccountsV2<'info> {
    pub metadata_account: AccountInfo<'info>,
    pub update_authority: AccountInfo<'info>,
    pub new_update_authority: AccountInfo<'info>,
}

pub fn update_primary_sale_happened_via_token<'a, 'b, 'c, 'info>(
    ctx: CpiContext<'a, 'b, 'c, 'info, UpdatePrimarySaleHappenedViaToken<'info>>,
) -> Result<()> {
    let ix = instruction::update_primary_sale_happened_via_token(
        mpl_token_metadata::ID,
        ctx.accounts.metadata_account.key(),
        ctx.accounts.owner.key(),
        ctx.accounts.token.key(),
    );
    solana_program::program::invoke_signed(
        &ix,
        &[
            ctx.accounts.metadata_account,
            ctx.accounts.owner,
            ctx.accounts.token,
        ],
        ctx.signer_seeds,
    )
    .map_err(Into::into)
}

#[derive(Accounts)]
pub struct UpdatePrimarySaleHappenedViaToken<'info> {
    pub metadata_account: AccountInfo<'info>,
    pub owner: AccountInfo<'info>,
    pub token: AccountInfo<'info>,
}

pub fn create_master_edition<'a, 'b, 'c, 'info>(
    ctx: CpiContext<'a, 'b, 'c, 'info, CreateMasterEdition<'info>>,
    max_supply: u64,
) -> Result<()> {
    let ix = instruction::create_master_edition(
        mpl_token_metadata::ID,
        ctx.accounts.edition.key(),
        ctx.accounts.mint.key(),
        ctx.accounts.update_authority.key(),
        ctx.accounts.mint_authority.key(),
        ctx.accounts.metadata.key(),
        ctx.accounts.payer.key(),
        Some(max_supply),
    );
    solana_program::program::invoke_signed(
        &ix,
        &[
            ctx.accounts.edition,
            ctx.accounts.mint,
            ctx.accounts.update_authority,
            ctx.accounts.mint_authority,
            ctx.accounts.payer,
            ctx.accounts.metadata,
            ctx.accounts.token_program.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
            ctx.accounts.rent.to_account_info(),
        ],
        ctx.signer_seeds,
    )
    .map_err(Into::into)
}

#[derive(Accounts)]
pub struct CreateMasterEdition<'info> {
    pub edition: AccountInfo<'info>,
    pub mint: AccountInfo<'info>,
    pub update_authority: AccountInfo<'info>,
    pub mint_authority: AccountInfo<'info>,
    pub metadata: AccountInfo<'info>,
    pub payer: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn create_master_edition_v3<'a, 'b, 'c, 'info>(
    ctx: CpiContext<'a, 'b, 'c, 'info, CreateMasterEditionV3<'info>>,
    max_supply: u64,
) -> Result<()> {
    let ix = instruction::create_master_edition_v3(
        mpl_token_metadata::ID,
        ctx.accounts.edition.key(),
        ctx.accounts.mint.key(),
        ctx.accounts.update_authority.key(),
        ctx.accounts.mint_authority.key(),
        ctx.accounts.metadata.key(),
        ctx.accounts.payer.key(),
        Some(max_supply),
    );
    solana_program::program::invoke_signed(
        &ix,
        &[
            ctx.accounts.edition,
            ctx.accounts.mint,
            ctx.accounts.update_authority,
            ctx.accounts.mint_authority,
            ctx.accounts.payer,
            ctx.accounts.metadata,
            ctx.accounts.token_program.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
            ctx.accounts.rent.to_account_info(),
        ],
        ctx.signer_seeds,
    )
    .map_err(Into::into)
}

#[derive(Accounts)]
pub struct CreateMasterEditionV3<'info> {
    pub edition: AccountInfo<'info>,
    pub mint: AccountInfo<'info>,
    pub update_authority: AccountInfo<'info>,
    pub mint_authority: AccountInfo<'info>,
    pub metadata: AccountInfo<'info>,
    pub payer: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}

// pub fn mint_new_edition_from_master_edition_via_token<'a, 'b, 'c, 'info>(
//     ctx: CpiContext<'a, 'b, 'c, 'info, MintNewEditionFromMasterEditionViaToken<'info>>,
//     edition: u64,
// ) -> Result<()> {
//     let ix = instruction::mint_new_edition_from_master_edition_via_token(
//         mpl_token_metadata::ID,
//        ctx.accounts.new_metadata.key(),
//        ctx.accounts.new_edition.key(),
//        ctx.accounts.master_edition.key(),
//        ctx.accounts.new_mint.key(),
//        ctx.accounts.new_mint_authority.key(),
//        ctx.accounts.payer.key(),
//        ctx.accounts.token_account_owner.key(),
//        ctx.accounts.token_account.key(),
//        ctx.accounts.new_metadata_update_authority.key(),
//        ctx.accounts.metadata.key(),
//        ctx.accounts.metadata_mint.key(),
//        edition
//     );
//     let edition_number = edition.checked_div(EDITION_MARKER_BIT_SIZE).unwrap();
//     let as_string = edition_number.to_string();
//     let (edition_mark_pda, _) = Pubkey::find_program_address(
//         &[
//             PREFIX.as_bytes(),
//             mpl_token_metadata::ID.as_ref(),
//             ctx.accounts.metadata_mint.key().as_ref(),
//             EDITION.as_bytes(),
//             as_string.as_bytes(),
//         ],
//         &mpl_token_metadata::ID,
//     );
//     // AccountMeta::new(new_metadata, false),
//     // AccountMeta::new(new_edition, false),
//     // AccountMeta::new(master_edition, false),
//     // AccountMeta::new(new_mint, false),
//     // AccountMeta::new(edition_mark_pda, false),
//     // AccountMeta::new_readonly(new_mint_authority, true),
//     // AccountMeta::new(payer, true),
//     // AccountMeta::new_readonly(token_account_owner, true),
//     // AccountMeta::new_readonly(token_account, false),
//     // AccountMeta::new_readonly(new_metadata_update_authority, false),
//     // AccountMeta::new_readonly(metadata, false),
//     // AccountMeta::new_readonly(spl_token::id(), false),
//     // AccountMeta::new_readonly(solana_program::system_program::id(), false),
//     // AccountMeta::new_readonly(sysvar::rent::id(), false),
//     solana_program::program::invoke_signed(
//         &ix,
//         &[
//            ctx.accounts.new_metadata,
//             ctx.accounts.new_edition,
//             ctx.accounts.master_edition,
//             ctx.accounts.new_mint,
//             edition_mark_pda.clone(),

//         ],
//         ctx.signer_seeds,
//     )
//     .map_err(Into::into)
// }

// #[derive(Accounts)]
// pub struct MintNewEditionFromMasterEditionViaToken<'info> {
//     pub new_metadata: AccountInfo<'info>,
//     pub new_edition: AccountInfo<'info>,
//     pub master_edition: AccountInfo<'info>,
//     pub new_mint: AccountInfo<'info>,
//     pub new_mint_authority: AccountInfo<'info>,
//     pub payer: AccountInfo<'info>,
//     pub token_account_owner: AccountInfo<'info>,
//     pub token_account: AccountInfo<'info>,
//     pub new_metadata_update_authority: AccountInfo<'info>,
//     pub metadata: AccountInfo<'info>,
//     pub metadata_mint: AccountInfo<'info>
// }

pub fn sign_metadata<'a, 'b, 'c, 'info>(
    ctx: CpiContext<'a, 'b, 'c, 'info, SignMetadata<'info>>,
) -> Result<()> {
    let ix = instruction::sign_metadata(
        mpl_token_metadata::ID,
        ctx.accounts.metadata.key(),
        ctx.accounts.creator.key(),
    );
    solana_program::program::invoke_signed(
        &ix,
        &[ctx.accounts.metadata, ctx.accounts.creator],
        ctx.signer_seeds,
    )
    .map_err(Into::into)
}

#[derive(Accounts)]
pub struct SignMetadata<'info> {
    pub metadata: AccountInfo<'info>,
    pub creator: AccountInfo<'info>,
}

pub fn remove_creator_verification<'a, 'b, 'c, 'info>(
    ctx: CpiContext<'a, 'b, 'c, 'info, RemoveCreatorVerification<'info>>,
) -> Result<()> {
    let ix = instruction::remove_creator_verification(
        mpl_token_metadata::ID,
        ctx.accounts.metadata.key(),
        ctx.accounts.creator.key(),
    );
    solana_program::program::invoke_signed(
        &ix,
        &[ctx.accounts.metadata, ctx.accounts.creator],
        ctx.signer_seeds,
    )
    .map_err(Into::into)
}

#[derive(Accounts)]
pub struct RemoveCreatorVerification<'info> {
    pub metadata: AccountInfo<'info>,
    pub creator: AccountInfo<'info>,
}

pub fn convert_master_edition_v1_to_v2<'a, 'b, 'c, 'info>(
    ctx: CpiContext<'a, 'b, 'c, 'info, ConvertMasterEditionV1toV2<'info>>,
) -> Result<()> {
    let ix = instruction::convert_master_edition_v1_to_v2(
        mpl_token_metadata::ID,
        ctx.accounts.master_edition.key(),
        ctx.accounts.one_time_auth.key(),
        ctx.accounts.printing_mint.key(),
    );
    solana_program::program::invoke_signed(
        &ix,
        &[
            ctx.accounts.master_edition,
            ctx.accounts.one_time_auth,
            ctx.accounts.printing_mint,
        ],
        ctx.signer_seeds,
    )
    .map_err(Into::into)
}

#[derive(Accounts)]
pub struct ConvertMasterEditionV1toV2<'info> {
    pub master_edition: AccountInfo<'info>,
    pub one_time_auth: AccountInfo<'info>,
    pub printing_mint: AccountInfo<'info>,
}

pub fn verify_collection<'a, 'b, 'c, 'info>(
    ctx: CpiContext<'a, 'b, 'c, 'info, VerifyCollection<'info>>,
) -> Result<()> {
    let ix = instruction::verify_collection(
        mpl_token_metadata::ID,
        ctx.accounts.metadata.key(),
        ctx.accounts.collection_authority.key(),
        ctx.accounts.payer.key(),
        ctx.accounts.collection_mint.key(),
        ctx.accounts.collection.key(),
        ctx.accounts.collection_master_edition_account.key(),
        Some(ctx.accounts.collection_authority_record.key()),
    );

    solana_program::program::invoke_signed(
        &ix,
        &[
            ctx.accounts.metadata,
            ctx.accounts.collection_authority,
            ctx.accounts.payer,
            ctx.accounts.collection_mint,
            ctx.accounts.collection,
            ctx.accounts.collection_master_edition_account,
        ],
        ctx.signer_seeds,
    )
    .map_err(Into::into)
}

#[derive(Accounts)]
pub struct VerifyCollection<'info> {
    pub metadata: AccountInfo<'info>,
    pub collection_authority: AccountInfo<'info>,
    pub payer: AccountInfo<'info>,
    pub collection_mint: AccountInfo<'info>,
    pub collection: AccountInfo<'info>,
    pub collection_master_edition_account: AccountInfo<'info>,
    pub collection_authority_record: AccountInfo<'info>,
}

pub fn unverify_collection<'a, 'b, 'c, 'info>(
    ctx: CpiContext<'a, 'b, 'c, 'info, UnverifyCollection<'info>>,
) -> Result<()> {
    let ix = instruction::unverify_collection(
        mpl_token_metadata::ID,
        ctx.accounts.metadata.key(),
        ctx.accounts.collection_authority.key(),
        ctx.accounts.collection_mint.key(),
        ctx.accounts.collection.key(),
        ctx.accounts.collection_master_edition_account.key(),
        Some(ctx.accounts.collection_authority_record.key()),
    );

    solana_program::program::invoke_signed(
        &ix,
        &[
            ctx.accounts.metadata,
            ctx.accounts.collection_authority,
            ctx.accounts.collection_mint,
            ctx.accounts.collection,
            ctx.accounts.collection_master_edition_account,
        ],
        ctx.signer_seeds,
    )
    .map_err(Into::into)
}

#[derive(Accounts)]
pub struct UnverifyCollection<'info> {
    pub metadata: AccountInfo<'info>,
    pub collection_authority: AccountInfo<'info>,
    pub collection_mint: AccountInfo<'info>,
    pub collection: AccountInfo<'info>,
    pub collection_master_edition_account: AccountInfo<'info>,
    pub collection_authority_record: AccountInfo<'info>,
}

pub fn utilize<'a, 'b, 'c, 'info>(
    ctx: CpiContext<'a, 'b, 'c, 'info, Utilize<'info>>,
    number_of_uses: u64,
) -> Result<()> {
    let ix = instruction::utilize(
        mpl_token_metadata::ID,
        ctx.accounts.metadata.key(),
        ctx.accounts.token_account.key(),
        ctx.accounts.mint.key(),
        Some(ctx.accounts.use_authority_record_pda.key()),
        ctx.accounts.use_authority.key(),
        ctx.accounts.owner.key(),
        Some(ctx.accounts.burner.key()),
        number_of_uses,
    );

    solana_program::program::invoke_signed(
        &ix,
        &[
            ctx.accounts.metadata,
            ctx.accounts.token_account,
            ctx.accounts.mint,
            ctx.accounts.use_authority,
            ctx.accounts.owner,
            ctx.accounts.token_program.to_account_info(),
            ctx.accounts.associated_token.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
            ctx.accounts.rent.to_account_info(),
        ],
        ctx.signer_seeds,
    )
    .map_err(Into::into)
}

#[derive(Accounts)]
pub struct Utilize<'info> {
    pub metadata: AccountInfo<'info>,
    pub token_account: AccountInfo<'info>,
    pub mint: AccountInfo<'info>,
    pub use_authority_record_pda: AccountInfo<'info>,
    pub use_authority: AccountInfo<'info>,
    pub owner: AccountInfo<'info>,
    pub burner: AccountInfo<'info>,
    pub associated_token: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn approve_use_authority<'a, 'b, 'c, 'info>(
    ctx: CpiContext<'a, 'b, 'c, 'info, ApproveUseAuthority<'info>>,
    number_of_uses: u64,
) -> Result<()> {
    let ix = instruction::approve_use_authority(
        mpl_token_metadata::ID,
        ctx.accounts.use_authority_record.key(),
        ctx.accounts.user.key(),
        ctx.accounts.owner.key(),
        ctx.accounts.payer.key(),
        ctx.accounts.owner_token_account.key(),
        ctx.accounts.metadata.key(),
        ctx.accounts.mint.key(),
        ctx.accounts.burner.key(),
        number_of_uses,
    );

    solana_program::program::invoke_signed(
        &ix,
        &[
            ctx.accounts.use_authority_record,
            ctx.accounts.owner,
            ctx.accounts.payer,
            ctx.accounts.user,
            ctx.accounts.owner_token_account,
            ctx.accounts.metadata.to_account_info(),
            ctx.accounts.mint.to_account_info(),
            ctx.accounts.burner.to_account_info(),
            ctx.accounts.token_program.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
            ctx.accounts.rent.to_account_info(),
        ],
        ctx.signer_seeds,
    )
    .map_err(Into::into)
}

#[derive(Accounts)]
pub struct ApproveUseAuthority<'info> {
    pub use_authority_record: AccountInfo<'info>,
    pub user: AccountInfo<'info>,
    pub owner: AccountInfo<'info>,
    pub payer: AccountInfo<'info>,
    pub owner_token_account: AccountInfo<'info>,
    pub metadata: AccountInfo<'info>,
    pub mint: AccountInfo<'info>,
    pub burner: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn revoke_use_authority<'a, 'b, 'c, 'info>(
    ctx: CpiContext<'a, 'b, 'c, 'info, RevokeUseAuthority<'info>>,
) -> Result<()> {
    let ix = instruction::revoke_use_authority(
        mpl_token_metadata::ID,
        ctx.accounts.use_authority_record.key(),
        ctx.accounts.user.key(),
        ctx.accounts.owner.key(),
        ctx.accounts.owner_token_account.key(),
        ctx.accounts.metadata.key(),
        ctx.accounts.mint.key(),
    );

    solana_program::program::invoke_signed(
        &ix,
        &[
            ctx.accounts.use_authority_record,
            ctx.accounts.owner,
            ctx.accounts.user,
            ctx.accounts.owner_token_account,
            ctx.accounts.mint.to_account_info(),
            ctx.accounts.metadata.to_account_info(),
            ctx.accounts.token_program.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
            ctx.accounts.rent.to_account_info(),
        ],
        ctx.signer_seeds,
    )
    .map_err(Into::into)
}

#[derive(Accounts)]
pub struct RevokeUseAuthority<'info> {
    pub use_authority_record: AccountInfo<'info>,
    pub user: AccountInfo<'info>,
    pub owner: AccountInfo<'info>,
    pub owner_token_account: AccountInfo<'info>,
    pub metadata: AccountInfo<'info>,
    pub mint: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn approve_collection_authority<'a, 'b, 'c, 'info>(
    ctx: CpiContext<'a, 'b, 'c, 'info, ApproveCollectionAuthority<'info>>,
) -> Result<()> {
    let ix = instruction::approve_collection_authority(
        mpl_token_metadata::ID,
        ctx.accounts.collection_authority_record.key(),
        ctx.accounts.new_collection_authority.key(),
        ctx.accounts.update_authority.key(),
        ctx.accounts.payer.key(),
        ctx.accounts.metadata.key(),
        ctx.accounts.mint.key(),
    );

    solana_program::program::invoke_signed(
        &ix,
        &[
            ctx.accounts.collection_authority_record,
            ctx.accounts.new_collection_authority,
            ctx.accounts.update_authority,
            ctx.accounts.payer,
            ctx.accounts.metadata,
            ctx.accounts.mint,
            ctx.accounts.system_program.to_account_info(),
            ctx.accounts.rent.to_account_info(),
        ],
        ctx.signer_seeds,
    )
    .map_err(Into::into)
}

#[derive(Accounts)]
pub struct ApproveCollectionAuthority<'info> {
    pub collection_authority_record: AccountInfo<'info>,
    pub new_collection_authority: AccountInfo<'info>,
    pub update_authority: AccountInfo<'info>,
    pub payer: AccountInfo<'info>,
    pub metadata: AccountInfo<'info>,
    pub mint: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn revoke_collection_authority<'a, 'b, 'c, 'info>(
    ctx: CpiContext<'a, 'b, 'c, 'info, RevokeCollectionAuthority<'info>>,
) -> Result<()> {
    let ix = instruction::revoke_collection_authority(
        mpl_token_metadata::ID,
        ctx.accounts.collection_authority_record.key(),
        ctx.accounts.delegate_authority.key(),
        ctx.accounts.update_authority.key(),
        ctx.accounts.metadata.key(),
        ctx.accounts.mint.key(),
    );

    solana_program::program::invoke_signed(
        &ix,
        &[
            ctx.accounts.collection_authority_record,
            ctx.accounts.delegate_authority,
            ctx.accounts.update_authority,
            ctx.accounts.metadata,
            ctx.accounts.mint,
        ],
        ctx.signer_seeds,
    )
    .map_err(Into::into)
}

#[derive(Accounts)]
pub struct RevokeCollectionAuthority<'info> {
    pub collection_authority_record: AccountInfo<'info>,
    pub delegate_authority: AccountInfo<'info>,
    pub update_authority: AccountInfo<'info>,
    pub metadata: AccountInfo<'info>,
    pub mint: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn set_and_verify_collection<'a, 'b, 'c, 'info>(
    ctx: CpiContext<'a, 'b, 'c, 'info, SetAndVerifyCollection<'info>>,
) -> Result<()> {
    let ix = instruction::set_and_verify_collection(
        mpl_token_metadata::ID,
        ctx.accounts.metadata.key(),
        ctx.accounts.collection.key(),
        ctx.accounts.payer.key(),
        ctx.accounts.update_authority.key(),
        ctx.accounts.collection_mint.key(),
        ctx.accounts.collection.key(),
        ctx.accounts.collection_master_edition_account.key(),
        Some(ctx.accounts.collection_authority_record.key()),
    );

    solana_program::program::invoke_signed(
        &ix,
        &[
            ctx.accounts.metadata,
            ctx.accounts.collection_authority,
            ctx.accounts.payer,
            ctx.accounts.update_authority,
            ctx.accounts.collection_mint,
            ctx.accounts.collection,
            ctx.accounts.collection_master_edition_account,
        ],
        ctx.signer_seeds,
    )
    .map_err(Into::into)
}

#[derive(Accounts)]
pub struct SetAndVerifyCollection<'info> {
    pub metadata: AccountInfo<'info>,
    pub collection_authority: AccountInfo<'info>,
    pub payer: AccountInfo<'info>,
    pub update_authority: AccountInfo<'info>,
    pub collection_mint: AccountInfo<'info>,
    pub collection: AccountInfo<'info>,
    pub collection_master_edition_account: AccountInfo<'info>,
    pub collection_authority_record: AccountInfo<'info>,
}
