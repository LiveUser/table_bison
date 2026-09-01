use dart_io::{Directory, File, FileSystemEntity};
use easy_bson::{Dynamic,Map};

pub struct ErrorPerformingTheOperation{
    pub message:String,
}

pub struct DbTable{
    pub table_path:String,
}
impl DbTable{
    //Create item and return uuid of the item
    pub fn create_table(&self) -> () {
        let table_directory:Directory = Directory{
            full_path: self.table_path.clone(),
        };
        if !table_directory.exists() {
            table_directory.create_sync();
        }
    }
    fn generate_file_path(&self, unique_number:u64) -> String{
        let table_directory:Directory = Directory{
            full_path: self.table_path.clone(),
        };
        let full_file_path:String = format!("{}/{}.bson",table_directory.full_path,unique_number.to_string());
        return full_file_path;
    }
    fn get_unused_uuid(&self) -> u64{
        let mut is_unique:bool = false;
        let mut unique_number:u64 = 1;
        while !is_unique {
            let full_file_path:String = self.generate_file_path(unique_number);
            let file:File = File { 
                full_path: full_file_path,
            };
            is_unique = !file.exists();
            if !is_unique{
                unique_number += 1;
            }
        }
        return  unique_number;
    }
    pub fn create_record(&self) -> Result<u64,ErrorPerformingTheOperation>{
        //Create table if it does not exist
        self.create_table();
        //Find a non existent file name
        let uuid:u64 = self.get_unused_uuid();
        let obj:Map = Map::new();
        match obj.save_as_bytes(){
            Ok(bytes) => {
                let full_path = self.generate_file_path(uuid);
                let new_file:File = File {
                    full_path: full_path,
                };
                new_file.create_sync();
                new_file.write_as_bytes(bytes);
                return Ok(
                    uuid,
                );
            },
            Err(error) => {
                return Err(ErrorPerformingTheOperation{
                    message: error.message,
                });
            },
        }
    }
    pub fn view(&self, uuid:u64,) -> Result<Map, ErrorPerformingTheOperation>{
        let full_path = self.generate_file_path(uuid);
        let file:File = File{
            full_path: full_path,
        };
        //Read file
        let bytes:Vec<u8> = file.read_as_bytes();
        match Map::load_from_bytes(bytes) {
            Err(_) => {
                return Err(ErrorPerformingTheOperation{
                    message: "Invalid BSON.".to_string(),
                });
            },
            Ok(mut object) => {
                object.insert("uuid".to_string(), Dynamic::Number(uuid as f64));
                return Ok(object);
            },
        }
    }
    pub fn insert(&self, uuid:u64, key:String, value:Dynamic) -> Result<(),ErrorPerformingTheOperation>{
        match self.view(uuid) {
            Err(error)=> {
                return Err(error);
            },
            Ok(mut object)=>{
                //insert data
                object.insert(key, value);
                match object.save_as_bytes() {
                    Ok(bytes) => {
                        let full_path = self.generate_file_path(uuid);
                        let file:File = File{
                            full_path: full_path,
                        };
                        file.write_as_bytes(bytes);
                        return Ok(());
                    },
                    Err(error) => {
                        return Err(ErrorPerformingTheOperation{
                            message: error.message,
                        });
                    },
                }
            },
        }
    }
    pub fn get(&self, uuid:u64, key:String) -> Dynamic{
        match self.view(uuid) {
            Err(_)=>{
                return Dynamic::Null;
            },
            Ok(mut object)=>{
                return object.get(key);
            },
        }
    }
    //TODO: Remove record
    pub fn remove_record(&self, uuid:u64) -> (){
        let full_path = self.generate_file_path(uuid);
        let file:File = File{
            full_path: full_path,
        };
        file.delete_sync();
    }
    //TODO: Iterator
    pub fn iterator(&self, callback: &mut dyn FnMut(Map)) -> (){
        let table_contents:Vec<FileSystemEntity> = Directory{
            full_path: self.table_path.clone(),
        }.list_contents();
        for file_system_entity in table_contents{
            match file_system_entity {
                FileSystemEntity::Directory(_)=>{
                    //Do nothing
                },
                FileSystemEntity::File(file)=>{
                    let mut file_uuid:String = file.full_path;
                    match file_uuid.rfind("/|\\") {
                        None=>{
                            //Do nothing
                        },
                        Some(slash_index)=>{
                            match file_uuid.rfind(".bson") {
                                None=>{
                                    //Do nothing
                                },
                                Some(extension_index)=>{
                                    file_uuid = file_uuid[(slash_index + 1)..extension_index].to_string();
                                    println!("--------------{}", file_uuid);
                                    match self.view(file_uuid.parse::<u64>().unwrap()) {
                                        Err(_)=>{
                                            //Ignore corrupted file
                                        },
                                        Ok(object)=>{
                                            callback(object);
                                        },
                                    }
                                },
                            }
                        },
                    }
                }
            }
        }
    }

}

#[cfg(test)]
mod tests {

use super::*;

    #[test]
    fn basic_tests() {
        let table:DbTable = DbTable {
            table_path: "./inventory".to_string(),
        };
        //Creates table if it does not exist
        table.create_table();
        match table.create_record() {
            Err(_)=>{
                //DO nothing
            },
            Ok(uuid)=>{
                let _ = table.insert(uuid, "product".to_string(), Dynamic::String("Steak".to_string()));
                let _ = table.insert(uuid, "price".to_string(), Dynamic::Number(4.78 as f64));
                let _ = table.insert(uuid, "amount".to_string(), Dynamic::Number(17 as f64));
                match table.view(uuid) {
                    Err(_)=>{
                        //Nothing
                    },
                    Ok(object)=>{
                        println!("{:?}", object); 
                    }
                }
                match table.get(uuid, "product".to_string()) {
                    Dynamic::String(text)=>{
                        println!("{}",text);
                    },
                    _=> {},
                };
            }
        }
        //Delete all records
        table.iterator(&mut |mut object|{
            match object.get("uuid".to_string()) {
                Dynamic::Number(uuid)=>{
                    table.remove_record(uuid.round() as u64);
                }
                _=>{}
            }
        });
    }
}
