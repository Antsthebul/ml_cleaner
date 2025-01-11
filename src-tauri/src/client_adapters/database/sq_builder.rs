use std::{collections::HashMap, ops::Range};

use postgres_types::ToSql;

pub struct SQBuilderError(pub String);

#[derive(Default)]
pub struct SQBuilder{
    query_string: String,
    table: String,
    columns: Vec<String>,
    values: Vec<Box<dyn ToSql + Sync>>
}

impl SQBuilder{
    pub fn new() -> Self{
        Default::default()
    }

    pub fn insert(mut self) -> Self{
        self.query_string = "INSERT INTO".into();
        return self
    }
    pub fn use_table(mut self, table:&str) -> Self{
        self.table = table.into();
        return self
    }

    pub fn with_columns(mut self, cols:Vec<&str>) -> Self{
        self.columns = cols.into_iter().map(|f|f.into()).collect();
        return self
    }

    pub fn with_values<T>(mut self, vals: Vec<Box<dyn ToSql + Sync>>) -> Self{
        self.values = vals;
        return self
    }

    pub fn build(self) -> Result<(String, Vec<Box<dyn ToSql + Sync>>), SQBuilderError>{
        let col_len = self.columns.len();
        let val_len = self.values.len();

        if col_len != val_len{
            let error_msg = format!("{col_len} != {val_len}");
            return Err(SQBuilderError(format!("length of cols must match vals. {error_msg}")))
        }

        Ok((format!("{} {} {} {}", self.query_string, self.table, 
            self.create_col_string(&self.columns), self.create_value_string()), 
            self.values))
    }

    fn create_col_string(&self, data:&Vec<String>)-> String{
        format!("({})",data.join(","))
    }

    fn create_value_string(&self) -> String{
        let value_string = 0..self.values.len();
        
        value_string.into_iter().map(|v| format!("${v}"))
            .collect::<Vec<String>>().join(" ").to_owned()
    }
}