use clap::Parser;
use latin_dictionary::get_lns_key_headword;
use latin_dictionary::query_asvocab_headword;
use latin_dictionary::query_clc4_headword;
use latin_dictionary::query_gcse_latin_headword;
use latin_dictionary::query_wwords_headword;
// use std::env;
use std::sync::Arc;
use std::thread;

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
    let mut threads = vec![];

    let pool0 = pool.clone();
    let term0 = term.clone();
    threads.push(thread::spawn({
        move || {
            let conn = &mut pool0.get().unwrap();
            get_lns_key_headword(&term0, conn).unwrap()
        }
    }));

    let pool1 = pool.clone();
    let term1 = term.clone();
    threads.push(thread::spawn({
        move || {
            let conn = &mut pool1.get().unwrap();
            query_gcse_latin_headword(&term1, conn).unwrap()
        }
    }));

    let pool2 = pool.clone();
    let term2 = term.clone();
    threads.push(thread::spawn({
        move || {
            let conn = &mut pool2.get().unwrap();
            query_clc4_headword(&term2, conn).unwrap()
        }
    }));

    let pool3 = pool.clone();
    let term3 = term.clone();
    threads.push(thread::spawn({
        move || {
            let conn = &mut pool3.get().unwrap();
            query_asvocab_headword(&term3, conn).unwrap()
        }
    }));

    let pool4 = pool.clone();
    let term4 = term.clone();
    threads.push(thread::spawn({
        move || {
            let conn = &mut pool4.get().unwrap();
            query_wwords_headword(&term4, conn).unwrap()
        }
    }));

    let mut results: Vec<String> = Vec::new();
    for handle in threads {
        let res = handle.join().unwrap();
        results.push(res);
    }

    let res_str = results.join(", ");
    println!("{{{}}}", res_str);
}
