extern crate diesel;

use crate::schema::{derivations, foods, manufacturers, nutrient_data, nutrients};
use crate::Browse;
use crate::Get;
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

    pub fn get_food_group_name(&self, conn: &MysqlConnection) -> Result<String, Box<dyn Error>> {
        use crate::schema::food_groups::dsl::*;
        let fg = food_groups
            .find(&self.food_group_id)
            .first::<Foodgroup>(conn)?;
        Ok(fg.description)
    }
    pub fn get_manufacturer_name(&self, conn: &MysqlConnection) -> Result<String, Box<dyn Error>> {
        use crate::schema::manufacturers::dsl::*;
        let m = manufacturers
            .find(&self.manufacturer_id)
            .first::<Manufacturer>(conn)?;
        Ok(m.name)
    }
    //
    pub fn get_nutrient_data(
        &self,
        nids: &Vec<String>,
        conn: &MysqlConnection,
    ) -> Result<Vec<NutrientdataForm>, Box<dyn Error>> {
        use crate::schema::derivations::dsl::*;
        use crate::schema::nutrient_data::dsl::*;
        use crate::schema::nutrients::dsl::*;
        let data = match nids.len()  {
            0 => nutrient_data
            .filter(food_id.eq(&self.id))
            .inner_join(nutrients)
            .inner_join(derivations)
            .load::<(Nutrientdata, Nutrient, Derivation)>(conn)?,
            _ => nutrient_data
            .filter(food_id.eq(&self.id))
            .inner_join(nutrients)
            .filter(nutrientno.eq_any(nids))
            .inner_join(derivations)
            .load::<(Nutrientdata, Nutrient, Derivation)>(conn)?,
        };
        let mut ndv: Vec<NutrientdataForm> = Vec::new();
        for i in &data {
            let (nd, n, d) = &i;
            let ndf = NutrientdataForm::create((nd, n, d));
            ndv.push(ndf);
        }
        Ok(ndv)
    }
}
impl Get for Food {
    type Item = Food;
    type Conn = MysqlConnection;
    fn get(&self, conn: &Self::Conn) -> Result<Vec<Self::Item>, Box<dyn Error>> {
        use crate::schema::foods::dsl::*;
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
}
impl Browse for Food {
    type Item = Food;
    type Conn = MysqlConnection;
    fn browse(
        &self,
        max: i64,
        off: i64,
        sort: String,
        order: String,
        conn: &Self::Conn,
    ) -> Result<Vec<Self::Item>, Box<dyn Error>> {
        use crate::schema::foods::dsl::*;
        let mut q = foods.into_boxed();
        match &*sort {
            "description" => q = q.order(Box::new(description.asc())),
            "upc" => q = q.order(Box::new(upc.asc())),
            "fdcId" => q = q.order(Box::new(fdc_id.asc())),
            _ => q = q.order(Box::new(id.asc())),
        };
        q = q.limit(max).offset(off);
        // let debug = diesel::debug_query::<diesel::mysql::Mysql, _>(&q);
        // println!("The query: {:?}", debug);
        let data = q.load::<Food>(conn)?;
        Ok(data)
    }
}
use crate::schema::food_groups::dsl::*;
use crate::schema::foods::dsl::*;
use crate::schema::manufacturers::dsl::*;

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
impl Get for Nutrient {
    type Item = Nutrient;
    type Conn = MysqlConnection;
    fn get(&self, conn: &Self::Conn) -> Result<Vec<Self::Item>, Box<dyn Error>> {
        use crate::schema::nutrients::dsl::*;
        let mut data = vec![];
        data = nutrients
            .filter(nutrientno.eq(&self.nutrientno))
            .load::<Nutrient>(conn)?;
        Ok(data)
    }
}
impl Browse for Nutrient {
    type Item = Nutrient;
    type Conn = MysqlConnection;
    fn browse(
        &self,
        max: i64,
        off: i64,
        sort: String,
        order: String,
        conn: &Self::Conn,
    ) -> Result<Vec<Self::Item>, Box<dyn Error>> {
        use crate::schema::nutrients::dsl::*;
        let mut q = nutrients.into_boxed();
        match &*sort {
            "description" => q = q.order(Box::new(description.asc())),
            "no" => q = q.order(Box::new(nutrientno.asc())),
            _ => q = q.order(Box::new(id.asc())),
        };
        q = q.limit(max).offset(off);
        // let debug = diesel::debug_query::<diesel::mysql::Mysql, _>(&q);
        // println!("The query: {:?}", debug);
        let data = q.load::<Nutrient>(conn)?;
        Ok(data)
    }
}
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
#[derive(Serialize, Deserialize, Debug)]
pub struct NutrientdataForm {
    pub value: f64,
    pub derivation: String,
    pub derivation_code: String,
    pub nutrient: String,
    pub nutrient_no: String,
    pub unit: String,
}
impl NutrientdataForm {
    pub fn new() -> Self {
        Self {
            value: 0.0,
            derivation: String::from("unknown"),
            derivation_code: String::from("unknown"),
            nutrient: String::from("unknown"),
            nutrient_no: String::from("unknown"),
            unit: String::from("unknown"),
        }
    }
    pub fn create((nd, n, d): (&Nutrientdata, &Nutrient, &Derivation)) -> Self {
        Self {
            value: nd.value,
            nutrient: (*(n.description)).to_string(),
            nutrient_no: (*(n.nutrientno)).to_string(),
            unit: (*(n.unit)).to_string(),
            derivation: (*(d.description)).to_string(),
            derivation_code: (*(d.code)).to_string(),
        }
    }
}
