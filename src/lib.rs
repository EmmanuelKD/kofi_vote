use solana_program::{
    account_info::AccountInfo,
    entrypoint,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    msg,
};

entrypoint!(my_first_instruction_processor);

fn my_first_instruction_processor(
    program_id: &Pubkey,
    account: &[AccountInfo],
    instruction_data: &[u8]
) -> ProgramResult {
    msg!("Hello king");
    Ok(())
}
