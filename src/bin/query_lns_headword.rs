use clap::Parser;
use latin_dictionary::get_lns_key_headword;
use std::sync::Arc;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    term: String,
}

fn main() {
    let args = Args::parse();
    run_query(Arc::new(args.term));
}

fn run_query(term: Arc<String>) {
    let pool = latin_dictionary::get_connection_pool();

    let conn = &mut pool.get().unwrap();
    let res_str = get_lns_key_headword(&term, conn).unwrap();

    println!("{{{}}}", res_str);
}
