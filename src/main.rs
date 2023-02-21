
use mongodb::{bson::{doc, Document, oid::ObjectId, from_document},Client, options::ClientOptions, Collection};
use tokio;
use serde::{Deserialize,Serialize};
//use future::{stream::TryStreamExt,StreamExt};
//use std::env;
use std::future;
use std::error::Error;
#[derive(Debug,Serialize,Deserialize)]
pub struct Student{
    #[serde(rename="_id")]
    id: ObjectId,
    name: String,
    email: String,
    course: String,
    university: String,
 }
 
#[tokio::main]
async fn main(){
    
    //client.database("admin").run_command(doc!{"ping":1},None).await?;
   // println!("connected successfully.");
    // for db_name in client.list_database_names(None, None).await?{
    //     println!("{}",db_name);
    // }
    // Ok(())
    let client = mongo_connect().await;
   let col= get_collection(client);
   let mut read_choice = true;
   while read_choice{
   let mut choice = String::new();
   println!("1: insert \n 2:update \n3:search \n4:delete ");
   println!("enter your choice:");
   std::io::stdin().read_line(&mut choice).unwrap();
   choice = choice.trim().to_string();
   match choice.as_str(){
    "1"=> {add_data(col.clone()).await;},
    "2"=> {update_data(col.clone()).await;},
    "3"=> {search_data(col.clone()).await;},
    "4"=>{delete_data(col.clone()).await;},
    "5"=>{println!("exited");
            read_choice=false;},
    _ => {println!("enter valid choice")}
   }}
    // let books = vec![
    //     Book{
    //         title:"abc".to_string(),
    //         author:"raphel".to_string(),
    //     },
    //     Book{
    //         title:"def".to_string(),
    //         author: "harry".to_string(),
    //     },
    // ];
     //collection.insert_many(books,None).await.unwrap();
//     for collection_name in db.list_collection_names(None).await?{
//         println!("{}",collection_name);
//     }
//     Ok(()
}
async fn add_data(collection:Collection<Student>){
    println!("enter name:");
    let mut sname= String::new();
    std::io::stdin().read_line(&mut sname).unwrap();
    sname = sname.trim().to_string();

    println!("enter email:");
    let mut semail = String::new();
    std::io::stdin().read_line(&mut semail).unwrap();
    semail=semail.trim().to_string();

    println!("enter course:");
    let mut scourse= String::new();
    std::io::stdin().read_line(&mut scourse).unwrap();
    scourse=scourse.trim().to_string();

    println!("enter university:");
    let mut uni = String::new();
    std::io::stdin().read_line(&mut uni).unwrap();
    uni = uni.trim().to_string();

    let  mut data = Student{
        id: ObjectId::new(),
        name: sname.to_string(),
        email: semail.to_string(),
        course: scourse.to_string(),
        university: uni.to_string(),
    };
    collection.insert_one(data,None).await.unwrap();
}
async fn update_data(collection:Collection<Student>){
    let mut sname = String::new();
    println!("enter name of the student:");
    std::io::stdin().read_line(&mut sname).unwrap();
    sname = sname.trim().to_string();
    let filter = doc!{"name":sname.to_owned()};
    let mut change = String::new();
    println!("enter field you want to change:");
    std::io::stdin().read_line(&mut change).unwrap();
    change=change.trim().to_string();
    let mut val = String::new();
    println!("{}:enter value",change);
    std::io::stdin().read_line(&mut val).unwrap();
    val=val.trim().to_string();
    let update = doc!{"$set":{change:val}};
    let result= collection.update_one(filter,update,None).await.unwrap();
    //println!("in progress");
}
async fn search_data(collection:Collection<Student>){
    let mut sname = String::new();
    println!("enter name of the student:");
    std::io::stdin().read_line(&mut sname).unwrap();
    sname = sname.trim().to_string();
    let filter = doc!{"name":sname.to_owned()};
    let result = collection.find_one(filter,None).await.unwrap();
    match result{
        Some(ref document)=>{
            let name = &document.name;
            let email = &document.email;
            let course = &document.course;
            let uni = &document.university;

            println!("name:{}, email: {}, course:{}, university:{}",name,email,course,uni);
        },
        None=>println!("not found")
    }
   // println!("in progress");
}
async fn delete_data(collection:Collection<Student>){
    let mut sname = String::new();
    println!("enter name of the student:");
    std::io::stdin().read_line(&mut sname).unwrap();
    sname = sname.trim().to_string();
    let filter = doc!{"name":sname.to_owned()};
    let result = collection.delete_one(filter,None).await.unwrap();
   // println!("in progress");
}

pub async fn mongo_connect()->Client{
    let mut client_options = ClientOptions::parse("mongodb://localhost:27017").await.unwrap();
    //client_options.app_name = Some("Rust Demo".to_string());
    let client = Client::with_options(client_options).unwrap();
    client
}

pub fn get_collection(client:Client)->Collection<Student>{
    let db = client.database("mydatabase");
    let collection_list = db.collection::<Student>("Student");
    collection_list
}