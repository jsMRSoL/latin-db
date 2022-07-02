use nom::bytes::complete::{tag, take_until};
use nom::{combinator::opt, multi::many0, sequence::delimited, IResult};
use serde::Serialize;

#[derive(Debug, Serialize, PartialEq, Clone)]
struct Sense<'a> {
    pos: Option<&'a str>,
    authors: Vec<&'a str>,
    i_tags: Vec<&'a str>,
}

#[derive(Debug, Serialize, PartialEq)]
pub struct Entry<'a> {
    head: &'a str,
    orth_orig: &'a str,
    early_i_tags: Vec<&'a str>,
    senses: Vec<Sense<'a>>,
}

impl<'a> Entry<'a> {
    pub fn print(&self) {
        println!("\n{}", self.head);
        println!("Pronunciation: {}", self.orth_orig);
        if self.early_i_tags.len() > 0 {
            println!("{}", self.early_i_tags.join(", "));
        }
        let mut count = 0;
        for sense in self.senses.iter() {
            if sense.i_tags.len() > 0 {
                count += 1;
                println!("# Sense {}", count);
                if let Some(pos) = sense.pos {
                    println!("Part of speech: {}", pos);
                }
                println!("{}", sense.i_tags.join(", "));
            }
            if sense.authors.len() > 0 {
                println!("Author(s): {}", sense.authors.join(", "))
            }
        }
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

pub fn parse_entry<'b>(input: &'b str) -> IResult<&'b str, Entry> {
    let (tail, _) = take_until("orth_orig")(input)?;
    let (tail, orth_orig) = delimited(tag("orth_orig=\""), take_until("\""), tag("\""))(tail)?;
    let (tail, head) = delimited(tag(">"), take_until("</head>"), tag("</head>"))(tail)?;
    let (tail, pre) = take_until("<sense")(tail)?;
    let (_, early_i_tags) = many0(chomp_and_get_i_tag)(pre)?;
    let (tail, senses) = many0(chomp_and_return_sense)(tail)?;
    Ok((
        tail,
        Entry {
            head,
            orth_orig,
            early_i_tags,
            senses,
        },
    ))
}

fn chomp_and_return_sense<'a>(input: &'a str) -> IResult<&'a str, Sense> {
    let (tail, _) = take_until("<sense")(input)?;
    let (tail, _) = take_until(">")(tail)?;
    let (tail, sense_block) = delimited(tag(">"), take_until("</sense>"), tag("</sense>"))(tail)?;
    let (_, i_tags) = many0(chomp_and_get_i_tag)(sense_block)?;
    let (_, mut authors) = many0(chomp_and_get_author_tag)(sense_block)?;
    authors.sort();
    authors.dedup();
    let (_, pos) = opt(chomp_and_get_pos_tag)(sense_block)?;
    Ok((
        tail,
        Sense {
            pos,
            authors,
            i_tags,
        },
    ))
}

fn chomp_and_get_i_tag(input: &str) -> IResult<&str, &str> {
    let (tail, _) = take_until("<i>")(input)?;
    let (tail, inner) = delimited(tag("<i>"), take_until("</i>"), tag("</i>"))(tail)?;
    Ok((tail, inner))
}

fn chomp_and_get_author_tag(input: &str) -> IResult<&str, &str> {
    let (tail, _) = take_until("<author>")(input)?;
    let (tail, inner) =
        delimited(tag("<author>"), take_until("</author>"), tag("</author>"))(tail)?;
    Ok((tail, inner))
}

fn chomp_and_get_pos_tag(input: &str) -> IResult<&str, &str> {
    let (tail, _) = take_until("<pos")(input)?;
    let (tail, _) = take_until(">")(tail)?;
    let (tail, pos) = delimited(tag(">"), take_until("</pos>"), tag("</pos>"))(tail)?;
    Ok((tail, pos))
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_chomp() {
        assert_eq!(
            chomp_and_get_i_tag("<sense><i>lord, master, sir</i></sense>"),
            Ok(("</sense>", "lord, master, sir"))
        );
    }

    #[test]
    fn test_chomp_and_return_sense() {
        let test_str = r#"<sense n="1"><pos n="1">test_pos</pos>"<i>test</i><i>test2</i><author>test_author1</author><author>test_author2</author></sense>"#;
        assert_eq!(
            chomp_and_return_sense(test_str),
            Ok((
                "",
                Sense {
                    pos: Some("test_pos",),
                    authors: ["test_author1", "test_author2",].to_vec(),
                    i_tags: ["test", "test2",].to_vec(),
                },
            ),)
        );
    }

    #[test]
    fn test_parse_entry() {
        let test_str = r#"<div1><head extent="full" lang="la" opt="n" orth_orig="dŏmĭnus">dominus</head><i>test</i><i>test2</i><sense n="1"><pos n="1">test_pos</pos>"<i>test</i><i>test2</i><author>test_author1</author><author>test_author2</author></sense><sense n="1"><pos n="1">test_pos</pos>"<i>test</i><i>test2</i><author>test_author1</author><author>test_author2</author></sense></div>"#;
        assert_eq!(
            parse_entry(test_str),
            Ok((
                "</div>",
                Entry {
                    head: "dominus",
                    orth_orig: "dŏmĭnus",
                    early_i_tags: ["test", "test2",].to_vec(),
                    senses: [
                        Sense {
                            pos: Some("test_pos"),
                            authors: ["test_author1", "test_author2",].to_vec(),
                            i_tags: ["test", "test2",].to_vec(),
                        },
                        Sense {
                            pos: Some("test_pos"),
                            authors: ["test_author1", "test_author2",].to_vec(),
                            i_tags: ["test", "test2",].to_vec(),
                        },
                    ]
                    .to_vec(),
                },
            ))
        );
    }

    #[test]
    fn test_serialize() {
        let test_str = r#"<div1><head extent="full" lang="la" opt="n" orth_orig="dŏmĭnus">dominus</head><i>test</i><i>test2</i><sense n="1"><pos n="1">test_pos</pos>"<i>test</i><i>test2</i><author>test_author1</author><author>test_author2</author></sense><sense n="1"><pos n="1">test_pos</pos>"<i>test</i><i>test2</i><author>test_author1</author><author>test_author2</author></sense></div>"#;
        let res = parse_entry(test_str);
        let (_, entry) = res.unwrap();
        let serialized_entry = entry.to_json();
        assert_eq!(
            serialized_entry,
            r#"{"head":"dominus","orth_orig":"dŏmĭnus","early_i_tags":["test","test2"],"senses":[{"pos":"test_pos","authors":["test_author1","test_author2"],"i_tags":["test","test2"]},{"pos":"test_pos","authors":["test_author1","test_author2"],"i_tags":["test","test2"]}]}"#
        );
    }
}
