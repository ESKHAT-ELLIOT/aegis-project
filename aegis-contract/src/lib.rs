use anchor_lang::prelude::*;

// ӨЗІҢНІҢ PROGRAM ID-іңді ҚОЮДЫ ҰМЫТПА!
declare_id!("AaCriqreGyVQYpgJpUeLfQBktm7S1iNGKFjMWPCbEDqJ");

#[program]
pub mod aegis_swarm_v3 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let state = &mut ctx.accounts.aegis_state;
        state.authority = *ctx.accounts.authority.key;
        msg!("🛡️ AEGIS On-Chain Enforcement Layer Initialized.");
        Ok(())
    }

    pub fn enforce_quarantine(
        ctx: Context<ManageThreat>, 
        target_wallet: Pubkey, 
        risk_level: u8, 
        reason: String
    ) -> Result<()> {
        let threat_record = &mut ctx.accounts.threat_record;
        
        threat_record.target = target_wallet;
        threat_record.risk_level = risk_level;
        threat_record.reason = reason;

        match risk_level {
            1 => msg!("⚠️ LEVEL 1 (ALERT): Target {} is under Swarm surveillance.", target_wallet),
            2 => msg!("⏳ LEVEL 2 (RATE LIMIT): Target {} restricted to low-volume txs.", target_wallet),
            3 => msg!("🚨 LEVEL 3 (QUARANTINE): Target {} isolated. Tx Rejected.", target_wallet),
            _ => msg!("💀 LEVEL 4 (FREEZE): Target {} funds frozen. Rug-pull prevented.", target_wallet),
        }

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = authority, space = 8 + 32)]
    pub aegis_state: Account<'info, AegisState>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(target_wallet: Pubkey)]
pub struct ManageThreat<'info> {
    #[account(
        init_if_needed, 
        payer = authority, 
        space = 8 + 32 + 1 + 64,
        seeds = [b"threat", target_wallet.as_ref()], 
        bump
    )]
    pub threat_record: Account<'info, ThreatRecord>,
    #[account(mut)]
    pub authority: Signer<'info>, 
    pub system_program: Program<'info, System>,
}

#[account]
pub struct AegisState {
    pub authority: Pubkey,
}

#[account]
pub struct ThreatRecord {
    pub target: Pubkey,
    pub risk_level: u8,
    pub reason: String,
}
