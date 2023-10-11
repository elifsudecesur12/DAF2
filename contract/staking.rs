use solana_program::{
    account_info::AccountInfo,
    entrypoint,
    entrypoint::ProgramResult,
    program::invoke,
    program_error::ProgramError,
    pubkey::Pubkey,
    system_instruction,
};
use std::mem;

const ANNUAL_INTEREST_RATE: f64 = 0.025; 

entrypoint!(process_instruction);

fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {

    if instruction_data[0] == 0 {
        let amount = u64::from_le_bytes(instruction_data[1..9].try_into().unwrap());
        let staker_account = &accounts[0];

      
        let rewards = (amount as f64 * ANNUAL_INTEREST_RATE / 365.0).round() as u64;

      

        let rewards_account = &accounts[1];
        invoke(
            &system_instruction::transfer(rewards_account.key, staker_account.key, rewards),
            &[
                rewards_account.clone(),
                staker_account.clone(),
            ],
        )?;


    } else if instruction_data[0] == 1 {
       
        let amount = u64::from_le_bytes(instruction_data[1..9].try_into().unwrap());
        
    } else {
        return Err(ProgramError::InvalidInstructionData);
    }

    Ok(())
}
