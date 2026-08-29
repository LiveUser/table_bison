use beve::{Value, Key, Object};
use std::collections::{BTreeMap, btree_map};
use dart_io::{Directory, File, FileSystemEntity};

pub struct ErrorPerformingTheOperation;

pub struct DbTable{
    pub table_path:String,
}
impl DbTable{
    //Create item and return uuid of the item
    fn create_table(&self) -> () {
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
        let full_file_path:String = format!("{}/{}..beve",table_directory.full_path,unique_number.to_string());
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
        let obj: Object = BTreeMap::new();
        let beve_value:Value = Value::Object(obj);
        match beve::to_vec(&beve_value) {
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
            Err(_) => {
                return Err(ErrorPerformingTheOperation);
            },
        }
    }
    pub fn view(&self, uuid:u64,) -> Result<BTreeMap<Key,Value>, ErrorPerformingTheOperation>{
        let full_path = self.generate_file_path(uuid);
        let file:File = File{
            full_path: full_path,
        };
        //Read file
        let bytes:Vec<u8> = file.read_as_bytes();
        match beve::from_slice::<Value>(&bytes) {
            Err(_) => {
                return Err(ErrorPerformingTheOperation);
            },
            Ok(Value::Object(mut object)) => {
                object.insert(Key::String("uuid".to_string()), Value::Number(beve::Number::U64(uuid)));
                return Ok(object);
            },
            Ok(_) => {
                return Err(ErrorPerformingTheOperation);
            },
        }
    }
    pub fn insert(&self, uuid:u64, key:String, value:Value) -> Result<(),ErrorPerformingTheOperation>{
        match self.view(uuid) {
            Err(_)=> {
                return Err(ErrorPerformingTheOperation);
            },
            Ok(mut object)=>{
                //insert data
                object.insert(Key::String(key), value);
                //write back to the file system
                let beve_value:Value = Value::Object(object);
                match beve::to_vec(&beve_value) {
                    Ok(bytes) => {
                        let full_path = self.generate_file_path(uuid);
                        let file:File = File{
                            full_path: full_path,
                        };
                        file.write_as_bytes(bytes);
                        return Ok(());
                    },
                    Err(_) => {
                        return Err(ErrorPerformingTheOperation);
                    },
                }
            },
        }
    }
    pub fn get(&self, uuid:u64, key:String) -> Option<Value>{
        match self.view(uuid) {
            Err(_)=>{
                return None;
            },
            Ok(object)=>{
                match object.get(&Key::String(key)) {
                    None=>{
                        return None;
                    },
                    Some(value)=>{
                        return  Some(value.clone());
                    }
                }
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
    pub fn iterator(&self, callback: &mut dyn FnMut(BTreeMap<Key, Value>)) -> (){
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
                    match file_uuid.find("/") {
                        None=>{
                            //Do nothing
                        },
                        Some(slash_index)=>{
                            let length = file_uuid.len();
                            file_uuid = file_uuid[slash_index..length].to_string();
                            match file_uuid.find(".beve") {
                                None=>{
                                    //Do nothing
                                },
                                Some(extension_index)=>{
                                    file_uuid = file_uuid[0..extension_index].to_string();
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
    use beve::Number;

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
                let _ = table.insert(uuid, "product".to_string(), Value::String("Steak".to_string()));
                let _ = table.insert(uuid, "price".to_string(), Value::Number(Number::F64(f64::from(4.78))));
                let _ = table.insert(uuid, "amount".to_string(), Value::Number(Number::U64(17 as u64)));
                match table.view(uuid) {
                    Err(_)=>{
                        //Nothing
                    },
                    Ok(object)=>{
                        println!("{:?}", object); 
                    }
                }
                match table.get(uuid, "product".to_string()) {
                    Some(value)=> {
                        match value {
                            Value::String(text)=>{
                                println!("{}",text);
                            },
                            _=> {},
                        }
                    },
                    _=> {},
                };
            }
        }
        //Delete all records
        table.iterator(&mut |object|{
            match object.get(&Key::String("uuid".to_string())) {
                None =>{

                },
                Some(value)=>{
                    match value {
                        Value::Number(uuid)=> {
                            table.remove_record(uuid.as_u64().unwrap());
                        },
                        _=>{
                            //Ignore other data types
                        },
                    }
                },
            }
        });
    }
}
