use ethers::prelude::abigen;
// ../../contract/Incrementer_ABI.json
abigen!(
    Incrementer,
    "./contract/Incrementer_ABI.json",
    event_derives(serde::Deserialize, serde::Serialize)
);

pub mod contract {
    use crate::Client;
    use ethers_solc::Solc;
    use ethers::{prelude::*};
    use k256::elliptic_curve::consts::U25;
    use std::{path::Path, sync::Arc};
    use crate::contract::Incrementer;

    pub async fn compile_deploy_contract(client: &Client) -> Result<H160, Box<dyn std::error::Error>> {
        let path = format!("{}/contract/Incrementer.sol", env!("CARGO_MANIFEST_DIR"));
        let source = Path::new(path.as_str());

        let compiled = Solc::default()
            .compile_source(source)
            .expect("Could not compile contracts");

        let (abi, bytecode, _runtime_bytecode) = compiled
            .find("Incrementer")
            .expect("could not find contract")
            .into_parts_or_default();

        let factory = ContractFactory::new(abi, bytecode, Arc::new(client));

        let contract = factory
            .deploy(U256::from(5)).unwrap().send().await.unwrap();

        let addr = contract.address();
        println!("Incrementer.sol has been deployed to {:?}", addr);

        Ok(addr)
    }

    pub async fn read_number(client: &Client, contract_addr: &H160) -> Result<U256, Box<dyn std::error::Error>> {
        let contract = Incrementer::new(*contract_addr, Arc::new(client.clone()));
        
        let value = contract.number().call().await.unwrap();

        println!("Incrementer's number is {}", value);

        Ok(value)
    }

    pub async fn increment_number(client: &Client, contract_addr: &H160) -> Result<(), Box<dyn std::error::Error>> {
        println!("Incrementing number...");

        let contract = Incrementer::new(*contract_addr, Arc::new(client.clone()));

        let tx = contract.increment(U256::from(5)).send().await.unwrap().await.unwrap();
        println!("Transaction Receipt: {}", serde_json::to_string(&tx)?);

        Ok(())
    }

    pub async fn reseat_number(client: &Client, contract_addr: &H160) -> Result<(), Box<dyn std::error::Error>> {
        println!("Incrementing reseat");

        let contract = Incrementer::new(*contract_addr, Arc::new(client.clone()));

        let tx = contract.reset().send().await.unwrap().await.unwrap();
        println!("Transaction Receipt: {}", serde_json::to_string(&tx)?);

        Ok(())
    }
}