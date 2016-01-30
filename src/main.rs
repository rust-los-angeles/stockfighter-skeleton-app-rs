extern crate stockfighter;

use stockfighter::Stockfighter;

fn main() {
    let sf = Stockfighter::new("my api key");

    // heartbeat does not require a valid API key
    sf.heartbeat().unwrap();
}
