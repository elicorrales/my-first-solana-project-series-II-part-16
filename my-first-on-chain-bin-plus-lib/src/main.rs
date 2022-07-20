use my_first_on_chain_bin_plus_lib::process_instruction;
use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, msg, pubkey::Pubkey,
    stake_history::Epoch,
};
use std::env;

fn main() {
    println!("\nHello, world!\n");

    let args: Vec<String> = env::args().collect();

    let mut instruction_data = "not set".as_bytes();
    if args.len() > 1 {
        instruction_data = &args[1].as_bytes();
    }

    let mut account_data: u8 = 0;
    if args.len() > 2 {
        account_data = args[2].parse().unwrap();
    }
    let mut acct_data_array = [account_data];

    let program_id = Pubkey::default();

    let key = Pubkey::default();
    let mut lamports = 0;
    let owner = Pubkey::default();
    //let mut data = [1, 2, 3, 4, 5, 6, 7, 8, 9, 0];

    let account = AccountInfo::new(
        &key,                 //the Public key of the account
        false, //is_signer: bool, //Was the transaction signed by this account's public key?
        true,  //is_writable: bool, // Is the account writable?
        &mut lamports, //Rc<RefCell<&'a mut u64>>, // The lamports in the account.  Modifiable by programs.
        &mut acct_data_array, // Rc<RefCell<&'a mut [u8]>>, // The data held in this account.  Modifiable by programs.
        &owner,               //&'a Pubkey, // Program that owns this account
        false, //executable: bool, // This account's data contains a loaded program // (and is now read-only)
        Epoch::default(), //rent_epoch: Epoch, // The epoch at which this account will next owe rent
    );
    let accounts = [account];
    process_instruction(&program_id, &accounts, &instruction_data);
}
