pub mod transaction {
    use ethers::abi::Address;
    use ethers::types::TransactionRequest;
    use crate::Client;
    use ethers::{utils, prelude::*};

    pub async fn send_transaction(client: &Client, address_from: Address, address_to: Address) -> Result<(), Box<dyn std::error::Error>> {
        println!(
            "Beginning transfer of 1 native currency from {} to {}.",
            address_from, address_to
        );

        // 2. Create a TransactionRequest object
        let tx = TransactionRequest::new()
            .to(address_to)
            .value(U256::from(utils::parse_ether(0.1)?))
            .from(address_from);

        // 3. Send the transaction with the client
        let tx = client.send_transaction(tx, None).await?.await?;

        // 4. Print out the result
        println!("Transaction Receipt: {}", serde_json::to_string(&tx)?);

        Ok(())
    }
}