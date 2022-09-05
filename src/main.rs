use clap::{App, Arg};

fn main() {
    let args = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(Arg::with_name("query").required(true).index(1))
        .get_matches();

    let query = args.value_of("query").unwrap();
 
    let img = image::open(query).unwrap().to_luma8();
    // Prepare for detection
    let mut img = rqrr::PreparedImage::prepare(img);
    // Search for grids, without decoding
    let grids = img.detect_grids();
    // Decode the grid
    let (_meta, content) = grids[0].decode().unwrap();
    print!("{}", content)
}
