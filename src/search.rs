use crate::parsing::parse_entry;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

lazy_static! {
    pub static ref XML_FILES: HashMap<char, &'static str> = {
        let mut m = HashMap::new();
        m.insert('a', "a.xml");
        m.insert('b', "b.xml");
        m.insert('c', "c.xml");
        m.insert('d', "d.xml");
        m.insert('e', "e.xml");
        m.insert('f', "f.xml");
        m.insert('g', "g.xml");
        m.insert('h', "h.xml");
        m.insert('i', "i.xml");
        m.insert('j', "i.xml");
        m.insert('k', "k.xml");
        m.insert('l', "l.xml");
        m.insert('m', "m.xml");
        m.insert('n', "n.xml");
        m.insert('o', "o.xml");
        m.insert('p', "p.xml");
        m.insert('q', "q.xml");
        m.insert('r', "r.xml");
        m.insert('s', "s.xml");
        m.insert('t', "t.xml");
        m.insert('u', "u.xml");
        m.insert('v', "v.xml");
        m.insert('w', "w.xml");
        m.insert('x', "x.xml");
        m.insert('y', "y.xml");
        m.insert('z', "z.xml");
        m
    };
}

pub fn search(term: &str, file: &str) -> Result<String, Box<dyn Error>> {
    let path = Path::new("/home/simon/Projects/python/latindictionary/data");

    let mut found = String::new();
    let full_path = path.join(file);
    // println!("path: {:?}", full_path);
    let file_text = File::open(full_path)?;
    let buffered_text = BufReader::new(file_text);
    let ptn = format!(r#"key="{}""#, regex::escape(term));
    let rgx = Regex::new(&ptn)?;
    for line in buffered_text.lines() {
        let line = line?;
        if rgx.is_match(&line) {
            found = line;
            break;
        }
    }
    Ok(found)
}

pub fn query_lns_vec(term_vec: Vec<String>) -> Result<String, Box<dyn std::error::Error>> {
    let mut lines_found = Vec::new();
    let mut entries_found = Vec::new();
    for term in term_vec {
        let initial: char = term.chars().next().unwrap().to_lowercase().next().unwrap();
        let file = XML_FILES.get(&initial).unwrap();
        let query_result = search(&term, &file);
        match query_result {
            Ok(line) => lines_found.push(line),
            Err(e) => eprintln!("{e}"),
        }
    }

    for line in lines_found.iter() {
        let parsed_entry = parse_entry(line);
        match parsed_entry {
            Ok((_, entry)) => entries_found.push(entry),
            Err(e) => eprintln!("{e}"),
        }
    }

    match serde_json::to_string(&entries_found) {
        Ok(json) => Ok(json),
        Err(e) => Err(Box::new(e)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search() {
        let res = search("do^mi^nus", "d.xml");
        if let Ok(lines) = res {
            println!("{:#?}", lines);
        }
    }
}
