use crate::db::models::NewMemento;
use crate::db::record_memento;
use dotenvy::dotenv;
use std::env;
use subxt::config::substrate::BlakeTwo256;
use subxt::config::Hasher;
use subxt::{OnlineClient, PolkadotConfig};

#[subxt::subxt(runtime_metadata_path = "./artifacts/polkadot_metadata_small.scale")]
pub mod polkadot {}

pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let polkadot_url = env::var("POLKADOT_URL").expect("POLKADOT_URL must be set");

    let api = OnlineClient::<PolkadotConfig>::from_url(polkadot_url).await?;

    println!("Starting to monitor Polkadot blocks...");

    // Subscribe to all new blocks:
    let mut blocks_sub = api.blocks().subscribe_finalized().await?;

    while let Some(block) = blocks_sub.next().await {
        let block = block?;

        let block_number = block.header().number;

        println!("Parsing block: {}", block_number);

        let block_hash = block.hash();

        let extrinsics = block.extrinsics().await?;
        for ext in extrinsics.iter() {
            let ext = ext.unwrap();
            let ext_id = ext.index();

            let events = ext.events().await?;
            let ext_hash = format!("{:?}", BlakeTwo256::hash_of(&ext.bytes()));

            for evt in events.iter() {
                let evt = evt?;
                // let pallet_name = evt.pallet_name();
                let event_name = evt.variant_name();

                if event_name == "ExtrinsicFailed" {
                    let new_memento = NewMemento {
                        extrinsic_hash: ext_hash.clone().to_string(),
                        extrinsic_id: ext_id as i32,
                        block_id: block_number as i32,
                        block_hash: block_hash.to_string(),
                        minted: 0,
                    };

                    record_memento(&new_memento);
                    println!(
                        "Found failed extrisinc at block : {} with hash: {}",
                        block_number, ext_hash
                    );
                }
            }

            // println!("      Signed Extensions:");
            // if let Some(signed_extensions) = ext.signed_extensions() {
            //     for signed_extension in signed_extensions.iter() {
            //         let signed_extension = signed_extension.unwrap();
            //         let name = signed_extension.name();
            //         let value = signed_extension.value()?.to_string();
            // println!("        {name}: {value}");
            // }
            // }
        }
    }

    Ok(())
}
