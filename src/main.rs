#[macro_use] extern crate quicli;
extern crate noise_search;
use quicli::prelude::*;
use noise_search::index::{Index,OpenOptions};

#[derive(Debug, StructOpt)]
struct Cli {
    search_args:String,
}


main!(|args: Cli| {
    println!("searchi {}",args.search_args);
    let index = Index::open("index",Some(OpenOptions::Create))?;
   
    let qr= args.search_args.split(" ").map(|x|format!("desc ~= \"{}\"",x)).collect::<Vec<String>>().join(" && "); 

    index.query(&format!("{{{}}}",qr),None)?;
});
