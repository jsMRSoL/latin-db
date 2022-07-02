#[macro_use]
extern crate diesel;

use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;
use diesel::prelude::*;
use dotenv;
// use diesel::result::Error;
// use crate::search::XML_FILES;
use serde::Serialize;
use serde_json;

pub mod models;
pub mod parsing;
pub mod schema;
pub mod search;

// use search::query_lns;
use search::query_lns_vec;

pub type QueryFunc = fn(&str, &PgConnection) -> Result<String, Box<dyn std::error::Error>>;

pub fn get_connection_pool() -> Pool<ConnectionManager<PgConnection>> {
    dotenv::from_path("/home/simon/.api_keys").expect("api_keys not accessible");
    // let url = env::var("LATIN_PG_DATABASE_URL").expect("LATIN_PG_DATABASE_URL must be set");
    let url = dotenv::var("LATIN_PG_DATABASE_URL").expect("LATIN_PG_DATABASE_URL must be set");
    
    let manager = ConnectionManager::<PgConnection>::new(url);
    Pool::builder()
        .max_size(25)
        .test_on_check_out(true)
        .build(manager)
        .expect("Could not build connection pool")
}

#[derive(Queryable, Debug, PartialEq, Clone, Serialize)]
struct QueryResult {
    dict_form: String,
    part_of_speech: String,
    meaning: String,
}

pub fn query_gcse_latin(
    term: &str,
    connection: &PgConnection,
) -> Result<String, Box<dyn std::error::Error>> {
    use self::schema::gcse_latin::dsl::dict_form as g_dict_form;
    use self::schema::gcse_latin::dsl::gcse_latin;
    use self::schema::gcse_latin::dsl::headword as g_headword;
    use self::schema::gcse_latin::dsl::meaning as g_meaning;
    use self::schema::gcse_latin::dsl::part_of_speech as g_part_of_speech;
    use self::schema::lewis_short_lemmata::dsl::*;

    let data: Result<Vec<(String, String, String)>, _> = gcse_latin
        .inner_join(lewis_short_lemmata.on(g_headword.eq(headword)))
        .filter(form.eq(term))
        .select((g_dict_form, g_part_of_speech, g_meaning))
        .order(g_dict_form.asc())
        .load(connection);

    match data {
        Ok(results) => match serde_json::to_string(&results) {
            Ok(json) => Ok(format!("\"gcse\": {}", json)),
            Err(e) => Err(Box::new(e)),
        },
        Err(e) => Err(Box::new(e)),
    }
}

pub fn query_clc4(
    term: &str,
    connection: &PgConnection,
) -> Result<String, Box<dyn std::error::Error>> {
    use self::schema::clc4::dsl::clc4;
    use self::schema::clc4::dsl::dict_form as c_dict_form;
    use self::schema::clc4::dsl::headword as c_headword;
    use self::schema::clc4::dsl::meaning as c_meaning;
    use self::schema::lewis_short_lemmata::dsl::*;

    let data: Result<Vec<(String, String)>, _> = clc4
        .inner_join(lewis_short_lemmata.on(c_headword.eq(headword)))
        .filter(form.eq(term))
        .select((c_dict_form, c_meaning))
        .order(c_dict_form.asc())
        .load(connection);

    match data {
        Ok(results) => match serde_json::to_string(&results) {
            Ok(json) => Ok(format!("\"clc\": {}", json)),
            Err(e) => Err(Box::new(e)),
        },
        Err(e) => Err(Box::new(e)),
    }
}

pub fn query_asvocab(
    term: &str,
    connection: &PgConnection,
) -> Result<String, Box<dyn std::error::Error>> {
    use self::schema::asvocab::dsl::asvocab;
    use self::schema::asvocab::dsl::dict_form as a_dict_form;
    use self::schema::asvocab::dsl::headword as a_headword;
    use self::schema::asvocab::dsl::meaning as a_meaning;
    use self::schema::asvocab::dsl::part_of_speech as a_part_of_speech;
    use self::schema::lewis_short_lemmata::dsl::*;

    let data: Result<Vec<(String, String, String)>, _> = asvocab
        .inner_join(lewis_short_lemmata.on(a_headword.eq(headword)))
        .filter(form.eq(term))
        .select((a_dict_form, a_part_of_speech, a_meaning))
        .order(a_dict_form.asc())
        .load(connection);

    match data {
        Ok(results) => match serde_json::to_string(&results) {
            Ok(json) => Ok(format!("\"asvocab\": {}", json)),
            Err(e) => Err(Box::new(e)),
        },
        Err(e) => Err(Box::new(e)),
    }
}

pub fn query_wwords(
    term: &str,
    connection: &PgConnection,
) -> Result<String, Box<dyn std::error::Error>> {
    use self::schema::wwords::dsl::wwords;
    use self::schema::wwords::dsl::dict_form as w_dict_form;
    use self::schema::wwords::dsl::headword as w_headword;
    use self::schema::wwords::dsl::meaning as w_meaning;
    use self::schema::wwords::dsl::part_of_speech as w_part_of_speech;
    use self::schema::wwords::dsl::class as w_class;
    use self::schema::lewis_short_lemmata::dsl::*;

    let data: Result<Vec<(String, String, Option<String>, String)>, _> = wwords
        .inner_join(lewis_short_lemmata.on(w_headword.eq(headword)))
        .filter(form.eq(term))
        .select((w_dict_form, w_part_of_speech, w_class, w_meaning))
        .order(w_dict_form.asc())
        .load(connection);

    match data {
        Ok(results) => match serde_json::to_string(&results) {
            Ok(json) => Ok(format!("\"wwords\": {}", json)),
            Err(e) => Err(Box::new(e)),
        },
        Err(e) => Err(Box::new(e)),
    }
}

