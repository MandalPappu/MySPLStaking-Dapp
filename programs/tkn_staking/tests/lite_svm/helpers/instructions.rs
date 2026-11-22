use {
    anchor_lang::{
        prelude::Pubkey,
        solana_program::{
            instruction::Instruction,
            system_program,
        },
        InstructionData,
        ToAccountMetas
    },
    anchor_spl::token,
    tkn_staking::{
        accounts,
        instruction,
        ID as PROGRAM_ID
    }
};

// =====================================================
// 1. INITIALIZE POOL INSTRUCTION
// =====================================================


pub fn initialize_pool_instruction(
    authority:Pubkey,
    staking_mint:Pubkey,
    reward_mint:Pubkey,
    pool:Pubkey,
    token_authority:Pubkey,
    reward_rate:u64
) -> Instruction {
    Instruction {
        program_id: PROGRAM_ID, 
        accounts: accounts::InitializePool {
            authority,
            staking_mint,
            reward_mint,
            pool,
            token_authority,
            token_program:token::ID,
            system_program:system_program::ID
        }.to_account_metas(None),
        data:instruction::InitializePool {
            reward_rate
        }
        .data(),
    }
}

// =====================================================
// 2. INITIALIZE VAULTS INSTRUCTION
// =====================================================

pub fn initialize_vaults_instruction(
    authority:Pubkey,
    pool:Pubkey,
    staking_mint:Pubkey,
    reward_mint:Pubkey,
    staking_vault:Pubkey,
    reward_vault:Pubkey
) -> Instruction {
    Instruction { 
        program_id: PROGRAM_ID, 
        accounts: accounts::InitializeVaults {
            authority,
            pool,
            staking_mint,
            reward_mint,
            staking_vault,
            reward_vault,
            token_program:token::ID,
            system_program:system_program::ID
        }.to_account_metas(None), 
        data: instruction::InitializeVaults {}.data(),
    }
}