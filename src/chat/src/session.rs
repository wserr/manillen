use crate::Config;
use actix_session::Session;
use actix_web::body::BoxBody;
use actix_web::error::ErrorInternalServerError;
use actix_web::get;
use actix_web::Result;
use actix_web::http::header::ContentType;
use actix_web::{HttpRequest, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

#[get("/session-test")]
pub async fn session_test(session: Session) -> Result<impl Responder> {
    println!("{:?}", session.entries());
    // access session data
    if let Some(count) = session.get::<i32>("counter")? {
        session.insert("counter", count + 1)?;
    } else {
        session.insert("counter", 1)?;
    }
    let count = session.get::<i32>("counter")?.unwrap();

    let mut input : Vec<i32> = Vec::new();

    input.push(1);
    let result = get_x_th_element_from_array(input, 1);

    //input.push(1);

    let mut result_string : String = "".to_string();
    if let Some(value) = result {
        result_string = format!("Some value was found! {:?}", value);
    } else {
        result_string = "No value was found :(".to_string();
    }

    Ok(result_string)
}

fn count(mut input: i32) -> i32 {
    input = input + 1;
    input
}

fn get_x_th_element_from_array(input: Vec<i32>, index: usize) -> Option<i32> {
    // Option = Some(value)
    // Option = None

    if (index > input.len()) 
    {
        return None;
    }
    return Some(input[index]);
}

#[test]
fn test_counter() {
    let first: i32 = 0;
    let result = count(first);
    assert_eq!(1, result);
}

#[test]
fn get_x_th_element_from_array_test() {
    let mut input: Vec<i32> = Vec::new();
    input.push(1);
    assert_eq!(Some(1), get_x_th_element_from_array(input, 0));
}