table! {
    asvocab (id) {
        id -> Int8,
        headword -> Varchar,
        dict_form -> Varchar,
        part_of_speech -> Varchar,
        meaning -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    asvocab2 (id) {
        id -> Int8,
        headword -> Nullable<Varchar>,
        dict_form -> Nullable<Varchar>,
        part_of_speech -> Nullable<Varchar>,
        meaning -> Nullable<Varchar>,
        asvocab -> Nullable<Bool>,
        gcsevocab -> Nullable<Bool>,
    }
}

table! {
    clc4 (id) {
        id -> Int8,
        headword -> Varchar,
        dict_form -> Varchar,
        meaning -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    gcse_latin (id) {
        id -> Int8,
        headword -> Varchar,
        dict_form -> Varchar,
        part_of_speech -> Varchar,
        meaning -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    lewis_short_lemmata (id) {
        id -> Int8,
        lemmata_no -> Int8,
        headword -> Varchar,
        suffixed -> Varchar,
        form -> Varchar,
        analysis -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    lns_entry_keys (id) {
        id -> Int8,
        xml_id -> Varchar,
        key -> Varchar,
        simple_key -> Varchar,
        head -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    wwords (id) {
        id -> Int8,
        headword -> Varchar,
        dict_form -> Varchar,
        part_of_speech -> Varchar,
        class -> Nullable<Varchar>,
        meaning -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

allow_tables_to_appear_in_same_query!(
    asvocab,
    asvocab2,
    clc4,
    gcse_latin,
    lewis_short_lemmata,
    lns_entry_keys,
    wwords,
);
