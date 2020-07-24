extern crate diesel;

use crate::schema::{derivations, foods, manufacturers, nutrient_data, nutrients};
use chrono::{NaiveDate, NaiveDateTime};
use diesel::mysql::MysqlConnection;
use std::error::Error;
//extern crate serde;
//extern crate serde_json;

//extern crate serde_derive;
use self::diesel::prelude::*;
#[derive(Identifiable, Queryable, Associations, PartialEq, Serialize, Deserialize, Debug)]
#[belongs_to(Manufacturer)]
#[table_name = "foods"]

pub struct Food {
    pub id: i32,
    pub publication_date: NaiveDateTime,
    pub modified_date: NaiveDateTime,
    pub available_date: NaiveDateTime,
    pub upc: String,
    pub fdc_id: String,
    pub description: String,
    pub food_group_id: i32,
    pub manufacturer_id: i32,
    pub datasource: String,
    pub serving_size: Option<f64>,
    pub serving_unit: Option<String>,
    pub serving_description: Option<String>,
    pub country: Option<String>,
    pub ingredients: Option<String>,
}
impl Food {
    pub fn new() -> Self {
        Self {
            id: 0,
            publication_date: NaiveDate::from_ymd(1970, 01, 01).and_hms(00, 00, 00),
            modified_date: NaiveDate::from_ymd(1970, 01, 01).and_hms(00, 00, 00),
            available_date: NaiveDate::from_ymd(1970, 01, 01).and_hms(00, 00, 00),
            upc: String::from("unknown"),
            fdc_id: String::from("unknown"),
            description: String::from("unknown"),
            food_group_id: 0,
            manufacturer_id: 0,
            datasource: String::from("unknown"),
            serving_size: None,
            serving_unit: None,
            serving_description: None,
            country: None,
            ingredients: None,
        }
    }
    pub fn get(&self, conn: &MysqlConnection) -> Result<Vec<Food>, Box<dyn Error>> {
        use crate::schema::food_groups::dsl::*;
        use crate::schema::foods::dsl::*;
        use crate::schema::manufacturers::dsl::*;
        let mut data = vec![];
        if self.upc != "unknown" {
            data = foods.filter(upc.eq(&self.upc)).load::<Food>(conn)?;
        } else if self.id > 0 {
            data = foods.find(&self.id).load::<Food>(conn)?;
        } else {
            data = foods.filter(fdc_id.eq(&self.fdc_id)).load::<Food>(conn)?;
        }
        Ok(data)
    }
    // Returns a JSON string representation of nutrient data elements for a food id
    /* pub fn get_nutrient_data(&self, conn: &MysqlConnection) -> Result<String, Box<dyn Error>> {
        use crate::schema::derivations::dsl::*;
        use crate::schema::nutrient_data::dsl::*;
        use crate::schema::nutrients::dsl::*;
        let data = nutrient_data
            .filter(food_id.eq(&self.id))
            .inner_join(nutrients)
            .inner_join(derivations)
            .load::<(Nutrientdata, Nutrient, Derivation)>(conn)?;
        let mut ndv: Vec<NutrientdataForm> = Vec::new();
        for i in &data {
            let (nd, n, d) = &i;
            let mut ndf = NutrientdataForm::new();
            ndf.value = nd.value;
            ndf.nutrient = (*(n.description)).to_string();
            ndf.nutrient_no = (*(n.nutrientno)).to_string();
            ndf.unit = (*(n.unit)).to_string();
            ndf.derivation = (*(d.description)).to_string();
            ndf.derivation_code = (*(d.code)).to_string();
            ndv.push(ndf);
        }
        Ok(serde_json::to_string(&ndv)?)
    }*/
}
use crate::schema::food_groups::dsl::*;
use crate::schema::foods::dsl::*;
use crate::schema::manufacturers::dsl::*;
/// Returns a JSON string representing a array of Foods identified by database id, UPC, or FDCID as
/// determined by the contents of Self.  That is, if self.upc is set to anything other than the default ("unknown")
/// then the lookup is by self.upc.  Ditto for FDCID and for id.
/*impl Get for Food {
    fn get(&self, conn: &MysqlConnection) -> Result<String, Box<dyn Error>> {

        let mut data = vec![];
        if self.upc != "unknown" {
            data = foods
                .filter(upc.eq(&self.upc))
                .inner_join(manufacturers)
                .inner_join(food_groups)
                .load::<(Food, Manufacturer, Foodgroup)>(conn)?;
        } else if self.id > 0 {
            data = foods
                .find(&self.id)
                .inner_join(manufacturers)
                .inner_join(food_groups)
                .load::<(Food, Manufacturer, Foodgroup)>(conn)?;
        } else {
            data = foods
                .filter(fdc_id.eq(&self.fdc_id))
                .inner_join(manufacturers)
                .inner_join(food_groups)
                .load::<(Food, Manufacturer, Foodgroup)>(conn)?;
        }
        let mut ffv: Vec<Foodform> = Vec::new();
        for i in &mut data {
            let (f, m, g) = &i;
            let mut ff = Foodform::new(f);
            ff.manufacturer = (*(m.name)).to_string();
            ff.food_group = (*(g.description)).to_string();
            ffv.push(ff)
        }
        Ok(serde_json::to_string(&ffv)?)
    }
}*/

