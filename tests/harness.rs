use std::str::FromStr;

use fuels::{prelude::*, tx::Bytes32, types::{Bits256}};

#[tokio::test]
async fn test_function() {
    abigen!(Contract(
        name = "MyContract",
        abi = "./out/debug/test-contract-abi.json"
    ));
    let wallet = launch_provider_and_get_wallet().await;
    let contract_id = Contract::load_from(
        "./out/debug/test-contract.bin",
        LoadConfiguration::default(),
    )
    .unwrap()
    .deploy(&wallet, TxParameters::default())
    .await.unwrap();
    let contract_methods = MyContract::new(contract_id.clone(), wallet.clone()).methods();

    let key = Bits256::from_hex_str("0x0000000000000000000000000000000000000000000000000000000000000002").unwrap();
    let result = contract_methods.test_function(key, vec![1, 2]).submit().await.unwrap();
    println!("Result {:#?}", result.response().await.unwrap().value);

    let result = contract_methods.return_vec(key).call().await.unwrap();
    println!("Result vec {:#?}", result.value);
}
