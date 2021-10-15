use akasha::{Block, Hashable};

fn main() {
    let mut block = Block::new(0, 0, vec![0; 32], 0, "Genesis".to_owned());
    println!("{:?}", &block);

    let h = block.hash();
    println!("Block Hash: {:?}", &h);

    block.hash = h;
    println!("{:?}", &block);
}
