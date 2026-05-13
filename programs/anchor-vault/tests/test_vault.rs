use anchor_lang::{
  prelude::Pubkey,
  AccountDeserialize
};
use {
  anchor_lang::{solana_program::instruction::Instruction, InstructionData, ToAccountMetas},
  litesvm::LiteSVM,
  solana_message::{Message},
  solana_keypair::Address,
  solana_transaction::Transaction,
  solana_signer::Signer,
  solana_keypair::Keypair,
};

fn setup() -> (LiteSVM, Keypair) {
  let program_id = anchor_vault::id();
  let payer = Keypair::new();
  let mut svm = LiteSVM::new();
  let bytes = include_bytes!("../../../target/deploy/anchor_vault.so");
  svm.add_program(program_id, bytes).unwrap();
  svm.airdrop(&payer.pubkey(), 1_000_000_000).unwrap();
  
  (svm, payer)
}

fn validate_state_data(svm: &LiteSVM, vault_state_pda: Address, vault_bump: u8, state_bump: u8) {
  let vault_state_account = svm.get_account(&vault_state_pda).unwrap();
  let mut data: &[u8] = &vault_state_account.data;
  let vault_state_account_data = anchor_vault::VaultState::try_deserialize(&mut data).unwrap();
  assert_eq!(vault_state_account_data.vault_bump, vault_bump);
  assert_eq!(vault_state_account_data.state_bump, state_bump);
}

#[test]
fn test_vault() {

  let (mut svm, payer) = setup();
  
  let (vault_state_pda, vault_state_bump) = Pubkey::find_program_address(
    &[
      b"vault_state",
      payer.pubkey().as_ref()
    ],
    &anchor_vault::id()
  );
  
  let (vault_pda, vault_bump) = Pubkey::find_program_address(
    &[
      b"vault",
      payer.pubkey().as_ref()
    ],
    &anchor_vault::id()
  );

  let initialize_instruction = Instruction::new_with_bytes(
      anchor_vault::id(),
      &anchor_vault::instruction::Initialize {}.data(),
      anchor_vault::accounts::Initialize {
        signer: payer.pubkey(),
        vault: vault_pda,
        vault_state: vault_state_pda,
        system_program: anchor_lang::system_program::ID
      }.to_account_metas(None),
  );

  let initialize_msg = Message::new(&[initialize_instruction], Some(&payer.pubkey()));
  let initialize_blockhash = svm.latest_blockhash();
  let initialize_tx = Transaction::new(&[&payer], initialize_msg, initialize_blockhash);
  
  svm.send_transaction(initialize_tx).unwrap();
  
  validate_state_data(&svm, vault_state_pda, vault_bump, vault_state_bump);
  
  let deposit_instruction = Instruction::new_with_bytes(
    anchor_vault::id(),
    &anchor_vault::instruction::Deposit {
      amount: 50_000
    }.data(),
    anchor_vault::accounts::Deposit {
      signer: payer.pubkey(),
      vault: vault_pda,
      vault_state: vault_state_pda,
      system_program: anchor_lang::system_program::ID
    }.to_account_metas(None)
  );
  
  let deposit_msg = Message::new(&[deposit_instruction], Some(&payer.pubkey()));
  let deposit_blockhash = svm.latest_blockhash();
  let deposit_tx = Transaction::new(&[&payer], deposit_msg, deposit_blockhash);
  
  svm.send_transaction(deposit_tx).unwrap();
  
  assert_eq!(svm.get_account(&vault_pda).unwrap().lamports, 50_000);
  
  let withdraw_instruction = Instruction::new_with_bytes(
    anchor_vault::id(),
    &anchor_vault::instruction::Withdraw {
      amount: 25_000
    }.data(),
    anchor_vault::accounts::Withdraw {
      signer: payer.pubkey(),
      vault: vault_pda,
      vault_state: vault_state_pda,
      system_program: anchor_lang::system_program::ID
    }.to_account_metas(None)
  );
  
  let withdraw_msg = Message::new(&[withdraw_instruction], Some(&payer.pubkey()));
  let withdraw_blockhash = svm.latest_blockhash();
  let withdraw_tx = Transaction::new(&[&payer], withdraw_msg, withdraw_blockhash);
  
  svm.send_transaction(withdraw_tx).unwrap();
  
  assert_eq!(svm.get_account(&vault_pda).unwrap().lamports, 25_000);
  
  let close_instruction = Instruction::new_with_bytes(
    anchor_vault::id(),
    &anchor_vault::instruction::Close {}.data(),
    anchor_vault::accounts::Close {
      signer: payer.pubkey(),
      vault: vault_pda,
      vault_state: vault_state_pda,
      system_program: anchor_lang::system_program::ID
    }.to_account_metas(None)
  );
  
  let close_msg = Message::new(&[close_instruction], Some(&payer.pubkey()));
  let close_blockhash = svm.latest_blockhash();
  let close_tx = Transaction::new(&[&payer], close_msg, close_blockhash);
  
  svm.send_transaction(close_tx).unwrap();
  
  assert!(svm.get_account(&vault_pda).is_none());
  assert!(svm.get_account(&vault_state_pda).is_none());
}
