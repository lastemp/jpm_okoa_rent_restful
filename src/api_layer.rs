//use std::error::Error;
//use reqwest::{Client, Method};
//use std::collections::HashMap;
//use actix_rt::System;
//use actix_web::client::{Client, Connector};
//use awc::Client;
//use openssl::ssl::{SslConnector, SslMethod};
//use reqwest::blocking::Client;
use reqwest::header::{ACCEPT, CONTENT_TYPE};
//use reqwest::Client;
use reqwest::StatusCode;
use serde::{Deserialize};
//use serde_json;

#[derive(Deserialize, Debug)]
struct Recipients {
    messageId: Option<String>,
	number: Option<String>,
	statusCode: Option<i32>,
	status: Option<String>,
	cost: Option<String>,
}

#[derive(Deserialize, Debug)]
struct SMSMessageData {
    Message: Option<String>,
	Recipients: Vec<Recipients>,
}

#[derive(Deserialize, Debug)]
struct ResultSendSmsMessage {
    Id: Option<String>,
	SMSMessageData: SMSMessageData,
}

// -> std::result::Result<reqwest::Response, reqwest::Error>
//std::result::Result<reqwest::blocking::Response, reqwest::Error>
pub async fn send_sms_message(_message: String, _to: String, _from: String, user_name: String, api_key: String, api_url: String) -> std::result::Result<(), reqwest::Error> {
	//let mut successful: bool = false;
	
	/*
	let mut params = HashMap::new();
	
	params.insert("username ", user_name);
	params.insert("to", _to);
	params.insert("message ", _message);
	params.insert("from", _from);

	let client = reqwest::Client::new();
	let res = client.post(&api_url)
		.header(CONTENT_TYPE, "application/x-www-form-urlencoded")
		.header(ACCEPT, "application/json")
		.header("apiKey", &api_key)
		.form(&params)
		.send()?;
	*/
	/*
	let params = [("username", user_name), ("to", _to), ("message", _message), ("from", _from)];

	let mut client = awc::Client::default();
	let response = client.post(api_url)
		.header("apiKey", api_key)
		.send_form(&params)
		.await?;
	*/
	// This will POST a body of `foo=bar&baz=quux`
	//let params = [("username", user_name), ("to", _to), ("message", _message), ("from", _from)];
	let params = [("username", user_name), ("to", _to), ("message", _message)];
	let client = reqwest::Client::new();
	//let client = Client::new();
	let res = client.post(api_url)
		.header(CONTENT_TYPE, "application/x-www-form-urlencoded")
		.header(ACCEPT, "application/json")
		.header("apiKey", api_key)
		.form(&params)
		.send()
		//.await?; //The "?" after the await returns errors immediately and hence will not be captured on match clause below
		.await;
		
		/*
		println!("Response status {}", res.status());
		
		if res.status() == StatusCode::CREATED {
			// Parse the response body as Json in this case
			let my_output = res
				.json::<ResultSendSmsMessage>()
				.await?;

			println!("Response struct {:?}", my_output);
		}
		*/
	
		/*
		match res.status() {
			StatusCode::CREATED => {
				println!("Response status {}", res.status());
				let my_output = res
				.json::<ResultSendSmsMessage>()
				.await?;

				println!("Response struct {:?}", my_output);
			}
			s => println!("Received response status: {:?}", s),
		};
		*/
		
		match res {
			   Err(e) => {
						println!("server not responding");
					   },
			   Ok(response) => {
						match response.status() {
							StatusCode::CREATED => {
								println!("Response status {}", response.status());
								let my_output = response
								.json::<ResultSendSmsMessage>()
								.await?;

								fetch_sms_message_result(my_output);
							}
							s => println!("Received response status: {:?}", s),
						}
			   }, 
		};
		
		Ok(())
}

fn fetch_sms_message_result(result_message: ResultSendSmsMessage) {
	let k = String::from(""); //Default value for string variables.i32
	let m: i32 = 0; //Default value for i32 variables.
	let _message = &result_message.SMSMessageData.Message.as_ref().unwrap_or(&k);
	let _recipients = &result_message.SMSMessageData.Recipients;
	
	println!("fetch_sms_message_result: struct {:?}", result_message);
	
	let x = _recipients.len();
	
	if x > 0 {
		for _recipient in _recipients.iter() {
			let _message_id = &_recipient.messageId.as_ref().unwrap_or(&k);
			let _number = &_recipient.number.as_ref().unwrap_or(&k);
			let status_code = &_recipient.statusCode.as_ref().unwrap_or(&m);
			let _status = &_recipient.status.as_ref().unwrap_or(&k);
			let _cost = &_recipient.cost.as_ref().unwrap_or(&k);
			
			/*
			println!("message_id {:?}", _message_id);
			println!("number {:?}", _number);
			println!("status_code {:?}", status_code);
			println!("status {:?}", _status);
			println!("cost {:?}", _cost);
			*/
		}
	}

}

pub fn send_sms_message_sync(_message: String, _to: String, _from: String, user_name: String, api_key: String, api_url: String) -> std::result::Result<(), reqwest::Error> {
		
	let params = [("username", user_name), ("to", _to), ("message", _message)];
	let client = reqwest::blocking::Client::new();
	let res = client.post(api_url)
		.header(CONTENT_TYPE, "application/x-www-form-urlencoded")
		.header(ACCEPT, "application/json")
		.header("apiKey", api_key)
		.form(&params)
		.send()?;
		/*
		match res.status() {
			StatusCode::CREATED => {
				println!("Response status {}", res.status());
				let my_output = res
				.json::<ResultSendSmsMessage>();
				//.await?;

				println!("Response struct {:?}", my_output);
			}
			s => println!("Received response status: {:?}", s),
		};
		*/
		println!("Response status {}", res.status());
		
		Ok(())
}