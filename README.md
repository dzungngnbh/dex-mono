# services
1. Run clickhouse, dragonfly for indexer
2. web-app for frontend ( can get prebuilt in target )
	cargo run -r ( to have config file in same dev folder )
3. indexer service to sync price
	$env:RUST_LOG='info'; cargo run --bin indexer_service

# smart contract
1. Run local anvil and deploy smart contract