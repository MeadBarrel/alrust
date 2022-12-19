use std::fs::File;
use serde::{Serialize, Deserialize};
use serde_json::{from_reader, to_writer_pretty};
use derive_more::*;


#[derive(Serialize, Deserialize, Into, Debug, From)]
#[serde(rename(deserialize="SomeStruct"))]
struct SomeStructV1 {
    a: String
}


#[derive(Serialize, Deserialize, Into, Debug, From)]
#[serde(rename(deserialize="UserStruct"))]
struct UserStructV1 {
    a: SomeStructV1,
    b: String
}

#[derive(Serialize, Deserialize, Debug, From)]
#[serde(rename(deserialize="SomeStruct"))]
struct SomeStruct {
    b: String
}


#[derive(Serialize, Deserialize, Debug, From)]
struct UserStruct {
    a: SomeStruct,
    b: String
}


fn main() {
    let reader = File::open("test.json").unwrap();
    let some_struct: SomeStructV1 = from_reader(reader).unwrap();
    let converted: UserStruct = some_struct.into();


//     let writer = File::create("test.json").unwrap();
    
//     let user_struct = UserStructV1 {
//         b: "ololo".to_string(),
//         a: SomeStructV1 { a: "govno".to_string() }
//     };

//     to_writer_pretty(writer, &user_struct).unwrap();
}