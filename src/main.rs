mod balance;
mod transaction;
mod contract;

use contract::contract::{compile_deploy_contract, read_number, increment_number, reseat_number};
use transaction::transaction::send_transaction;
use ethers::providers::{Provider, Http};
use dotenv_codegen::dotenv;
use ethers::{utils, prelude::*};
use crate::balance::balance::print_balance;

type Client = SignerMiddleware<Provider<Http>, Wallet<k256::ecdsa::SigningKey>>;

#[tokio::main]
async fn main() -> Result<(), String> {
    let public_key = dotenv!("PUBLIC_KEY").parse::<Address>();
    let to_account_key = dotenv!("TO_ACCOUNT_KEY").parse::<Address>();
    let api_url = dotenv!("API_URL");

    if let Err(error) = public_key {
        return Err(format!("public_key: {}", error.to_string()));
    }

    if let Err(error) = to_account_key {
        return Err(format!("to_account_key: {}", error.to_string()));
    }

    let provider = Provider::<Http>::try_from(
        api_url
    );

    if let Err(error) = provider {
        return Err(format!("provider: {}", error.to_string()));
    }

    let provider = provider.unwrap();

    let wallet: LocalWallet = dotenv!("PRIVATE_KEY")
        .parse::<LocalWallet>().unwrap()
        .with_chain_id(Chain::Moonbase);

    let client = SignerMiddleware::new(provider.clone(), wallet.clone());

    print_balance(&provider, public_key.unwrap(), to_account_key.unwrap()).await;

    let transaction_result = send_transaction(
        &client, public_key.unwrap(),
         to_account_key.unwrap()
    ).await;

    if let Err(error) = transaction_result {
        println!("transaction error: {}", error);
    }

    let contract_addr_result = compile_deploy_contract(&client).await;
    if let Err(error) = &contract_addr_result {
        println!("{}", error)
    }

    let contract_addr = contract_addr_result.unwrap();

    println!("contract address: {:?}", contract_addr);

    let contract_addr = "0xe6bf01564f6ce56a7b498fd5df4f070a6171d74f".parse::<H160>().unwrap();
    read_number(&client, &contract_addr).await.unwrap();

    let increment_number = increment_number(&client, &contract_addr).await;
    if let Err(error) = increment_number {
        println!("Error: {}", error);
    }

    let reset_number = reseat_number(&client, &contract_addr).await;
    if let Err(error) = reset_number {
        println!("Error: {}", error);
    }
    let number = read_number(&client, &contract_addr).await.unwrap();
    
    println!("read_number: {}", number);

    Ok(())
}