# Table Bison
A BEVE based data storage system.
--- Under Development ---
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


## Full example (I'm new to Rust so the code is a mess)
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
        None =>
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
~~~