pub mod db;
pub mod parsers;

use parsers::polkadot_parser;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    polkadot_parser::run().await
}
