use solana_program::{
    account_info::AccountInfo, 
    entrypoint, 
    entrypoint::ProgramResult,
    msg, 
    pubkey::Pubkey,
    stake_history::Epoch,
};
use std::str;

// Declare and export the program's entrypoint
entrypoint!(process_instruction);

// Program entrypoint's implementation
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    msg!("This is the entry point.");
    msg!("This is the program_id: {:?}", program_id);
    msg!("These are the accounts: {:?}", accounts);
    msg!("This is the data: {:?}", instruction_data);
    let disp_inst_data = str::from_utf8(instruction_data).unwrap();
    msg!("This is instruction data: {:?}", disp_inst_data);

    let acct_iter = accounts.iter();
    for account in acct_iter {
        msg!("Account: {:?}", account);
        let mut data = account.data.borrow_mut();
        msg!("Account Data[]: {:?}", data);
        msg!("Account Data: {:?}", data[0]);

        match disp_inst_data {
            "increment" => {
                msg!("LIB: INCREMENT");
                data[0] += 1;
            }
            "decrement" => {
                msg!("LIB: DECREMENT");
                data[0] -= 1;
            }
            "reset" => {
                msg!("LIB: RESET");
                data[0] = 0;
            }
            _ => msg!("LIB: UNKNOWN: {:?}", disp_inst_data),
        }
        msg!("Account Data: {:?}", data[0]);
    }

    Ok(())
}
