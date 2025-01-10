mod programs;
#[allow(unused_imports)]
use crate::programs::Turbin3_prereq::{CompleteArgs, Turbin3PrereqProgram, UpdateArgs};

use bs58;
#[allow(unused_imports)]
use dotenv::dotenv;
#[allow(unused_imports)]
use serde_json::Value;
#[allow(unused_imports)]
use solana_sdk::{
    pubkey::Pubkey,
    signature::{Keypair, Signer},
};
#[allow(unused_imports)]
use std::{
    env,
    io::{self, BufRead},
};

#[allow(dead_code)]
fn base58_to_wallet() {
    let mut input = String::new();
    println!("Paste your key as base58: ");
    io::stdin()
        .read_line(&mut input)
        .expect("Invalid input try again");
    let base58 = input.trim();
    let wallet = bs58::decode(base58).into_vec().unwrap();
    println!("{:?}", wallet);
    println!("======================================================")
}

#[allow(dead_code)]
fn wallet_to_base58() {
    let mut input = String::new();
    println!("Paste your key as wallet-file/byte array: ");
    io::stdin()
        .read_line(&mut input)
        .expect("Invalid input try again");

    let input = input
        .lines()
        .next()
        .unwrap()
        .trim_start_matches('[')
        .trim_end_matches(']')
        .split(',')
        .map(|s| s.trim().parse::<u8>().unwrap())
        .collect::<Vec<u8>>();

    let base58 = bs58::encode(input).into_string();
    println!("{:?}", base58);
}

#[allow(dead_code)]
fn get_key_pair() -> Vec<u8> {
    dotenv().ok();

    let kp = match env::var("KEY_PAIR") {
        Ok(value) => value,
        Err(e) => {
            println!("Error occured while fetching from .env {:?}", e);
            "Keypair is not available".to_string()
        }
    };

    let kp: Value = serde_json::from_str(&kp).unwrap();
    // println!("Keypair in non-byte form{:?}", kp);

    // Convert the Value into a byte array (Vec<u8>)
    let keypair_bytes = match kp.as_array() {
        Some(arr) => arr
            .iter()
            .filter_map(|x| x.as_u64())
            .map(|x| x as u8)
            .collect::<Vec<u8>>(),
        None => {
            println!("Error: Keypair data is not a valid array");
            let temp: Vec<u8> = vec![0];
            return temp;
        }
    };
    keypair_bytes
}

