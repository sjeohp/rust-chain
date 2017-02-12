use transaction::*;
use block::*;
use util::*;

#[test]
fn test_block()
{
    let tx = Tx::new(
        vec![],
        vec![
            Txo {
                amount: 55555555,
                address: [1; 32]
            }
        ],
        9000);

    // 
    // println!("block hash: {:?}", to_hex_string(&block.block_hash));
    // println!("txs hash: {:?}", to_hex_string(&block.txs_hash));
}
