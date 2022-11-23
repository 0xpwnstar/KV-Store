use std::process::exit;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, )]
struct Args {

    #[arg(short, long)]
    get: Option<String>,
    
    #[arg(short, long, num_args = 2 )]
    set: Option<Vec<String>>,

    #[arg(short, long)]
    rm: Option<String>,
}



fn main() {
    let args = Args::parse();
    let mut kvs = kvs::KvStore::new();
    if let Some(key) = args.get {
        println!("{:#?}",kvs.get(key));
    }
    
    if let Some(mut key) = args.set {
        kvs.set(key.swap_remove(0), key.swap_remove(0));
        println!("{:#?}",kvs.get("ki".to_string()));
    }
    if let Some(_rm) = args.rm {
        eprintln!("unimplemented");
        exit(1);
    }



   

}