#[allow(dead_code)]
fn get_wallet_key_pair() -> Vec<u8> {
    dotenv().ok();

    let kp = match env::var("WALLET_KEY") {
        Ok(value) => value,
        Err(e) => {
            println!("Error occured while fetching from .env {:?}", e);
            "Keypair is not available".to_string()
        }
    };

    let kp: Value = serde_json::from_str(&kp).unwrap();
    // println!("Keypair in non-byte form{:?}", kp);

    // Convert the Value into a byte array (Vec<u8>)
    let keypair_bytes = match kp.as_array() {
        Some(arr) => arr
            .iter()
            .filter_map(|x| x.as_u64())
            .map(|x| x as u8)
            .collect::<Vec<u8>>(),
        None => {
            println!("Error: Keypair data is not a valid array");
            let temp: Vec<u8> = vec![0];
            return temp;
        }
    };
    keypair_bytes
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use serde_json::Value;
    use solana_client::rpc_client::RpcClient;
    use solana_sdk::{
        message::Message, native_token::LAMPORTS_PER_SOL, system_instruction::transfer,
        system_program, transaction::Transaction,
    };

    use super::*;

    #[test]
    #[allow(unused_variables)]
    fn keygen() {
        dotenv().ok();
        let keypair = Keypair::new();
        println!("Hears your public key {}", keypair.pubkey());
        println!("");
        println!("Hears your Keypair {:?}", keypair.to_bytes());
        println!("");

        let env_kp = match env::var("KEY_PAIR") {
            Ok(key) => key,
            Err(e) => {
                println!("{e}");
                "key is not available".to_string()
            }
        };
        let kp: Value = serde_json::from_str(&env_kp).unwrap();
        println!("✅ Your Official Keypair in JSON is: {kp}");
        println!("");

        println!("+++++++++++++++ Simple Rust CLI +++++++++++++++");

        loop {
            println!("1. If you want to convert base58 to wallet");
            println!("2. If you want to convert wallet to base58");
            println!("3. If you want to break the loop");

            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Invalid Input dude");
            let option = match input.trim().parse() {
                Ok(value) => value,
                Err(e) => {
                    println!("Something went wrong {e}");
                    0
                }
            };

            match option {
                0 => {
                    println!("Do You want to break the loop yes or no");
                    let mut input = String::new();
                    let a = String::from("value");
                    io::stdin().read_line(&mut input).expect("Invalid input");
                    let option = input.trim().to_lowercase();
                    match option.as_str() {
                        "yes" => {
                            break;
                        }
                        "no" => {
                            continue;
                        }
                        _ => {
                            println!("Invalid input");
                            continue;
                        }
                    }
                }
                1 => {
                    base58_to_wallet();
                    continue;
                }
                2 => {
                    wallet_to_base58();
                    continue;
                }
                3 => {
                    break;
                }
                _ => {
                    println!("Your input number is not in the option");
                    continue;
                }
            }
        }
    }

    #[test]
    fn airdop() {
        dotenv().ok();
        let url = "https://api.devnet.solana.com";
        let client = RpcClient::new(url);

        let keypair_bytes = get_key_pair();

        match Keypair::from_bytes(&keypair_bytes) {
            Ok(val) => {
                // lets make a airdrop
                match client.request_airdrop(&val.pubkey(), LAMPORTS_PER_SOL * 2) {
                    Ok(val) => {
                        println!("Success! Check out your TX here:");
                        println!(
                            "https://explorer.solana.com/tx/{}?cluster=devnet",
                            val.to_string()
                        );
                    }
                    Err(e) => {
                        println!(
                            "Oops error occures while trying request airdrop {:?}",
                            e.to_string()
                        )
                    }
                }
            }
            Err(e) => {
                println!("You got error while creating Keypair {:?}", e)
            }
        }
    }

    #[test]
    fn transfer_sol() {
        let wallet_keypair = get_key_pair();
        let url = "https://api.devnet.solana.com";

        let rpc_client = RpcClient::new(url);
        let recent_blockhash = rpc_client.get_latest_blockhash().unwrap();

        match Keypair::from_bytes(&wallet_keypair) {
            Ok(keypair) => {
                let to_pubkey =
                    Pubkey::from_str("12r4uFpQHVvVfX3qpAxHhddExdGMecFSZYPcVhxRPZNm").unwrap();

                let trx = Transaction::new_signed_with_payer(
                    &[transfer(&keypair.pubkey(), &to_pubkey, 1_000_000)],
                    Some(&keypair.pubkey()),
                    &vec![&keypair],
                    recent_blockhash,
                );

                let trx_sign = rpc_client.send_and_confirm_transaction(&trx).unwrap();

                println!(
                "Success! Check out your TX here: https://explorer.solana.com/tx/{}/?cluster=devnet", trx_sign
                );

                let balance = rpc_client.get_balance(&keypair.pubkey()).unwrap();

                let mock_trx = Message::new_with_blockhash(
                    &[transfer(&keypair.pubkey(), &to_pubkey, balance)],
                    Some(&keypair.pubkey()),
                    &recent_blockhash,
                );

                let fees = rpc_client.get_fee_for_message(&mock_trx).unwrap();

                let trx_all_sol = Transaction::new_signed_with_payer(
                    &[transfer(&keypair.pubkey(), &to_pubkey, balance - fees)],
                    Some(&keypair.pubkey()),
                    &vec![&keypair],
                    recent_blockhash,
                );

                let trx_all_sol_sign = rpc_client
                    .send_and_confirm_transaction(&trx_all_sol)
                    .unwrap();

                println!(
                "Success! Check out your TX here: https://explorer.solana.com/tx/{}/?cluster=devnet", trx_all_sol_sign
                );
            }
            Err(e) => {
                println!("Error occures while creating Keypair {e}")
            }
        }
    }

    #[test]
    fn enroll() {
        dotenv().ok();

        let wallet_keypair = get_wallet_key_pair();
        match Keypair::from_bytes(&wallet_keypair) {
            Ok(signer_keypair) => {
                let url = "https://api.devnet.solana.com";
                let rpc_client = RpcClient::new(url);

                let pre_req = Turbin3PrereqProgram::derive_program_address(&[
                    b"prereq",
                    signer_keypair.pubkey().to_bytes().as_ref(),
                ]);

                let args = CompleteArgs {
                    github: b"baindlapranayraj".to_vec(),
                };

                let blockhash = rpc_client
                    .get_latest_blockhash()
                    .expect("Failed to get recent blockhas");

                let transaction = Turbin3PrereqProgram::complete(
                    &[&signer_keypair.pubkey(), &pre_req, &system_program::id()],
                    &args,
                    Some(&signer_keypair.pubkey()),
                    &[&signer_keypair],
                    blockhash,
                );

                let signature = rpc_client
                    .send_and_confirm_transaction(&transaction)
                    .expect("Failed to send transaction");

                println!("Success! Check out your TX here:https://explorer.solana.com/tx/{}/?cluster=devnet", signature);
            }
            Err(e) => {
                println!("{:?}", e);
            }
        }
    }
}
