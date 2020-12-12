table! {
    derivations (id) {
    id -> Int4,
    code -> Varchar,
    description -> Text,
 }
}
table! {
    food_groups (id) {
        id -> Int4,
        description -> Varchar,
    }
}
//use diesel_full_text_search::TsVector;
table! {
    foods (id) {
        id -> Int4,
        publication_date -> Timestamptz,
        modified_date -> Timestamptz,
        available_date -> Timestamptz,
        upc -> Varchar,
        fdc_id -> Varchar,
        description -> Varchar,
        food_group_id -> Int4,
        manufacturer_id -> Int4,
        datasource -> Varchar,
        serving_size -> Nullable<Float8>,
        serving_unit -> Nullable<Varchar>,
        serving_description -> Nullable<Varchar>,
        country -> Nullable<Varchar>,
        ingredients -> Nullable<Text>,
       kw_tsvector -> diesel_full_text_search::TsVector,
    }
}

table! {
    manufacturers (id) {
        id -> Int4,
        name -> Varchar,
    }
}

table! {
    nutrient_data (id) {
        id -> Int4,
        value -> Float8,
        standard_error -> Nullable<Float8>,
        minimum -> Nullable<Float8>,
        maximum -> Nullable<Float8>,
        median -> Nullable<Float8>,
        derivation_id -> Int4,
        nutrient_id -> Int4,
        food_id -> Int4,
    }
}

table! {
    nutrients (id) {
        id -> Int4,
        nutrientno -> Varchar,
        description -> Varchar,
        unit -> Varchar,
    }
}

joinable!(foods -> food_groups (food_group_id));
joinable!(foods -> manufacturers (manufacturer_id));
joinable!(nutrient_data -> derivations (derivation_id));
joinable!(nutrient_data -> foods (food_id));
joinable!(nutrient_data -> nutrients (nutrient_id));

allow_tables_to_appear_in_same_query!(
    derivations,
    food_groups,
    foods,
    manufacturers,
    nutrient_data,
    nutrients,
);
