extern crate diesel;

use self::diesel::prelude::*;
use crate::schema::{derivations, foods, manufacturers, nutrient_data, nutrients};
use crate::{Browse, Count, Get};
use chrono::{NaiveDate, NaiveDateTime};
use diesel::dsl::count_star;
use diesel::pg::expression::dsl::any;
use diesel::pg::Pg;
use diesel::pg::PgConnection;
use diesel_full_text_search::{plainto_tsquery,TsVectorExtensions};
use regex::Regex;
use std::error::Error;
#[derive(
    Identifiable,
    Queryable,
    Associations,
    PartialEq,
    AsChangeset,
    Clone,
    Serialize,
    Deserialize,
    Debug,
)]
#[belongs_to(Manufacturer)]
//#[table_name = "foods"]
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
// defines a constant for returning all columns except the tsvector
type FoodColumns = (
    foods::id,
    foods::publication_date,
    foods::modified_date,
    foods::available_date,
    foods::upc,
    foods::fdc_id,
    foods::description,
    foods::food_group_id,
    foods::manufacturer_id,
    foods::datasource,
    foods::serving_size,
    foods::serving_unit,
    foods::serving_description,
    foods::country,
    foods::ingredients,
);
const FOOD_COLUMNS: FoodColumns = (
    foods::id,
    foods::publication_date,
    foods::modified_date,
    foods::available_date,
    foods::upc,
    foods::fdc_id,
    foods::description,
    foods::food_group_id,
    foods::manufacturer_id,
    foods::datasource,
    foods::serving_size,
    foods::serving_unit,
    foods::serving_description,
    foods::country,
    foods::ingredients,
);
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

    pub fn get_food_group_name(&self, conn: &PgConnection) -> Result<String, Box<dyn Error>> {
        use crate::schema::food_groups::dsl::*;
        let fg = food_groups
            .find(&self.food_group_id)
            .first::<Foodgroup>(conn)?;
        Ok(fg.description)
    }
    pub fn get_manufacturer_name(&self, conn: &PgConnection) -> Result<String, Box<dyn Error>> {
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
        conn: &PgConnection,
    ) -> Result<Vec<NutrientdataForm>, Box<dyn Error>> {
        use crate::schema::derivations::dsl::*;
        use crate::schema::nutrient_data::dsl::*;
        use crate::schema::nutrients::dsl::*;
        let data = match nids.len() {
            0 => nutrient_data
                .filter(food_id.eq(&self.id))
                .inner_join(nutrients)
                .inner_join(derivations)
                .load::<(Nutrientdata, Nutrient, Derivation)>(conn)?,
            _ => nutrient_data
                .filter(food_id.eq(&self.id))
                .inner_join(nutrients)
                .filter(nutrientno.eq(any(nids)))
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
    type Conn = PgConnection;
    fn get(&self, conn: &Self::Conn) -> Result<Vec<Self::Item>, Box<dyn Error>> {
        use crate::schema;
        use crate::schema::foods::dsl::*;
        let q = schema::foods::table.into_boxed::<Pg>();
        let data;
        if self.upc != "unknown" {
            data = q
                .select(FOOD_COLUMNS)
                .filter(upc.eq(&self.upc))
                .load::<Food>(conn)?;
        } else if self.id > 0 {
            data = q
                .select(FOOD_COLUMNS)
                .filter(id.eq(&self.id))
                .load::<Food>(conn)?;
        } else {
            data = q
                .select(FOOD_COLUMNS)
                .filter(fdc_id.eq(&self.fdc_id))
                .load::<Food>(conn)?;
        }
        Ok(data)
    }
}
impl Browse for Food {
    type Item = Food;
    type Conn = PgConnection;
    fn browse(
        &self,
        max: i64,
        off: i64,
        sort: String,
        order: String,
        conn: &Self::Conn,
    ) -> Result<Vec<Self::Item>, Box<dyn Error>> {
        use crate::schema;
        use crate::schema::foods::dsl::*;
        let mut q = schema::foods::table.into_boxed::<Pg>();
        let mut query = "";
        if !self.description.is_empty() {
            query = &self.description;
        }
        if !query.is_empty() {
            q = q.filter(kw_tsvector.matches(plainto_tsquery(query)));
        }
        match &*sort {
            "description" => {
                q = match &*order {
                    "desc" => q.order(Box::new(description.desc())),
                    _ => q.order(Box::new(description.asc())),
                }
            }
            "upc" => {
                q = match &*order {
                    "desc" => q.order(Box::new(upc.desc())),
                    _ => q.order(Box::new(upc.asc())),
                }
            }
            "fdcId" => {
                q = match &*order {
                    "desc" => q.order(Box::new(fdc_id.desc())),
                    _ => q.order(Box::new(fdc_id.asc())),
                }
            }
            _ => {
                /* q = match &*order {
                    "desc" => q.order(Box::new(rank.desc())),
                    _ => q.order(Box::new(rank.asc())),
                }*/
            }
        };
        if self.food_group_id > 0 {
            q = q.filter(food_group_id.eq(self.food_group_id));
        }
        if self.manufacturer_id > 0 {
            q = q.filter(manufacturer_id.eq(self.manufacturer_id));
        }
        // build publication date range if we have at least one date
        let pubrange: String = match &self.ingredients {
            None => "".to_string(),
            Some(inner) => inner.to_string(),
        };
        if pubrange != "" {
            let dv = pubrange.split(":").collect::<Vec<&str>>();
            //let dv=ds.collect::<Vec<&str>>();
            let mut fdate = dv[0].to_string();
            let mut tdate = dv[0].to_string();
            // set through date if provided in request
            if dv.len() > 1 {
                tdate = dv[1].to_string();
            }
            let re = Regex::new(r"(?P<y>\d{4})[-/ ](?P<m>\d{2})[-/ ](?P<d>\d{2})").unwrap();
            fdate = re.replace_all(&fdate, "$y$m$d").to_string() + " 00:01:00";
            tdate = re.replace_all(&tdate, "$y$m$d").to_string() + " 23:59:00";
            let lhs = NaiveDateTime::parse_from_str(&fdate, "%Y%m%d %H:%M:%S")?;
            let uhs = NaiveDateTime::parse_from_str(&tdate, "%Y%m%d %H:%M:%S")?;
            q = q.filter(publication_date.between(lhs, uhs));
        }
        //let debug = diesel::debug_query::<Pg, _>(&q);
       //println!("The query: {:?}", debug);
        let data = q
            .select(FOOD_COLUMNS)
            .limit(max)
            .offset(off)
            .load::<Food>(conn)?;
        Ok(data)
    }
}
impl Count for Food {
    type Item = Food;
    type Conn = PgConnection;
    fn query_count(&self, conn: &Self::Conn) -> Result<i64, Box<dyn Error>> {
        use crate::schema::foods::dsl::*;
        let mut q = foods.into_boxed();
        let mut query = "";
        if !self.description.is_empty() {
            query = &self.description;
        }
        if !query.is_empty() {
            q = q.filter(kw_tsvector.matches(plainto_tsquery(query)));
        }
        if self.manufacturer_id > 0 {
            q = q.filter(manufacturer_id.eq(self.manufacturer_id));
        }
        if self.food_group_id > 0 {
            q = q.filter(food_group_id.eq(self.food_group_id));
        }
        // build publication date range if we have at least one date
        let pubrange: String = match &self.ingredients {
            None => "".to_string(),
            Some(inner) => inner.to_string(),
        };
        if pubrange != "" {
            let dv = pubrange.split(":").collect::<Vec<&str>>();
            //let dv=ds.collect::<Vec<&str>>();
            let mut fdate = dv[0].to_string();
            let mut tdate = dv[0].to_string();
            // set through date if provided in request
            if dv.len() > 1 {
                tdate = dv[1].to_string();
            }
            let re = Regex::new(r"(?P<y>\d{4})[-/ ](?P<m>\d{2})[-/ ](?P<d>\d{2})").unwrap();
            fdate = re.replace_all(&fdate, "$y$m$d").to_string() + " 00:01:00";
            tdate = re.replace_all(&tdate, "$y$m$d").to_string() + " 23:59:00";
            let lhs = NaiveDateTime::parse_from_str(&fdate, "%Y%m%d %H:%M:%S")?;
            let uhs = NaiveDateTime::parse_from_str(&tdate, "%Y%m%d %H:%M:%S")?;
            q = q.filter(publication_date.between(lhs, uhs));
        }
        //let debug = diesel::debug_query::<Pg, _>(&q);
        //println!("The query: {:?}", debug);
        let c = q.select(count_star()).first::<i64>(conn)?;

        Ok(c)
    }
}
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
    pub fn find_by_name(&self, conn: &PgConnection) -> Result<Manufacturer, Box<dyn Error>> {
        use crate::schema::manufacturers::dsl::*;
        let m = manufacturers
            .filter(name.eq(&self.name))
            .first::<Manufacturer>(conn)?;
        Ok(m)
    }
}
impl Get for Manufacturer {
    type Item = Manufacturer;
    type Conn = PgConnection;
    fn get(&self, conn: &Self::Conn) -> Result<Vec<Self::Item>, Box<dyn Error>> {
        use crate::schema::manufacturers::dsl::*;
        let data = manufacturers.find(&self.id).load::<Manufacturer>(conn)?;
        Ok(data)
    }
}
impl Browse for Manufacturer {
    type Item = Manufacturer;
    type Conn = PgConnection;
    fn browse(
        &self,
        max: i64,
        off: i64,
        sort: String,
        order: String,
        conn: &Self::Conn,
    ) -> Result<Vec<Self::Item>, Box<dyn Error>> {
        use crate::schema::manufacturers::dsl::*;
        let mut q = manufacturers.into_boxed();
        match &*sort {
            "name" => {
                q = match &*order {
                    "name" => q.order(Box::new(name.desc())),
                    _ => q.order(Box::new(name.asc())),
                }
            }
            _ => {
                q = match &*order {
                    "desc" => q.order(Box::new(id.desc())),
                    _ => q.order(Box::new(id.asc())),
                }
            }
        };
        q = q.limit(max).offset(off);
        // let debug = diesel::debug_query::<diesel::mysql::Mysql, _>(&q);
        // println!("The query: {:?}", debug);
        let data = q.load::<Manufacturer>(conn)?;
        Ok(data)
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
    pub fn find_by_description(&self, conn: &PgConnection) -> Result<Foodgroup, Box<dyn Error>> {
        use crate::schema::food_groups::dsl::*;
        let fg = food_groups
            .filter(description.eq(&self.description))
            .first::<Foodgroup>(conn)?;
        Ok(fg)
    }
}
impl Get for Foodgroup {
    type Item = Foodgroup;
    type Conn = PgConnection;
    fn get(&self, conn: &Self::Conn) -> Result<Vec<Self::Item>, Box<dyn Error>> {
        use crate::schema::food_groups::dsl::*;
        let data = food_groups.find(&self.id).load::<Foodgroup>(conn)?;
        Ok(data)
    }
}
impl Browse for Foodgroup {
    type Item = Foodgroup;
    type Conn = PgConnection;
    fn browse(
        &self,
        max: i64,
        off: i64,
        sort: String,
        order: String,
        conn: &Self::Conn,
    ) -> Result<Vec<Self::Item>, Box<dyn Error>> {
        use crate::schema::food_groups::dsl::*;
        let mut q = food_groups.into_boxed();
        match &*sort {
            "group" => {
                q = match &*order {
                    "desc" => q.order(Box::new(description.desc())),
                    _ => q.order(Box::new(description.asc())),
                }
            }
            _ => {
                q = match &*order {
                    "desc" => q.order(Box::new(id.desc())),
                    _ => q.order(Box::new(id.asc())),
                }
            }
        };
        q = q.limit(max).offset(off);
        // let debug = diesel::debug_query::<diesel::mysql::Mysql, _>(&q);
        // println!("The query: {:?}", debug);
        let data = q.load::<Foodgroup>(conn)?;
        Ok(data)
    }
}

#[derive(
    Identifiable, Queryable, Associations, PartialEq, Insertable, Serialize, Deserialize, Debug,
)]
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
    type Conn = PgConnection;
    fn get(&self, conn: &Self::Conn) -> Result<Vec<Self::Item>, Box<dyn Error>> {
        use crate::schema::nutrients::dsl::*;
        let data = nutrients
            .filter(nutrientno.eq(&self.nutrientno))
            .load::<Nutrient>(conn)?;
        Ok(data)
    }
}
impl Browse for Nutrient {
    type Item = Nutrient;
    type Conn = PgConnection;
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
            "name" => {
                q = match &*order {
                    "desc" => q.order(Box::new(description.desc())),
                    _ => q.order(Box::new(description.asc())),
                }
            }
            "nbr" => {
                q = match &*order {
                    "desc" => q.order(Box::new(nutrientno.desc())),
                    _ => q.order(Box::new(nutrientno.asc())),
                }
            }
            _ => {
                q = match &*order {
                    "desc" => q.order(Box::new(id.desc())),
                    _ => q.order(Box::new(id.asc())),
                }
            }
        };
        q = q.limit(max).offset(off);
        // let debug = diesel::debug_query::<diesel::mysql::Mysql, _>(&q);
        // println!("The query: {:?}", debug);
        let data = q.load::<Nutrient>(conn)?;
        Ok(data)
    }
}
#[derive(
    Identifiable, Queryable, Associations, PartialEq, Insertable, Serialize, Deserialize, Debug,
)]
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
#[derive(
    Identifiable, Queryable, Associations, PartialEq, Insertable, Serialize, Deserialize, Debug,
)]
pub struct Derivation {
    pub id: i32,
    pub code: String,
    pub description: String,
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
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn new_nutrientdata_form() {
        let nf = NutrientdataForm::new();
        assert_eq!(0.0, nf.value);
        assert_eq!("unknown", nf.derivation);
        assert_eq!("unknown", nf.derivation_code);
        assert_eq!("unknown", nf.nutrient_no);
        assert_eq!("unknown", nf.nutrient);
        assert_eq!("unknown", nf.unit);
    }
    #[test]
    fn create_nutrientdata_form() {
        let n = Nutrient {
            id: 0,
            description: String::from("A nutrient"),
            nutrientno: String::from("999"),
            unit: String::from("g"),
        };
        let d = Derivation {
            id: 0,
            description: String::from("Some derivation"),
            code: String::from("LXXX"),
        };
        let nd = Nutrientdata::new();
        let nf = NutrientdataForm::create((&nd, &n, &d));
        assert_eq!(0.0, nf.value);
        assert_eq!("Some derivation", nf.derivation);
        assert_eq!("LXXX", nf.derivation_code);
        assert_eq!("999", nf.nutrient_no);
        assert_eq!("A nutrient", nf.nutrient);
        assert_eq!("g", nf.unit);
    }
    #[test]
    fn new_food() {
        let f = Food::new();
        assert_eq!(0, f.id);
        assert_eq!(
            NaiveDate::from_ymd(1970, 01, 01).and_hms(00, 00, 00),
            f.publication_date
        );
        assert_eq!(
            NaiveDate::from_ymd(1970, 01, 01).and_hms(00, 00, 00),
            f.modified_date
        );
        assert_eq!(
            NaiveDate::from_ymd(1970, 01, 01).and_hms(00, 00, 00),
            f.available_date
        );
        assert_eq!("unknown", f.upc);
        assert_eq!("unknown", f.fdc_id);
        assert_eq!("unknown", f.description);
        assert_eq!(0, f.food_group_id);
        assert_eq!(0, f.manufacturer_id);
        assert_eq!("unknown", f.datasource);
        assert_eq!(None, f.serving_size);
        assert_eq!(None, f.serving_unit);
        assert_eq!(None, f.serving_description);
        assert_eq!(None, f.country);
        assert_eq!(None, f.ingredients);
    }
}
