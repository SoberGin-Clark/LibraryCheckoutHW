#![allow(dead_code)]
#![allow(non_snake_case)]

use crate::v1::items::{ShowInfo, Info, Book, Dvd};

pub struct Catalog { // Create the catalog struct
    catList: Vec<Box<dyn ShowInfo>>
}

impl Catalog {
    pub fn new() -> Self { // Constructor
        Catalog {
            catList: Vec::new()
        }
    }

    pub fn add(&mut self, input: Box<dyn ShowInfo>) { // Add new item to the catalog- fails if have duplicate already inside
        let mut msg = "".to_string();
        for i in self.catList.iter() {
            if input.giveInfo().showId() == i.giveInfo().showId() {
                msg = "Id already in use in list!".to_string();
            }
        }

        if msg == "".to_string() {
            self.catList.push(input);
        } else {
            panic!("{}", msg);
        }
    }

    pub fn get(&self, id: &str) -> Info { // Get information on the provided id. fails if not present in catalog
        let trueId = id.to_string();
        for i in self.catList.iter() {
            if trueId == i.giveInfo().showId() {
                let temp = i.giveInfo();
                return temp;
            }
        }
        panic!("Couldn't get value for ID- not in catalog!");
    }

    pub fn details_for(&self, idsList : Vec<String>) -> Vec<(String, String, String)> { 
        // Get the details of a member's borrowed items. fails if item is asked for that's not in the catalog
        let mut finalReturn : Vec<(String, String, String)> = Vec::new();  

        for o in idsList {
            let mut check = false;
            for i in self.catList.iter() {
                check = true;
                if *o == i.giveInfo().showId().to_string() {
                    let temp = i.giveInfo();
                    let tempVec = (temp.showId(), temp.showTitle(), temp.showDays());
                    finalReturn.push(tempVec);
                    check = false;
                    break;
                }
            }
            if check == true { 
                panic!("Couldn't get value for ID- not in catalog!"); 
            }
        }
        return finalReturn;
    }
}