use axum::{
    extract::{Extension, Query},
    http::StatusCode,
    Json,
};

use serde::{Deserialize,Serialize};
use serde_json::json;

use crate::{models::{Account, User}, handler::{MyError, type_of}};
use mongodb::{Collection,Database,bson::{doc,serde_helpers}, options::{Acknowledgment, ReadConcern, TransactionOptions, WriteConcern},};

#[derive(Deserialize)]
pub struct UserParams {
    pub username: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Users {

    // this line states that convert the object id into string 
    #[serde(
        deserialize_with = "serde_helpers::deserialize_hex_string_from_object_id"
    )]

    pub _id: String,
    pub username:String

 
}


#[axum_macros::debug_handler]
pub async fn get_balance(Extension(db): Extension<Database>, user_params: Query<UserParams>) -> Result<Json<String>, MyError> {
    // let user_id = user_params.username;
    let collection:Collection<Users> = db.collection("users");

    println!("userparams {}",type_of(&user_params.username.clone()) );
    let  query = doc! { "username":"Deepak"};

    // let user_result = collection
    //     .find_one(
    //         query
           
    //     )
    //     .await;

        let user_result = collection.find_one(query).await.map_err(|err| {
            // Log or handle the error as needed
            eprintln!("Failed to find user: {:?}", err);
            err
        })?;

        match user_result {
            Some(user) =>{

                
                
               
                 let account_collection:Collection<Account> = db.collection("accounts");
                 let obj_id  = user._id;
                 let user_obj_id = format!("ObjectId(\"{}\")", obj_id.to_string());

                 let account = account_collection.find_one(doc! {
                    
                     "user_id":user_obj_id
                 }).await?;
                 let account_balance = account.as_ref().map(|acct| acct.balance).unwrap();
                 
                 Ok(Json(account_balance.to_string()))
                 
                
        
        
        
        },
            None => {
                println!("No user found.");
                Err(MyError::MONGOERROR)

            }
        }
    
        
   

 
       
   


    //    let user_id = match user_result{
    //     Ok(Some(user))=> user.username,
    //     Ok(None) => "MyError::MONGOERROR".into(),
    //     Err(err)=> ("StatusCode::INTERNAL_SERVER_ERROR.into() {}").into()};
    //     println!("user_id {}", user_id);
        
    //    };
    //     // let user_id = user_result.as_ref().map(|user| user.id.clone()).unwrap();
        
        /*
        1. user_result is an Option<Users>, which means it can either be Some(Users) or None.
        2. as_ref() is a method that converts Option<Users> to Option<&Users>. This is necessary because the map() 
            method expects a reference to the value inside the Option.
        3. map(|user| user.id) is a closure that takes a reference to a Users struct (&Users) and extracts the id field. 
            This closure is applied to the value inside the Option<&Users>
        4. The map() method returns a new Option with the transformed value, in this case, an Option<u32>.
        5. unwrap_or(0) is used to handle the case where user_result is None. If user_result is None, the map() 
           method will return None, and unwrap_or(0) will return the default value of 0. If user_result is Some(Users), 
            unwrap_or(0) will return the id value from the Users struct.       
        
        */



        
       
  

   
}
