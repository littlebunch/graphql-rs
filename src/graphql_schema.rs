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
            let mut nutform: Vec<NutrientdataForm> = f.get_nutrient_data(&conn).expect("error loading nutrient data");
            let mut ndv: Vec<Nutrientdataview> = Vec::new();
            for j in &nutform {
            let nf=&j;
              let nv = Nutrientdataview::create(&nf);
              ndv.push(nv);
            }
          
            let mut fdv = Foodview::create(&f, &conn);
            fdv.nutrient_data=ndv;
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
    pub publication_date: String,
    #[graphql(description = "Date of last change")]
    pub modified_date: String,
    #[graphql(description = "Date this food was first available")]
    pub available_date: String,
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
    #[graphql(description = "nutrient data for a food")]
    pub nutrient_data: Vec<Nutrientdataview>,
}
impl Foodview {
    /// creates a new food view from a food
    pub fn create(f: &Food, conn: &MysqlConnection) -> Self {
        Self {
            publication_date: f.publication_date.format("%Y-%m-%d").to_string(),
            modified_date: f.modified_date.format("%Y-%m-%d").to_string(),
            available_date: f.available_date.format("%Y-%m-%d").to_string(),
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
            nutrient_data: Vec::new(),
        }
    }
}

#[derive(juniper::GraphQLObject,Debug)]
#[graphql(description = "A nutrient value for a given food and nutrient")]
pub struct Nutrientdataview {
    pub value: f64,
    pub derivation: String,
    pub derivation_code: String,
    pub nutrient_no: String,
    pub nutrient: String,
}

impl Nutrientdataview {
  pub fn create(n: &NutrientdataForm) -> Self {
    Self {
      value: n.value,
      nutrient_no: n.nutrient_no.to_string(),
      nutrient: n.nutrient.to_string(),
      derivation: n.derivation.to_string(),
      derivation_code: n.derivation_code.to_string(),
    }
  }
}

#[derive(juniper::GraphQLObject,Debug)]
#[graphql(description = "How a nutrient value is dervied for a food")]
pub struct Derivationview {
    code: String,
    description: String,
}
#[derive(juniper::GraphQLObject,Debug)]
#[graphql(description = "How a nutrient value is dervied for a food")]
pub struct Nutrientview {
    no: String,
    name: String,
    unit: String,
}

#[derive(juniper::GraphQLInputObject,Debug)]
pub struct BrowseRequest {
    pub max: i32,
    pub offset: i32,
}
