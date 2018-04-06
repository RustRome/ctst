#[macro_use] extern crate quicli;
extern crate noise_search;
use quicli::prelude::*;
use noise_search::index::{Index,OpenOptions};
use noise_search::json_value::{JsonValue};

#[derive(Debug, StructOpt)]
struct Cli {
    search_args:String,
    
    #[structopt(long = "index", short = "i")]
    index:String,
}

fn get_string_value(vl:&JsonValue) -> String{
    match vl {
        &JsonValue::String(ref s) => {
            s.to_string()
        } 
        _ => {
            " ~~".to_string()
        }
    }
}

main!(|args: Cli| {
    let index = Index::open(&args.index,Some(OpenOptions::Create))?;
   
    let qr= args.search_args.split(" ").map(|x|format!("desc: ~= \"{}\"",x)).collect::<Vec<String>>().join(" && "); 
    let query = &format!("find {{{}}} \n return [.command, .desc]",qr);
    //println!("query: {}",query);
    let mut results = index.query(&query,None)?;

    println!("#command \t\t #description ");
    let mut res = results.next_result();
    while res.is_some(){
        let j= res.unwrap();
        match j {
            JsonValue::Array(val) => {
                println!("{}\t\t{}",get_string_value(&val[0]),get_string_value(&val[1]));
            }
            _ => {
                println!("gneee {:?}",j );
            }
        }
        res = results.next_result();
    }

});
