extern crate dotenv;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;
use crate::db::MysqlPool;
use diesel::mysql::MysqlConnection;
use graphql_rs::models::*;
use graphql_rs::Browse;
use graphql_rs::Get;
use juniper::{graphql_value, IntoFieldError,FieldError, FieldResult, RootNode};
const MAX_RECS: i32 = 150;

#[derive(Clone)]
pub struct Context {
    pub db: MysqlPool,
}

impl juniper::Context for Context {}

pub struct QueryRoot;
enum CustomError {
    MaxValidationError,
    InvalidOffsetError,
}

impl juniper::IntoFieldError for CustomError {
    fn into_field_error(self) -> FieldError {
        match self {
            CustomError::MaxValidationError => FieldError::new(
                format!("max parameter exceeds allowed amounts.  Enter 1 to {}",MAX_RECS),
                graphql_value!({
                    "type": "MAX_ERROR"
                }),
            ),
            CustomError::InvalidOffsetError => FieldError::new(
                "offset parameter must be greater than 1",
                graphql_value!({
                    "type": "OFFSET_ERROR"
                }),
            ),
        }
    }
}
#[juniper::object(Context = Context)]
impl QueryRoot {
    fn foods(
        context: &Context,
        mut max: i32,
        mut offset: i32,
        mut sort: String,
        mut order: String,
        nids: Vec<String>,
    ) -> FieldResult<Vec<Foodview>> {
        let conn = context.db.get().unwrap();
        if max > MAX_RECS || max < 1 {
            return Err(CustomError::MaxValidationError.into_field_error())
        }
        if offset < 0 {
            return  Err(CustomError::InvalidOffsetError.into_field_error())
        }
        let food = Food::new();
        let data = food.browse(max as i64, offset as i64, sort, order, &conn)?;
        Ok(Foodview::build_view(data, &nids, &conn))
    }
    fn food(context: &Context, fid: String, nids: Vec<String>) -> FieldResult<Vec<Foodview>> {
        let conn = context.db.get().unwrap();
        let mut food = Food::new();

        if fid.len() >= 10 {
            food.upc = fid;
        } else {
            food.fdc_id = fid;
        }

        let data = food.get(&conn)?;
        Ok(Foodview::build_view(data, &nids, &conn))
    }
    fn nutrient(context: &Context, nno: String) -> FieldResult<Vec<Nutrientview>> {
        let conn = context.db.get().unwrap();
        let mut n = Nutrient::new();
        n.nutrientno = nno;
        let nut = n.get(&conn)?;
        let mut nv: Vec<Nutrientview> = Vec::new();
        for i in &nut {
            let nv1 = &i;
            nv.push(Nutrientview::create(nv1));
        }
        Ok(nv)
    }
    fn nutrients(
        context: &Context,
        mut max: i32,
        mut offset: i32,
        mut sort: String,
        mut order: String,
        nids: Vec<String>,
    ) -> FieldResult<Vec<Nutrientview>> {
        let conn = context.db.get()?;
        let mut b = false;
        if max > MAX_RECS || max < 1 {
           return Err(CustomError::MaxValidationError.into_field_error())
        }
        if offset < 0 {
            return  Err(CustomError::InvalidOffsetError.into_field_error())
        }
        let n = Nutrient::new();

        let data = n.browse(max as i64, offset as i64, sort, order, &conn)?;
        let mut nv: Vec<Nutrientview> = Vec::new();
        for i in &data {
            let nv1 = &i;
            nv.push(Nutrientview::create(nv1));
        }

        Ok(nv)
    }

