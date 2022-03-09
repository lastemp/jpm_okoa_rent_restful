use reqwest::header::{ACCEPT, CONTENT_TYPE};
use reqwest::StatusCode;
use serde::{Deserialize};
use actix_web::{web};
use mysql::Pool;

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
//pub async fn send_sms_message(_message: String, _to: String, _from: String, user_name: String, api_key: String, api_url: String) -> std::result::Result<(), reqwest::Error> {
//pub async fn send_sms_message(_message: String, _to: String, _from: String, user_name: String, api_key: String, api_url: String) -> std::result::Result<reqwest::Response, reqwest::Error> {
//pub async fn send_sms_message(data: &web::Data<Pool>, _message: String, _to: String, _from: String, user_name: String, api_key: String, api_url: String) -> std::result::Result<reqwest::Response, reqwest::Error> {
pub async fn send_sms_message(data: web::Data<Pool>, _message: String, _to: String, _from: String, user_name: String, api_key: String, api_url: String) -> std::result::Result<(), reqwest::Error> {
	
	//let params = [("username", user_name), ("to", _to), ("message", _message), ("from", _from)];
	let params = [("username", user_name), ("to", _to.to_string()), ("message", _message.to_string())];
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
		
		match res {
			   Err(e) => {
						println!("server not responding");
					   },
			   Ok(response) => {
						match response.status() {
							StatusCode::CREATED => {
								//println!("Response status {}", response.status());
								let my_output = response
								.json::<ResultSendSmsMessage>()
								.await?;

								fetch_sms_message_result(data, _message, _to, _from, my_output);
							}
							s => println!("Received response status: {:?}", s),
						}
			   }, 
		};
		
		Ok(())
}

fn fetch_sms_message_result(data: web::Data<Pool>, sms_message: String, _to: String, _from: String, result_message: ResultSendSmsMessage) {
	let k = String::from(""); //Default value for string variables.i32
	let m: i32 = 0; //Default value for i32 variables.
	let _message = &result_message.SMSMessageData.Message.as_ref().unwrap_or(&k);
	let _recipients = &result_message.SMSMessageData.Recipients;
	
	//println!("fetch_sms_message_result: struct {:?}", result_message);
	
	let x = _recipients.len();
	
	if x > 0 {
		for _recipient in _recipients.iter() {
			let message_id = &_recipient.messageId.as_ref().unwrap_or(&k);
			let _number = &_recipient.number.as_ref().unwrap_or(&k);
			let status_code = _recipient.statusCode.as_ref().unwrap_or(&m);
			let _status = &_recipient.status.as_ref().unwrap_or(&k);
			let _cost = &_recipient.cost.as_ref().unwrap_or(&k);
			
			let status_code = *status_code;
			let sms_message_1: String = sms_message.to_string();
			let to_1: String = _to.to_string();
			let from_1: String = _from.to_string();
			
			crate::db_layer::create_outgoing_sms_message_data(&data, sms_message_1, to_1, from_1, _message.to_string(), message_id.to_string(), _number.to_string(), status_code, _status.to_string(), _cost.to_string());
			
		}
	}
}

/*
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
*/