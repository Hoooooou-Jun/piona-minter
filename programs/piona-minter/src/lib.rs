use anchor_lang::{ prelude::*, solana_program::program::invoke, system_program };
use anchor_spl::{ associated_token, token };
use mpl_token_metadata::{ ID as TOKEN_METADATA_ID, instructions as token_instruction };

declare_id!("3VMvufM48KC5U57LvLqMUNkRzzvRPYMnipAEHdCZrmxR");

#[program]
pub mod piona_minter {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
