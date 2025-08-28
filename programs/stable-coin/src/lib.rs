use anchor_lang::prelude::*;
mod state;
mod instructions;
pub use instructions::*;
mod constants;
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

   
}

