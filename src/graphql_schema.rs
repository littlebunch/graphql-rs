extern crate dotenv;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;
use crate::db::MysqlPool;
use crate::schema::{derivations, food_groups, foods, manufacturers, nutrient_data, nutrients};
use chrono::{NaiveDate, NaiveDateTime};
use diesel::mysql::MysqlConnection;
use graphql_rs::models::*;
use juniper::RootNode;

const MAX_RECS: i32 = 150;

#[derive(Clone)]
pub struct Context {
    pub db: MysqlPool,
}

impl juniper::Context for Context {}

pub struct QueryRoot;

#[juniper::object(Context = Context)]
impl QueryRoot {
    fn foods(context: &Context, mut max: i32, mut offset: i32, mut sort: String) -> Vec<Foodview> {
        use crate::schema::foods::dsl::*;
        let conn = context.db.get().unwrap();
        if max > MAX_RECS {
            max = MAX_RECS
        }
        if offset < 0 {
            offset = 0
        }
        let food = Food::new();
        let data = food
            .browse(max as i64, offset as i64, sort, String::from("asc"), &conn)
            .expect("error loading foods");

        let mut fv: Vec<Foodview> = Vec::new();
        for i in &data {
            let f = &i;
            let mut fdv = Foodview::create(&f, &conn);
            fv.push(fdv);
        }
        fv
    }
    fn food(context: &Context, fid: String) -> Vec<Foodview> {
        use crate::schema::foods::dsl::*;
        let conn = context.db.get().unwrap();
        let mut food = Food::new();

        if fid.len() >= 10 {
            food.upc = fid;
        } else {
            food.fdc_id = fid;
        }

        let data = food.get(&conn).expect("error loading food");
        let mut fv: Vec<Foodview> = Vec::new();
        for i in &data {
            let f = &i;
            let mut fdv = Foodview::create(&f, &conn);
            fv.push(fdv);
        }
        fv
    }
}
pub struct MutationRoot;

