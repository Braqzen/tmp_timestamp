use fuels::{prelude::*, tx::ContractId};
use chrono::{DateTime, Utc, Duration};

abigen!(Contract(
    name = "MyContract",
    abi = "out/debug/tmp_timestamp-abi.json"
));

async fn get_contract_instance() -> (MyContract, ContractId, WalletUnlocked) {
    let config = Config {
        manual_blocks_enabled: true, // Necessary so the `produce_blocks` API can be used locally
        ..Config::local_node()
    };

    let mut wallets = launch_custom_provider_and_get_wallets(
        WalletsConfig::new(
            Some(1),
            Some(1),
            Some(1_000_000_000),
        ),
        Some(config),
        None,
    )
    .await;
    let wallet = wallets.pop().unwrap();

    let id = Contract::deploy(
        "./out/debug/tmp_timestamp.bin",
        &wallet,
        TxParameters::default(),
        StorageConfiguration::with_storage_path(Some(
            "./out/debug/tmp_timestamp-storage_slots.json".to_string(),
        )),
    )
    .await
    .unwrap();

    let instance = MyContract::new(id.clone(), wallet.clone());

    (instance, id.into(), wallet)
}

#[tokio::test]
async fn can_get_contract_id() {
    let (instance, _id, wallet) = get_contract_instance().await;

    let time = TimeParameters { 
        start_time: DateTime::from(Utc::now()) + Duration::seconds(10),
        block_time_interval: Duration::seconds(10),
    };

    let r1 = instance.methods().test_function().call().await.unwrap();

    wallet.get_provider().unwrap().produce_blocks(15, Some(time)).await.unwrap();

    let r2 = instance.methods().test_function().call().await.unwrap();
    
    println!("r1: {:?}", r1.get_logs().unwrap());
    println!("r2: {:?}", r2.get_logs().unwrap());

    assert!(false);
}
