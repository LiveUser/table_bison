use beve::{Value, Key, Object};
use std::collections::BTreeMap;
use dart_io::{Directory, File, FileSystemEntity};

pub struct ErrorPerformingTheOperation;

pub enum DataType {
    String(String),
    Number(i128),
    Boolean(bool),
    Null,
}
pub struct DbTable{
    table_path:String,
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
    fn generate_file_path(&self, unique_number:u128) -> String{
        let table_directory:Directory = Directory{
            full_path: self.table_path.clone(),
        };
        let full_file_path:String = format!("{}/{}..beve",table_directory.full_path,unique_number.to_string());
        return full_file_path;
    }
    fn get_unused_uuid(&self) -> u128{
        let mut is_unique:bool = false;
        let mut unique_number:u128 = 1;
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
    pub fn create_item(&self) -> Result<u128,ErrorPerformingTheOperation>{
        //Create table if it does not exist
        self.create_table();
        //Find a non existent file name
        let uuid:u128 = self.get_unused_uuid();
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
    pub fn insert(&self, uuid:u128, key:String, value:Value) -> Result<(),ErrorPerformingTheOperation>{
        let full_path = self.generate_file_path(uuid);
        let file:File = File{
            full_path: full_path,
        };
        //Read file
        let mut bytes:Vec<u8> = file.read_as_bytes();
        match beve::from_slice::<Value>(&bytes) {
            Err(_) => {
                return Err(ErrorPerformingTheOperation);
            },
            Ok(Value::Object(mut object)) => {
                //insert data
                object.insert(Key::String(key), value);
                //write back to the file system
                let beve_value:Value = Value::Object(object);
                match beve::to_vec(&beve_value) {
                    Ok(bytes) => {
                        file.write_as_bytes(bytes);
                        return Ok(());
                    },
                    Err(_) => {
                        return Err(ErrorPerformingTheOperation);
                    },
                }
            },
            Ok(_) => {
                return Err(ErrorPerformingTheOperation);
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