    fn food_groups(
        context: &Context,
        mut max: i32,
        mut offset: i32,
        mut sort: String,
        order: String,
    ) -> FieldResult<Vec<FoodgroupView>> {
        let conn = context.db.get().unwrap();
        if max > MAX_RECS || max < 1 {
            return Err(CustomError::MaxValidationError.into_field_error())
        }
        if offset < 0 {
            return  Err(CustomError::InvalidOffsetError.into_field_error())
        }
        let fg = Foodgroup::new();
        let data = fg.browse(max as i64, offset as i64, sort, order, &conn)?;
        let mut fgv: Vec<FoodgroupView> = Vec::new();
        for i in &data {
            let fgv1 = &i;
            fgv.push(FoodgroupView::create(fgv1));
        }
        Ok(fgv)
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
    #[graphql(description = "Date food was updated")]
    pub publication_date: String,
    #[graphql(
        description = "This date reflects when the product data was last modified by the data provider, i.e., the manufacturer"
    )]
    pub modified_date: String,
    #[graphql(
        description = "This is the date when the product record was available for inclusion in the database."
    )]
    pub available_date: String,
    #[graphql(
        description = "GTIN or UPC code identifying the food. Duplicate codes signify an update to the product, use the publication_date found in the food table to distinguish when each update was published, e.g. the latest publication date will be the most recent update of the product."
    )]
    pub upc: String,
    #[graphql(description = "Food Data Central Id")]
    pub fdc_id: String,
    #[graphql(description = "Food name")]
    pub description: String,
    #[graphql(description = "The category of the branded food, assigned by GDSN or Label Insight")]
    pub food_group: String,
    #[graphql(description = "Brand owner for the food")]
    pub manufacturer: String,
    #[graphql(description = "Provider of food data -- GDSN or LI")]
    pub datasource: String,
    #[graphql(
        description = "The amount of the serving size expressed as 100 gram or ml equivalent"
    )]
    pub serving_size: Option<f64>,
    #[graphql(description = "The unit used to express the serving size (gram or ml)")]
    pub serving_unit: Option<String>,
    #[graphql(description = "amount and unit of serving size when expressed in household units")]
    pub serving_description: Option<String>,
    #[graphql(description = "The primary country where the product is marketed.")]
    pub country: Option<String>,
    #[graphql(description = "The list of ingredients (as it appears on the product label)")]
    pub ingredients: Option<String>,
    #[graphql(description = "nutrient data for a food")]
    pub nutrient_data: Vec<Nutrientdataview>,
}
impl Foodview {
    pub fn build_view(fd: Vec<Food>, nids: &Vec<String>, conn: &MysqlConnection) -> Vec<Foodview> {
        let mut fv: Vec<Foodview> = Vec::new();
        for i in &fd {
            let f = &i;
            let mut fdv = Foodview::create(&f, &conn);
            let nutform: Vec<NutrientdataForm> = f
                .get_nutrient_data(nids, &conn)
                .expect("error loading nutrient data");

            let mut ndv: Vec<Nutrientdataview> = Vec::new();
            for j in &nutform {
                let nf = &j;
                let mut nv = Nutrientdataview::create(&nf);
                nv.portion_value = match fdv.serving_size {
                    Some(x) => (x as f64 / 100.0) * nv.value,
                    None => 0.0,
                };
                ndv.push(nv);
            }
            fdv.nutrient_data = ndv;
            fv.push(fdv);
        }
        fv
    }
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

#[derive(juniper::GraphQLObject, Debug)]
#[graphql(description = "A nutrient value for a given food and nutrient")]
pub struct Nutrientdataview {
    #[graphql(
        description = "Amount of the nutrient per 100g of food. Specified in unit defined in the nutrient table."
    )]
    pub value: f64,
    #[graphql(
        description = "Amount of the nutrient per portion size of food. Specified in unit defined in the nutrient table."
    )]
    pub portion_value: f64,
    #[graphql(description = "Description of the derivation")]
    pub derivation: String,
    #[graphql(description = "Code used for the derivation (e.g. A means analytical)")]
    pub derivation_code: String,
    #[graphql(description = "A unique code identifying a nutrient or food constituent")]
    pub nutrient_no: String,
    #[graphql(description = "Name of the nutrient")]
    pub nutrient: String,
    #[graphql(description = "The standard unit of measure for the nutrient (per 100g of food)")]
    pub unit: String,
}

impl Nutrientdataview {
    pub fn create(n: &NutrientdataForm) -> Self {
        Self {
            value: n.value,
            portion_value: 0.0,
            nutrient_no: n.nutrient_no.to_string(),
            nutrient: n.nutrient.to_string(),
            unit: n.unit.to_string(),
            derivation: n.derivation.to_string(),
            derivation_code: n.derivation_code.to_string(),
        }
    }
}

#[derive(juniper::GraphQLObject, Debug)]
#[graphql(description = "How a nutrient value is dervied for a food")]
pub struct Derivationview {
    code: String,
    description: String,
}
#[derive(juniper::GraphQLObject, Debug)]
#[graphql(description = "The category assigned to a food")]
pub struct FoodgroupView {
    #[graphql(description = "A unique code identifying a food group")]
    pub id: i32,
    #[graphql(description = "Food group name")]
    pub group: String,
}
impl FoodgroupView {
    fn create(fg: &Foodgroup) -> Self {
        Self {
            id: fg.id,
            group: fg.description.to_string(),
        }
    }
}

#[derive(juniper::GraphQLObject, Debug)]
#[graphql(
    description = "The chemical constituent of a food (e.g. calcium, vitamin E) officially recognized as essential to human health"
)]
pub struct Nutrientview {
    #[graphql(description = "A unique code identifying a nutrient or food constituent")]
    nbr: String,
    #[graphql(description = "Name of the nutrient")]
    name: String,
    #[graphql(description = "The standard unit of measure for the nutrient (per 100g of food)")]
    unit: String,
}
impl Nutrientview {
    pub fn create(n: &Nutrient) -> Self {
        Self {
            nbr: n.nutrientno.to_string(),
            name: n.description.to_string(),
            unit: n.unit.to_string(),
        }
    }
}

#[derive(juniper::GraphQLInputObject, Debug)]
#[graphql(
    name = "BrowseRequest",
    description = "Input object for defining a foods browse query"
)]
pub struct Browsequery {
    pub max: i32,
    pub offset: i32,
    #[graphql(description = "Sort order, one of: database id (default),description, upc or fdcId")]
    pub sort: String,
    pub order: String,
}