pub fn get_lns_key(
    term: &str,
    connection: &PgConnection,
) -> Result<String, Box<dyn std::error::Error>> {
    use self::schema::lewis_short_lemmata::dsl::*;
    use self::schema::lns_entry_keys::dsl::*;

    let data: Result<Vec<String>, _> = lns_entry_keys
        .inner_join(lewis_short_lemmata.on(headword.eq(head)))
        .filter(form.eq(term))
        .select(key)
        .load(connection);

    match data {
        Ok(results) => {
            match query_lns_vec(results) {
                Ok(parsed_entries) => Ok(format!("\"lns\": {}", parsed_entries)),
                Err(e) => Err(e),
            }
        }
        Err(e) => Err(Box::new(e)),
    }
}

pub fn query_gcse_latin_headword(
    term: &str,
    connection: &PgConnection,
) -> Result<String, Box<dyn std::error::Error>> {
    use self::schema::gcse_latin::dsl::dict_form as g_dict_form;
    use self::schema::gcse_latin::dsl::gcse_latin;
    use self::schema::gcse_latin::dsl::headword as g_headword;
    use self::schema::gcse_latin::dsl::meaning as g_meaning;
    use self::schema::gcse_latin::dsl::part_of_speech as g_part_of_speech;

    let data: Result<Vec<(String, String, String)>, _> = gcse_latin
        .filter(g_headword.eq(term))
        .select((g_dict_form, g_part_of_speech, g_meaning))
        .order(g_dict_form.asc())
        .load(connection);

    match data {
        Ok(results) => match serde_json::to_string(&results) {
            Ok(json) => Ok(format!("\"gcse\": {}", json)),
            Err(e) => Err(Box::new(e)),
        },
        Err(e) => Err(Box::new(e)),
    }
}

pub fn query_clc4_headword(
    term: &str,
    connection: &PgConnection,
) -> Result<String, Box<dyn std::error::Error>> {
    use self::schema::clc4::dsl::clc4;
    use self::schema::clc4::dsl::dict_form as c_dict_form;
    use self::schema::clc4::dsl::headword as c_headword;
    use self::schema::clc4::dsl::meaning as c_meaning;

    let data: Result<Vec<(String, String)>, _> = clc4
        .filter(c_headword.eq(term))
        .select((c_dict_form, c_meaning))
        .order(c_dict_form.asc())
        .load(connection);

    match data {
        Ok(results) => match serde_json::to_string(&results) {
            Ok(json) => Ok(format!("\"clc\": {}", json)),
            Err(e) => Err(Box::new(e)),
        },
        Err(e) => Err(Box::new(e)),
    }
}

pub fn query_asvocab_headword(
    term: &str,
    connection: &PgConnection,
) -> Result<String, Box<dyn std::error::Error>> {
    use self::schema::asvocab::dsl::asvocab;
    use self::schema::asvocab::dsl::dict_form as a_dict_form;
    use self::schema::asvocab::dsl::headword as a_headword;
    use self::schema::asvocab::dsl::meaning as a_meaning;
    use self::schema::asvocab::dsl::part_of_speech as a_part_of_speech;

    let data: Result<Vec<(String, String, String)>, _> = asvocab
        .filter(a_headword.eq(term))
        .select((a_dict_form, a_part_of_speech, a_meaning))
        .order(a_dict_form.asc())
        .load(connection);

    match data {
        Ok(results) => match serde_json::to_string(&results) {
            Ok(json) => Ok(format!("\"asvocab\": {}", json)),
            Err(e) => Err(Box::new(e)),
        },
        Err(e) => Err(Box::new(e)),
    }
}

pub fn query_wwords_headword(
    term: &str,
    connection: &PgConnection,
) -> Result<String, Box<dyn std::error::Error>> {
    use self::schema::wwords::dsl::wwords;
    use self::schema::wwords::dsl::dict_form as w_dict_form;
    use self::schema::wwords::dsl::headword as w_headword;
    use self::schema::wwords::dsl::meaning as w_meaning;
    use self::schema::wwords::dsl::part_of_speech as w_part_of_speech;
    use self::schema::wwords::dsl::class as w_class;

    let data: Result<Vec<(String, String, Option<String>, String)>, _> = wwords
        .filter(w_headword.eq(term))
        .select((w_dict_form, w_part_of_speech, w_class, w_meaning))
        .order(w_dict_form.asc())
        .load(connection);

    match data {
        Ok(results) => match serde_json::to_string(&results) {
            Ok(json) => Ok(format!("\"wwords\": {}", json)),
            Err(e) => Err(Box::new(e)),
        },
        Err(e) => Err(Box::new(e)),
    }
}
pub fn get_lns_key_headword(
    term: &str,
    connection: &PgConnection,
) -> Result<String, Box<dyn std::error::Error>> {
    use self::schema::lns_entry_keys::dsl::*;

    let data: Result<Vec<String>, _> = lns_entry_keys
        .filter(head.eq(term))
        .select(key)
        .load(connection);

    match data {
        Ok(results) => {
            match query_lns_vec(results) {
                Ok(parsed_entries) => Ok(format!("\"lns\": {}", parsed_entries)),
                Err(e) => Err(e),
            }
        }
        Err(e) => Err(Box::new(e)),
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
