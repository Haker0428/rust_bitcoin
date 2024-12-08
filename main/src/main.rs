use core::blockchain;
use std::thread;
use std::time::Duration;

const MINNNG_TIME: u64 = 2;

fn main() {
    let mut bc = blockchain::Blockchain::new_blockchain();

    println!("start mining...");
    thread::sleep(Duration::from_secs(MINNNG_TIME));
    bc.add_block("a->b: 5btc".to_string());

    println!("produce a block! mining 2nd block...");
    thread::sleep(Duration::from_secs(MINNNG_TIME));
    bc.add_block("b->c: 2btc".to_string());

    println!("produce a block! mining 3rd block...");
    thread::sleep(Duration::from_secs(MINNNG_TIME));
    bc.add_block("a->c: 6btc".to_string());

    for b in bc.blocks {
        println!("++++++++++++++++++++++++");
        println!("{:#?}", b);
        println!("");
    }
}
