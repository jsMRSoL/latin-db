use clap::Parser;
use latin_dictionary::get_lns_key;
use latin_dictionary::query_asvocab;
use latin_dictionary::query_clc4;
use latin_dictionary::query_gcse_latin;
use latin_dictionary::query_wwords;
use latin_dictionary::QueryFunc;
// use std::env;
use std::sync::Arc;
use std::thread;


#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    term: String,
}


fn main() {
    // let args: Vec<String> = env::args().collect();
    // println!("{:#?}", args);
    // match env::args().skip(1).next() {
    //     Some(term) => run_query(Arc::new(term)),
    //     None => eprintln!("No search term provided!"),
    // }
    let args = Args::parse();
    run_query(Arc::new(args.term));
}
fn run_query(term: Arc<String>) {
    let pool = latin_dictionary::get_connection_pool();
    let mut threads = vec![];
    let queries: [QueryFunc; 5] = [query_gcse_latin, query_clc4, query_asvocab, query_wwords, get_lns_key];

    for f in queries {
        let pool1 = pool.clone();
        let term1 = term.clone();
        threads.push(thread::spawn({
            move || {
                let conn = &mut pool1.get().expect("Could not get connection from pool");
                f(&term1, conn).expect("Database did not return result.")
            }
        }))
    }

    let mut results: Vec<String> = Vec::new();
    for handle in threads {
        let res = handle.join().unwrap();
        results.push(res);
    }

    let res_str = results.join(", ");
    println!("{{{}}}", res_str);

}

// fn run_query_old(term: Arc<String>) {
//     let pool = latin_dictionary::get_connection_pool();
//     let mut threads = vec![];
//
//     let pool0 = pool.clone();
//     let term0 = term.clone();
//     threads.push(thread::spawn({
//         move || {
//             let conn = &mut pool0.get().unwrap();
//             get_lns_key(&term0, conn).unwrap()
//         }
//     }));
//
//     let pool1 = pool.clone();
//     let term1 = term.clone();
//     threads.push(thread::spawn({
//         move || {
//             let conn = &mut pool1.get().unwrap();
//             query_gcse_latin(&term1, conn).unwrap()
//         }
//     }));
//
//     let pool2 = pool.clone();
//     let term2 = term.clone();
//     threads.push(thread::spawn({
//         move || {
//             let conn = &mut pool2.get().unwrap();
//             query_clc4(&term2, conn).unwrap()
//         }
//     }));
//
//     let pool3 = pool.clone();
//     let term3 = term.clone();
//     threads.push(thread::spawn({
//         move || {
//             let conn = &mut pool3.get().unwrap();
//             query_asvocab(&term3, conn).unwrap()
//         }
//     }));
//
//     let pool4 = pool.clone();
//     let term4 = term.clone();
//     threads.push(thread::spawn({
//         move || {
//             let conn = &mut pool4.get().unwrap();
//             query_wwords(&term4, conn).unwrap()
//         }
//     }));
//
//     let mut results: Vec<String> = Vec::new();
//     for handle in threads {
//         let res = handle.join().unwrap();
//         results.push(res);
//     }
//
//     let res_str = results.join(", ");
//     println!("{{{}}}", res_str);
// }
