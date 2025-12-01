
#![allow(dead_code)]
#![allow(non_snake_case)]

pub trait ShowInfo { // ShowInfo trait for the different types of item
    fn new(id: &str, title: &str) -> Self where Self: Sized;
    fn giveInfo(&self) -> Info;
}

pub struct Info { // Info struct for the actual information so it can be used outside of a specific data type
    pub id: String,
    pub title : String,
    pub days_allowed: u32
}

impl Info { // Functions for retrieving the data
    pub fn showId(&self) -> String {
        return self.id.to_string();
    }

    pub fn showTitle(&self) -> String {
        return self.title.to_string();
    }

    pub fn showDays(&self) -> String {
        return self.days_allowed.to_string();
    }
}

pub struct Book { // Book structure
    myInfo: Info
}

impl ShowInfo for Book {
    fn new(newId: &str, newTitle: &str) -> Self where Self: Sized {
        Book {
            myInfo : Info {
                id : newId.to_string(),
                title : newTitle.to_string(),
                days_allowed : 7 // Special 7 day allowance
            }
        }
    }

    fn giveInfo(&self) -> Info {
        let temp = Info {
            id : self.myInfo.id.clone(),
            title : self.myInfo.title.clone(),
            days_allowed : self.myInfo.days_allowed.clone()
        };
        return temp;
    }
}

pub struct Dvd { // Dvd structure
    myInfo: Info
}

impl ShowInfo for Dvd {
    fn new(newId: &str, newTitle: &str) -> Self where Self: Sized {
        Dvd {
            myInfo : Info {
                id : newId.to_string(),
                title : newTitle.to_string(),
                days_allowed : 14 // Special 14 day allowance
            }
        }
    }

    fn giveInfo(&self) -> Info {
        let temp = Info {
            id : self.myInfo.id.clone(),
            title : self.myInfo.title.clone(),
            days_allowed : self.myInfo.days_allowed.clone()
        };
        return temp;
    }
}