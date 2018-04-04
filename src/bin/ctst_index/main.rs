#[macro_use] extern crate quicli;
extern crate noise_search;
use quicli::prelude::*;
use noise_search::index::{Index,OpenOptions,Batch};
use std::fs::File;
use std::io::{BufRead,BufReader};



#[derive(Debug, StructOpt)]
struct Cli {
    #[structopt(long = "source", short = "s")]
    source:String,
    #[structopt(long = "index", short = "s")]
    index:String,
}

fn parse(source:&str) -> Result<Vec<(String,String)>> {
    let mut vec = Vec::<(String,String)>::new();
    let file = File::open(source)?;
    let reader = BufReader::new(file);
    let mut description:Option<String>= None;
    let mut command:Option<String> = None;
    for line in reader.lines() {
        let trimmed:String = line?.trim().to_string().clone();
        if trimmed.starts_with("#") {
            description = Some(trimmed.trim_left_matches('#').to_string());
        } else if trimmed.len() == 0 {
            if let Some(ref desc) = description {
                if  let Some(ref comm) = command {
                vec.push((comm.clone(),desc.clone()));
            }
            }
        } else  {
            command = Some(trimmed);
        }
    }

    Ok(vec)
}


main!(|args: Cli| {
    
    let mut index = Index::open(&args.index,Some(OpenOptions::Create))?;
    
    let res = parse(&args.source)?;
    let mut batch = Batch::new();
    for  (command,desc) in res {
        index.add(&format!("{{\"command\":\"{}\",\"desc\":\"{}\" }}",command,desc),&mut batch)?;
    }
    index.flush(batch)?;
});




