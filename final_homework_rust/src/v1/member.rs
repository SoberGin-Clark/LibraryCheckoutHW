#![allow(dead_code)]
#![allow(non_snake_case)]

use crate::v1::items::{ShowInfo, Info, Book, Dvd};
use crate::v1::catalog::{Catalog};

pub struct Member { // Create the member struct
    name : String,
    checkOuts : Vec<String>
}

impl Member {
    pub fn new(newName: &str) -> Self { // Constructor
        Member {
            name : newName.to_string(),
            checkOuts : Vec::new()
        }
    }

    pub fn borrow(&mut self, newID : &str) { // Borrow a new item- fail if 6th or have duplicate 
        let mut msg = "".to_string();
        for v in self.checkOuts.iter() {
            if *v == newID {
                msg = ("Cannot borrow what is already borrowed!").to_string();
            } else if self.checkOuts.len() >= 5 {
                msg = ("Cannot check out more than 5 at a time!").to_string();
            }
        }

        if msg == "".to_string() {
            self.checkOuts.push(newID.to_string());
        } else {
            panic!("{}", msg);
        }
    }

    pub fn name(&self) -> String { // Give name of member
        return self.name.to_string();
    }

    pub fn borrowed_ids(&self) -> &Vec<String> { // Give borrowed ids
        return &self.checkOuts;
    }

    pub fn return_item(&mut self, removal : &str) { // Return a borrowed id- fails if haven't borrowed the id given
        let removeMe = removal.to_string();

        for (i, v) in self.checkOuts.iter().enumerate() {
            if *v == removeMe {
                self.checkOuts.remove(i);
                return;
            }
        }
        panic!("Tried to remove an ID not borrowed!");
    }
}