/*impl Browse for Food {
    fn browse(&self, r: &BrowseRequest, conn: &MysqlConnection) -> Result<String, Box<dyn Error>> {
        use crate::schema::foods::dsl::*;
        /*let q=foods
        .inner_join(manufacturers)
        .inner_join(food_groups)
        .limit(r.max)
        .offset(r.offset)
        .order(description);

        let debug = diesel::debug_query::<diesel::mysql::Mysql, _>(&q);
        println!("The query: {:?}", debug);*/
       let mut data = foods
           .inner_join(manufacturers)
            .inner_join(food_groups)
 //           .order(id)
            .limit(r.max)
            .offset(r.offset)
            .load::<(Food,Manufacturer,Foodgroup)>(conn)?;
        let mut ffv: Vec<Foodform> = Vec::new();
        for i in &mut data {
            let (f, m, g) = &i;
            let mut ff = Foodform::new(f);
            ff.manufacturer = (*(m.name)).to_string();
            ff.food_group = (*(g.description)).to_string();
            ffv.push(ff);
        }
        Ok(serde_json::to_string(&ffv)?)
    }
}*/
#[derive(Identifiable, Queryable, Associations, PartialEq, Serialize, Deserialize, Debug)]
#[table_name = "manufacturers"]
pub struct Manufacturer {
    pub id: i32,
    pub name: String,
}
impl Manufacturer {
    pub fn new() -> Self {
        Self {
            id: 0,
            name: String::from("Unknown"),
        }
    }
}