#[juniper::object(Context = Context)]
impl MutationRoot {
    fn create_food_not_implemented(context: &Context) -> String {
        String::from("not implemented")
    }
}
pub type Schema = RootNode<'static, QueryRoot, MutationRoot>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, MutationRoot {})
}
#[derive(juniper::GraphQLObject, Debug)]
#[graphql(description = "Defines a branded food product")]
pub struct Foodview {
    #[graphql(description = "Date this food was last published")]
    pub publication_date: NaiveDateTime,
    #[graphql(description = "Date of last change")]
    pub modified_date: NaiveDateTime,
    #[graphql(description = "Date this food was first available")]
    pub available_date: NaiveDateTime,
    #[graphql(description = "UPC/GTIN code for a food")]
    pub upc: String,
    #[graphql(description = "Food Data Central Id")]
    pub fdc_id: String,
    #[graphql(description = "Food name")]
    pub description: String,
    #[graphql(description = "Food Group to which a food belongs")]
    pub food_group: String,
    #[graphql(description = "Manufacturer to which a food belongs")]
    pub manufacturer: String,
    #[graphql(description = "Provider of food data -- GDSN or LI")]
    pub datasource: String,
    #[graphql(description = "Food portion size in specified unit")]
    pub serving_size: Option<f64>,
    #[graphql(description = "Unit of measure for food portion size")]
    pub serving_unit: Option<String>,
    #[graphql(description = "Food portion description")]
    pub serving_description: Option<String>,
    #[graphql(description = "Country of origin")]
    pub country: Option<String>,
    #[graphql(description = "Food ingredients")]
    pub ingredients: Option<String>,
}
impl Foodview {
    pub fn publication_date(&self) -> NaiveDateTime {
        self.publication_date
    }
    pub fn modified_date(&self) -> NaiveDateTime {
        self.modified_date
    }
    pub fn available_date(&self) -> NaiveDateTime {
        self.available_date
    }
    pub fn upc(&self) -> &str {
        &self.upc
    }
    pub fn fdc_id(&self) -> &str {
        &self.fdc_id
    }
    pub fn description(&self) -> &str {
        &self.description
    }
    pub fn food_group(&self) -> &str {
        &self.food_group
    }
    pub fn manufacturer(&self) -> &str {
        &self.manufacturer
    }
    pub fn datasource(&self) -> &str {
        self.datasource.as_str()
    }
    pub fn serving_size(&self) -> Option<f64> {
        self.serving_size
    }
    pub fn serving_unit(&self) -> Option<String> {
        Some(
            self.serving_unit
                .as_ref()
                .map(|n| n.to_string())
                .unwrap_or("unknown".to_string()),
        )
    }
    pub fn serving_description(&self) -> Option<String> {
        Some(
            self.serving_description
                .as_ref()
                .map(|n| n.to_string())
                .unwrap_or("unknown".to_string()),
        )
    }
    pub fn country(&self) -> Option<String> {
        Some(
            self.country
                .as_ref()
                .map(|n| n.to_string())
                .unwrap_or("unknown".to_string()),
        )
    }
    pub fn ingredients(&self) -> Option<String> {
        Some(
            self.ingredients
                .as_ref()
                .map(|n| n.to_string())
                .unwrap_or("unknown".to_string()),
        )
    }
    pub fn new() -> Self {
        Self {
            publication_date: NaiveDate::from_ymd(1970, 01, 01).and_hms(00, 00, 00),
            modified_date: NaiveDate::from_ymd(1970, 01, 01).and_hms(00, 00, 00),
            available_date: NaiveDate::from_ymd(1970, 01, 01).and_hms(00, 00, 00),
            upc: String::from("unknown"),
            fdc_id: String::from("unknown"),
            description: String::from("unknown"),
            food_group: String::from("unknown"),
            manufacturer: String::from("unknown"),
            datasource: String::from("unknown"),
            serving_size: None,
            serving_unit: None,
            serving_description: None,
            country: None,
            ingredients: None,
        }
    }
    /// creates a new food view from a food
    pub fn create(f: &Food, conn: &MysqlConnection) -> Self {
        Self {
            publication_date: f.publication_date,
            modified_date: f.modified_date,
            available_date: f.available_date,
            upc: f.upc.to_string(),
            fdc_id: f.fdc_id.to_string(),
            description: f.description.to_string(),
            food_group: f.get_food_group_name(&conn).unwrap(),
            manufacturer: f.get_manufacturer_name(&conn).unwrap(),
            datasource: f.datasource.to_string(),
            serving_description: Some(
                f.serving_description
                    .as_ref()
                    .map(|n| n.to_string())
                    .unwrap_or("unknown".to_string()),
            ),
            serving_size: f.serving_size,
            serving_unit: Some(
                f.serving_unit
                    .as_ref()
                    .map(|n| n.to_string())
                    .unwrap_or("unknown".to_string()),
            ),
            country: Some(
                f.country
                    .as_ref()
                    .map(|n| n.to_string())
                    .unwrap_or("unknown".to_string()),
            ),
            ingredients: Some(
                f.ingredients
                    .as_ref()
                    .map(|n| n.to_string())
                    .unwrap_or("unknown".to_string()),
            ),
        }
    }
}

#[derive(juniper::GraphQLObject)]
#[graphql(description = "A nutrient value for a given food and nutrient")]
pub struct Nutrientdata_view {
    pub id: i32,
    pub value: f64,
    // pub derivation: Derivation,
    // pub nutrient: Nutrient,
    pub food_id: i32,
}

#[derive(juniper::GraphQLObject)]
#[graphql(description = "How a nutrient value is dervied for a food")]
pub struct Derivation {
    id: i32,
    code: String,
    description: String,
}
#[derive(juniper::GraphQLInputObject)]
pub struct BrowseRequest {
    pub max: i32,
    pub offset: i32,
}
