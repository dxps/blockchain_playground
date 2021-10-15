use akasha::{now, Block, Blockchain};

fn main() {
    let difficulty = 0x0000ffffffffffffffffffffffffffff;

    let mut block = Block::new(
        0,
        now(),
        vec![0; 32],
        0,
        "Genesis Block".to_owned(),
        difficulty,
    );
    block.mine();
    println!("Mined block: {:?}", &block);

    let mut prev_block_hash = block.hash.clone();

    let mut blockchain = Blockchain {
        blocks: vec![block],
    };

    println!("Verify: {}", &blockchain.verify());

    for i in 1..=10 {
        let mut block = Block::new(
            i,
            now(),
            prev_block_hash,
            0,
            "Some Block".to_owned(),
            difficulty,
        );
        block.mine();
        println!("Mined block: {:?}", &block);

        prev_block_hash = block.hash.clone();

        blockchain.blocks.push(block);

        println!("Verify: {}", &blockchain.verify());
    }
}
