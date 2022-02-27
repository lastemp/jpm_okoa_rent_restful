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
		//.send();
		.send()
		.await?;
		
		println!("Response status {}", res.status());
		
		if res.status() == StatusCode::CREATED {
			// Parse the response body as Json in this case
			let my_output = res
				.json::<ResultSendSmsMessage>()
				.await?;

			println!("Response struct {:?}", my_output);
		}
		
		/*
		//StatusCode::OK => println!("success!"),
		match res.status() {
			StatusCode::OK => {
				//println!("success: {:?}", res)
				let p: ResultSendSmsMessage = res.unwrap();
				println!("The name is {}", p);
			},
			StatusCode::PAYLOAD_TOO_LARGE => {
				println!("Request payload is too large!");
			}
			s => println!("Received response status: {:?}", s),
		};
		*/
		
		Ok(())
	/*	
	System::new().block_on(async {
		//let builder = SslConnector::builder(SslMethod::tls()).unwrap();
		let params = [("username", user_name), ("to", _to), ("message", _message), ("from", _from)];
        //let client = Client::default();
		
		//let client = Client::build()
			//.connector(Connector::new().ssl(builder.build()).finish())
			//.finish();
		
		let client = awc::Client::build()
			.connector(awc::Connector::new().ssl(builder.build()).finish())
			.finish();
        let res = client
            .post(api_url)                      // <- Create request builder
			.header("Accept", "application/json")
            .header("apiKey", api_key)
            .send_form(&params)                 // <- Send http request
            .await;

        println!("Response: {:?}", res);        // <- server http response
    });	
	*/
}