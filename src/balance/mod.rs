pub mod balance {
    use ethers::prelude::{Http, Provider};
    use ethers::{utils, prelude::*};

    pub async fn print_balance(provider: &Provider<Http>, address_from: Address, address_to: Address) {
        let balance_from = provider.get_balance(address_from, None).await;
        let balance_to = provider.get_balance(address_to, None).await;

        if let Err(error) = &balance_from {
            println!("Failed to get address_from: {}", error.to_string());
        }

        if let Err(error) = &balance_to {
            println!("Failed to get address_to: {}", error.to_string());
        }

        println!("Address from balance {}: {}", address_from, balance_from.unwrap());
        println!("Address to balance {}: {}", address_to, balance_to.unwrap());
    }
}