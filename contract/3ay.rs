

use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint, entrypoint::ProgramResult,
    program_error::ProgramError,
    pubkey::Pubkey,
    sysvar::rent::Rent,
    system_instruction,
    clock::Clock,
    program,
};

entrypoint!(_entrypoint);

pub fn _entrypoint(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let user_account = next_account_info(accounts_iter)?;
    let company_account = next_account_info(accounts_iter)?;
    let pool_account = next_account_info(accounts_iter)?;
    let clock = Clock::get()?;
    let main_account = next_account_info(accounts_iter)?;
    let distribution_account = next_account_info(accounts_iter)?;

    let main_balance = main_account.lamports();

    if main_balance >= 250_000 {
        solana_program::program::invoke(
            &system_instruction::transfer(
                main_account.key,
                user_account.key,
                main_balance - 250_000,
            ),
            &[
                main_account.clone(),
                user_account.clone(),
            ],
        )?;
    } else {
        let user_balance = user_account.lamports();
        let cut_amount = user_balance / 100;

        solana_program::program::invoke(
            &system_instruction::transfer(
                user_account.key,
                company_account.key,
                cut_amount,
            ),
            &[user_account.clone(), company_account.clone()],
        )?;

        let pool_balance = pool_account.lamports();
        let user_pool_deposit = user_balance - cut_amount * 113 / 100;

        let current_time = clock.unix_timestamp;
        let pool_open_time = 10 * 24 * 60 * 60;
        let time = current_time - pool_open_time;

        if time <= 0 {
            solana_program::program::invoke(
                &system_instruction::transfer(
                    main_account.key,
                    user_account.key,
                    main_balance - 250_000,
                ),
                &[
                    main_account.clone(),
                    user_account.clone(),
                ],
            )?;
        } else {
            let remaining_pool_balance = pool_balance - user_pool_deposit;

            let last_payment_time = pool_account.data.borrow().get_u64(0);

            if current_time >= last_payment_time + (3 * 30 * 24 * 60 * 60) {
                solana_program::program::invoke(
                    &system_instruction::transfer(
                        distribution_account.key,
                        user_account.key,
                        user_pool_deposit,
                    ),
                    &[user_account.clone(), distribution_account.clone()],
                )?;

                solana_program::program::invoke(
                    &system_instruction::transfer(
                        user_account.key,
                        company_account.key,
                        remaining_pool_balance,
                    ),
                    &[user_account.clone(), company_account.clone()],
                )?;

                pool_account.data.borrow_mut().set_u64(0, current_time);
            }
        }
    }

    Ok(())
}