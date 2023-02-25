use std::collections::HashMap;

use gloo_net::{http::Request, Error};

use crate::constants::API_ROOT;

pub async fn get_text(parameters: &HashMap<String, String>, route: String) -> Result<String, Error>{
    let mut param_string: String = "?".to_owned();
    let mut first: bool = true;
    for (param, value) in parameters {
        param_string = format!("{param_string}{}{param}={value}", if first { "" } else { "?" });
        first = false;
    }

    match Request::get(&(format!("{}{route}{param_string}", API_ROOT)))
    .send()
    .await {
        Ok(response) => 
            match response.text().await {
                Ok(text) => Ok(text),
                Err(error) => Err(error)
            }
        ,
        Err(error) => Err(error)
    }
}

pub async fn get<T>(parameters: &HashMap<String, String>, route: String) -> Result<T, Error>
where T: for<'de> serde::de::Deserialize<'de>
{
    let mut param_string: String = "?".to_owned();
    let mut first: bool = true;
    for (param, value) in parameters {
        param_string = format!("{param_string}{}{param}={value}", if first { "" } else { "?" });
        first = false;
    }

    match Request::get(&(format!("{}{route}{param_string}", API_ROOT)))
    .send()
    .await {
        Ok(response) => 
            match response.json().await {
                Ok(obj) => Ok(obj),
                Err(error) => Err(error)
            }
        ,
        Err(error) => Err(error)
    }
}