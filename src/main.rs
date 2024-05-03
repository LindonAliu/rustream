extern crate m3u;
use m3u::Reader;

fn main() {
    let filename = std::env::args().nth(1).unwrap();
    let mut reader = Reader::open_iptv(filename).unwrap();
    let chans: Vec<_> = reader.iptv_entries().map(Result::unwrap).collect();

    for chan in chans {
        println!("{:?}", chan);
    }
}
