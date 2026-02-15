use anchor_lang::prelude::*;
use arcium_anchor::prelude::*;

const COMP_DEF_OFFSET_RISK: u32 = comp_def_offset("check_risk");

declare_id!("85U5tQ7GmJgSvASvXfysodiKVpuc5teAsnpYzNRsaQst");

#[arcium_program]
pub mod arcperps {
    use super::*;

    pub fn init_config(ctx: Context<InitConfig>) -> Result<()> {
        init_comp_def(ctx.accounts, None, None)?;
        Ok(())
    }

    /// [New] Open a confidential position
    pub fn open_position(
        ctx: Context<OpenPosition>,
        enc_entry: [u8; 32],
        enc_size: [u8; 32],
        enc_col: [u8; 32],
        enc_side: [u8; 32],
    ) -> Result<()> {
        let pos = &mut ctx.accounts.position;
        pos.owner = ctx.accounts.owner.key();
        pos.encrypted_entry = enc_entry;
        pos.encrypted_size = enc_size;
        pos.encrypted_collateral = enc_col;
        pos.encrypted_side = enc_side;
        
        msg!("Confidential Position Opened. All data secret-shared.");
        Ok(())
    }

    /// [Core] Risk check / Liquidation trigger
    /// Keeper submits encrypted Mark Price, protocol reads on-chain positions, Arcium calculates liquidation status
    pub fn check_risk(
        ctx: Context<CheckRisk>,
        computation_offset: u64,
        enc_mark_price: [u8; 32], // Oracle price (encrypted)
        pubkey: [u8; 32],
        nonce: u128,
    ) -> Result<()> {
        let pos = &ctx.accounts.position;
        ctx.accounts.sign_pda_account.bump = ctx.bumps.sign_pda_account;
        
        let args = ArgBuilder::new()
            .x25519_pubkey(pubkey)
            .plaintext_u128(nonce)
            // Position Data (From Chain)
            .encrypted_u64(pos.encrypted_entry)
            .encrypted_u64(pos.encrypted_size)
            .encrypted_u64(pos.encrypted_collateral)
            .encrypted_u64(pos.encrypted_side)
            // Market Data (From Arg)
            .encrypted_u64(enc_mark_price)
            .plaintext_u64(500) // 5% Maint Margin (Hardcoded for demo)
            .build();

        queue_computation(
            ctx.accounts,
            computation_offset,
            args,
            vec![CheckRiskCallback::callback_ix(
                computation_offset,
                &ctx.accounts.mxe_account,
                &[]
            )?],
            1,
            0,
        )?;
        Ok(())
    }

    #[arcium_callback(encrypted_ix = "check_risk")]
    pub fn check_risk_callback(
        ctx: Context<CheckRiskCallback>,
        output: SignedComputationOutputs<CheckRiskOutput>,
    ) -> Result<()> {
        let o = match output.verify_output(&ctx.accounts.cluster_account, &ctx.accounts.computation_account) {
            Ok(CheckRiskOutput { field_0 }) => field_0,
            Err(_) => return Err(ErrorCode::AbortedComputation.into()),
        };

        // Parse results: { is_liquidatable, equity, pnl_is_positive }
        let liq_bytes: [u8; 8] = o.ciphertexts[0][0..8].try_into().unwrap();
        let equity_bytes: [u8; 8] = o.ciphertexts[1][0..8].try_into().unwrap();

        let is_liquidatable = u64::from_le_bytes(liq_bytes) == 1;
        let equity = u64::from_le_bytes(equity_bytes);

        if is_liquidatable {
            msg!("🚨 LIQUIDATION TRIGGERED! Equity: {}", equity);
            // Execute liquidation logic here (seize collateral)
        } else {
            msg!("✅ Position Safe. Equity: {}", equity);
        }

        emit!(RiskEvent {
            position: ctx.accounts.computation_account.key(),
            is_liquidatable,
            equity,
        });
        Ok(())
    }
}

// --- Accounts ---

