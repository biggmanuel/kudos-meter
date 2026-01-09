use anchor_lang::prelude::*;

// This is a placeholder ID. We will update it after we build.
declare_id!("BrComrWp8AWEy3rDDxQohg8n7M7e5mLSrwiDRbGNNKu1");

#[program]
pub mod kudos_meter {
    use super::*;

    // INSTRUCTION 1: Create the Kudos Card
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let kudos_account = &mut ctx.accounts.kudos_account;
        kudos_account.count = 0; // Start at 0
        
        // We store the "bump" to verify this address later
        kudos_account.bump = ctx.bumps.kudos_account; 
        
        msg!("Kudos Card Initialized!");
        Ok(())
    }

    // INSTRUCTION 2: Increment the Counter
    pub fn give_kudos(ctx: Context<GiveKudos>) -> Result<()> {
        let kudos_account = &mut ctx.accounts.kudos_account;
        kudos_account.count += 1;
        msg!("High Five! Current Kudos: {}", kudos_account.count);
        Ok(())
    }
}

// CONTEXT 1: Validation for Initialize
#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub user: Signer<'info>, // The person paying to create the account

    #[account(
        init, 
        payer = user, 
        space = 8 + 8 + 1, // 8 byte discriminator + 8 byte u64 + 1 byte bump
        seeds = [b"kudos", user.key().as_ref()], // Unique seed: "kudos" + User's PubKey
        bump
    )]
    pub kudos_account: Account<'info, KudosAccount>,

    pub system_program: Program<'info, System>,
}

// CONTEXT 2: Validation for GiveKudos
#[derive(Accounts)]
pub struct GiveKudos<'info> {
    #[account(
        mut, // Mutable because we are changing the count
        seeds = [b"kudos", recipient.key().as_ref()], // Find the address using "kudos" + Recipient Key
        bump = kudos_account.bump,
    )]
    pub kudos_account: Account<'info, KudosAccount>,

    /// CHECK: We only need the Public Key to calculate the seeds above.
    pub recipient: AccountInfo<'info>, 
}

// STATE: The actual data structure stored on-chain
#[account]
pub struct KudosAccount {
    pub count: u64,
    pub bump: u8,
}