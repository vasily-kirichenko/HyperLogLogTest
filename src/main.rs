extern crate hyperloglog; 
extern crate uuid;
extern crate time;

#[macro_use] 
mod utils;
 
use hyperloglog as hll;
use uuid::Uuid;

fn main() {
    let mut hll = hll::HLL::new(0.01625);
    let keys = (1..1000).map(|_| Uuid::new_v4().to_string()).collect::<Vec<_>>();
    let keys = keys.into_iter().cycle().take(50_000_000).collect::<Vec<_>>();
    
    let count = time!("total", {
        for k in keys {
            hll.insert(&k);
        }
    
        hll.count().round()
    });
    
    println!("hll.len().round() = {}", count);
    println!("Done.");
}
