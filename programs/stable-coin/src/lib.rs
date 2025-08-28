use anchor_lang::prelude::*;
mod state;
mod instructions;
pub use instructions::*;
mod constants;
mod error;
declare_id!("ED7eNUqcLMZJMSVNFaTmb5qMxehDTe7iCfmZy48h8AZc");

#[program]
pub mod stable_coin {
    use crate::instructions::{process_initialize_config, InitializeConfig};

    use super::*;

    pub fn initialize_config(ctx: Context<InitializeConfig>) -> Result<()> {
        process_initialize_config(ctx)
    }

    pub fn update_config(ctx: Context<UpdateConfig>, health_factor: u64) -> Result<()> {
        process_update_config(ctx, health_factor)
    }

    pub fn deposit_collateral_and_mint_tokens(ctx: Context<DepositCollateralAndMintTokens>, amount_collateral: u64, amount_to_mint: u64) -> Result<()> {
        process_deposit_collateral_and_mint_tokens(ctx, amount_collateral, amount_to_mint)
    }
   
}

