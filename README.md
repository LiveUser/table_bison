# Table Bison
A BSON based data storage system.
## Breaking changes
Switched from BEVE to BSON for cross compatibility. I want to build a similar library for dart but no BEVE package is available in the dart ecosystem.

## Class
- DbTable
An object with a file system path pointing to the folder where all of the table data will be stored.
~~~rs
pub struct DbTable{
    table_path:String,
}
~~~
## Methods
- create_table
Creates the folder that DbTable table_path points to.
- create_record
Creates an object (you may also see it as a row if you come from SQL) and returns its uuid
- view
Returns a BTreeMap for the given uuid.
- insert
Inserts data into the object with the specified uuid
- get
Gets the corresponding value for a given a uuid and key.
- remove_record
Deletes the object with the specified uuid
- iterator
Allows to easily index through a table


## Full example
~~~rs
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
~~~