#[derive(Accounts)]
pub struct OpenPosition<'info> {
    #[account(
        init, 
        payer = owner, 
        space = 8 + 32 + (32*4) + 1,
        seeds = [b"perp", owner.key().as_ref()],
        bump
    )]
    pub position: Account<'info, PerpAccount>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct PerpAccount {
    pub owner: Pubkey,
    pub encrypted_entry: [u8; 32],
    pub encrypted_size: [u8; 32],
    pub encrypted_collateral: [u8; 32],
    pub encrypted_side: [u8; 32], // 1=Long, 2=Short
}

#[queue_computation_accounts("check_risk", payer)]
#[derive(Accounts)]
#[instruction(computation_offset: u64)]
pub struct CheckRisk<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    pub position: Account<'info, PerpAccount>, // Read target position
    
    #[account(init_if_needed, space = 9, payer = payer, seeds = [&SIGN_PDA_SEED], bump, address = derive_sign_pda!())]
    pub sign_pda_account: Account<'info, ArciumSignerAccount>,
    #[account(address = derive_mxe_pda!())]
    pub mxe_account: Box<Account<'info, MXEAccount>>,
    #[account(mut, address = derive_mempool_pda!(mxe_account, ErrorCode::ClusterNotSet))]
    /// CHECK: Mempool
    pub mempool_account: UncheckedAccount<'info>,
    #[account(mut, address = derive_execpool_pda!(mxe_account, ErrorCode::ClusterNotSet))]
    /// CHECK: Execpool
    pub executing_pool: UncheckedAccount<'info>,
    #[account(mut, address = derive_comp_pda!(computation_offset, mxe_account, ErrorCode::ClusterNotSet))]
    /// CHECK: Comp
    pub computation_account: UncheckedAccount<'info>,
    #[account(address = derive_comp_def_pda!(COMP_DEF_OFFSET_RISK))]
    pub comp_def_account: Account<'info, ComputationDefinitionAccount>,
    #[account(mut, address = derive_cluster_pda!(mxe_account, ErrorCode::ClusterNotSet))]
    pub cluster_account: Account<'info, Cluster>,
    #[account(mut, address = ARCIUM_FEE_POOL_ACCOUNT_ADDRESS)]
    pub pool_account: Account<'info, FeePool>,
    #[account(mut, address = ARCIUM_CLOCK_ACCOUNT_ADDRESS)]
    pub clock_account: Account<'info, ClockAccount>,
    pub system_program: Program<'info, System>,
    pub arcium_program: Program<'info, Arcium>,
}

#[callback_accounts("check_risk")]
#[derive(Accounts)]
pub struct CheckRiskCallback<'info> {
    pub arcium_program: Program<'info, Arcium>,
    #[account(address = derive_comp_def_pda!(COMP_DEF_OFFSET_RISK))]
    pub comp_def_account: Account<'info, ComputationDefinitionAccount>,
    #[account(address = derive_mxe_pda!())]
    pub mxe_account: Box<Account<'info, MXEAccount>>,
    /// CHECK: Comp
    pub computation_account: UncheckedAccount<'info>,
    #[account(address = derive_cluster_pda!(mxe_account, ErrorCode::ClusterNotSet))]
    pub cluster_account: Account<'info, Cluster>,
    #[account(address = ::anchor_lang::solana_program::sysvar::instructions::ID)]
    /// CHECK: Sysvar
    pub instructions_sysvar: AccountInfo<'info>,
}

#[init_computation_definition_accounts("check_risk", payer)]
#[derive(Accounts)]
pub struct InitConfig<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(mut, address = derive_mxe_pda!())]
    pub mxe_account: Box<Account<'info, MXEAccount>>,
    #[account(mut)]
    /// CHECK: Def
    pub comp_def_account: UncheckedAccount<'info>,
    #[account(mut, address = derive_mxe_lut_pda!(mxe_account.lut_offset_slot))]
    /// CHECK: Lut
    pub address_lookup_table: UncheckedAccount<'info>,
    #[account(address = LUT_PROGRAM_ID)]
    /// CHECK: Lut Prog
    pub lut_program: UncheckedAccount<'info>,
    pub arcium_program: Program<'info, Arcium>,
    pub system_program: Program<'info, System>,
}

#[event]
pub struct RiskEvent {
    pub position: Pubkey,
    pub is_liquidatable: bool,
    pub equity: u64,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Aborted")] AbortedComputation,
    #[msg("No Cluster")] ClusterNotSet,
}