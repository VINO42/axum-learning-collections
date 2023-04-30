use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]


pub struct TempUser {
    pub age:  Option<i32>,
    pub name:Option<String>,
}
impl  TempUser {
    
    pub fn new(age:Option<i32>,name:Option<String>) -> Self {
        TempUser {
            age,name
        }
}
}