/*impl Get for Manufacturer {
    fn get(&self, conn: &MysqlConnection) -> Result<String, Box<dyn Error>> {
        let m = manufacturers.find(&self.id).first::<Manufacturer>(conn)?;
        Ok(serde_json::to_string(&m)?)
    }
}
impl Browse for Manufacturer {
    fn browse(&self, r: &BrowseRequest, conn: &MysqlConnection) -> Result<String, Box<dyn Error>> {
        let data = manufacturers
            .limit(r.max)
            .offset(r.offset)
            .load::<Manufacturer>(conn)?;
        Ok(serde_json::to_string(&data)?)
    }
}*/
#[derive(Queryable, Associations, Serialize, Deserialize, Debug)]
#[table_name = "food_groups"]
pub struct Foodgroup {
    pub id: i32,
    pub description: String,
}
impl Foodgroup {
    pub fn new() -> Self {
        Self {
            id: 0,
            description: String::from("Unknown"),
        }
    }
}
/*impl Get for Foodgroup {
    fn get(&self, conn: &MysqlConnection) -> Result<String, Box<dyn Error>> {
        use crate::schema::food_groups::dsl::*;
        let m = food_groups.find(&self.id).first::<Foodgroup>(conn)?;
        Ok(serde_json::to_string(&m)?)
    }
}
impl Browse for Foodgroup {
    fn browse(&self, r: &BrowseRequest, conn: &MysqlConnection) -> Result<String, Box<dyn Error>> {
        use crate::schema::food_groups::dsl::*;
        let data = food_groups
            .limit(r.max)
            .offset(r.offset)
            .load::<Foodgroup>(conn)?;
        Ok(serde_json::to_string(&data)?)
    }
}*/
#[derive(Identifiable, Queryable, Associations, PartialEq, Serialize, Deserialize, Debug)]
// Nutrient as in Calcium, Energy, etc, etc.
pub struct Nutrient {
    pub id: i32,
    pub nutrientno: String,
    pub description: String,
    pub unit: String,
}
impl Nutrient {
    pub fn new() -> Self {
        Self {
            id: 0,
            nutrientno: String::from("Unknown"),
            description: String::from("Unknown"),
            unit: String::from("Unknown"),
        }
    }
}
/*use crate::schema::nutrients::dsl::*;
impl Get for Nutrient {
    fn get(&self, conn: &MysqlConnection) -> Result<String, Box<dyn Error>> {

        let n = nutrients.find(&self.id).first::<Nutrient>(conn)?;
        Ok(serde_json::to_string(&n)?)
    }
}
impl Browse for Nutrient {
    fn browse(&self, r: &BrowseRequest, conn: &MysqlConnection) -> Result<String, Box<dyn Error>> {
        let data = nutrients
            .limit(r.max)
            .offset(r.offset)
            .load::<Nutrient>(conn)?;
        Ok(serde_json::to_string(&data)?)
    }
}*/
#[derive(Identifiable, Queryable, Associations, PartialEq, Serialize, Deserialize, Debug)]
#[belongs_to(Food)]
#[belongs_to(Nutrient)]
#[table_name = "nutrient_data"]
/// Nutrientdata links nutrients and foods, i.e. it describes the list of nutrient values for a given food
pub struct Nutrientdata {
    pub id: i32,
    pub value: f64,
    pub standard_error: Option<f64>,
    pub minimum: Option<f64>,
    pub maximum: Option<f64>,
    pub median: Option<f64>,
    pub derivation_id: i32,
    pub nutrient_id: i32,
    pub food_id: i32,
}
impl Nutrientdata {
    pub fn new() -> Self {
        Self {
            id: 0,
            value: 0.0,
            standard_error: None,
            minimum: None,
            maximum: None,
            median: None,
            derivation_id: 0,
            nutrient_id: 0,
            food_id: 0,
        }
    }
}
/*impl Get for Nutrientdata {
    fn get(&self, conn: &MysqlConnection) -> Result<String, Box<dyn Error>> {
        use crate::schema::nutrient_data::dsl::*;
        let nd = nutrient_data.find(&self.id).first::<Nutrientdata>(conn)?;
        Ok(serde_json::to_string(&nd)?)
    }
}*/

// Derivations are descriptions of how a nutrient value was derived.
#[derive(Identifiable, Queryable, Associations, PartialEq, Serialize, Deserialize, Debug)]
pub struct Derivation {
    id: i32,
    code: String,
    description: String,
}
impl Derivation {
    pub fn new() -> Self {
        Self {
            id: 0,
            code: String::from("Unknown"),
            description: String::from("Unknown"),
        }
    }
}
/*use crate::schema::derivations::dsl::*;
impl Get for Derivation {
    fn get(&self, conn: &MysqlConnection) -> Result<String, Box<dyn Error>> {

        let d = derivations.find(&self.id).first::<Derivation>(conn)?;
        Ok(serde_json::to_string(&d)?)
    }
}
impl Browse for Derivation {
    fn browse(&self, r: &BrowseRequest, conn: &MysqlConnection) -> Result<String, Box<dyn Error>> {
        let data = derivations
            .limit(r.max)
            .offset(r.offset)
            .load::<Derivation>(conn)?;
        Ok(serde_json::to_string(&data)?)
    }
}*/
