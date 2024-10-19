use anyhow::Result;
use redis::Client;
use std::fs;

use merkle_tree_rs::standard::{LeafType, StandardMerkleTree, StandardMerkleTreeData};
use shared::db;

fn generate_merkle_proof(values: &Vec<Vec<String>>, output_name: &str) {
    let tree = StandardMerkleTree::of(
        values.clone(),
        &["address".to_string(), "uint256".to_string()],
    );

    let root = tree.root();
    println!("Merkle root: {}", root);

    let tree_json = serde_json::to_string(&tree.dump()).unwrap();
    fs::write(output_name, tree_json).unwrap()
}

fn read_merkle_proof(_input_name: &str) {
    let tree_json = fs::read_to_string("tree.json").unwrap();
    let tree_data: StandardMerkleTreeData = serde_json::from_str(&tree_json).unwrap();
    let tree = StandardMerkleTree::load(tree_data);

    for (i, v) in tree.clone().enumerate() {
        if v[0] == "0x1111111111111111111111111111111111111111" {
            let proof = tree.get_proof(LeafType::Number(i));
            println!("Value : {:?}", v);
            println!("Proof : {:?}", proof);
        }
    }
}

// Load merkle tree to a redis key to get proof faster
async fn save_to_redis(redis_client: &Client, key: &str, input_file: &str) -> Result<()> {
    let tree_json = fs::read_to_string(input_file)?.into_bytes();
    db::redis::set(redis_client, key, &tree_json).await?;
    Ok(())
}

async fn read_from_redis(redis_client: &Client, key: &str) -> Result<StandardMerkleTree> {
    let tree_json = db::redis::get_raw(redis_client, key).await?;
    let tree_json = String::from_utf8(tree_json)?;

    let tree_data: StandardMerkleTreeData = serde_json::from_str(&tree_json)?;
    Ok(StandardMerkleTree::load(tree_data))
}

mod tests {

    #[test]
    fn test_generate_merkle_proof() {
        // TODO: in real world application you need to create this kind of data to/from csv file
        let values = vec![
            vec![
                "0x1111111111111111111111111111111111111111".to_string(),
                "5000000000000000000".to_string(),
            ],
            vec![
                "0x2222222222222222222222222222222222222222".to_string(),
                "2500000000000000000".to_string(),
            ],
        ];
        generate_merkle_proof(&values, "tree.json");
    }

    #[test]
    fn test_read_merkle_proof() {
        read_merkle_proof("tree.json");
    }

    #[tokio::test]
    async fn test_save_to_redis() -> Result<()> {
        let redis_client = db::redis::get_redis_client()?;
        save_to_redis(&redis_client, "airdrop_firstweek", "tree.json").await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_read_from_redis() -> Result<()> {
        let redis_client = db::redis::get_redis_client()?;
        let tree = read_from_redis(&redis_client, "airdrop_firstweek").await?;
        println!("Tree: {:?}", tree);
        Ok(())
    }
}
