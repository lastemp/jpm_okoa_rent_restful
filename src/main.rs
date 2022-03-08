extern crate base64;

mod db_layer;
mod api_layer;

use actix_web::{post, web, App, HttpServer, Responder};
use serde::{Deserialize};//Serialize,
//use base64::{encode, decode};
use std::str;
use mysql::*;
//use mysql::prelude::*;

#[derive(Deserialize)]
struct InputData {
    sessionId: Option<String>,
	phoneNumber: Option<String>,
	networkCode: Option<String>,
	serviceCode: Option<String>,
	text: Option<String>,
}

async fn greet() -> impl Responder {
	format!("")
}

// request is of content type `application/x-www-form-urlencoded`
// request payload is deserialized into `InputData` struct from the URL encoded format
#[post("/processussdactions")]
async fn process_ussd_actions(form: web::Form<InputData>, data: web::Data<Pool>) -> String {
	let k = String::from(""); //Default value for string variables.
	let session_id = &form.sessionId.as_ref().unwrap_or(&k);
	let phone_number = &form.phoneNumber.as_ref().unwrap_or(&k);
	//let network_code = &form.networkCode.as_ref().unwrap_or(&k);
	//let service_code = &form.serviceCode.as_ref().unwrap_or(&k);
	let text = &form.text.as_ref().unwrap_or(&k);
	
	let session_id = session_id.replace(" ","");
	let phone_number = phone_number.replace(" ","");
	
	if session_id.len() == 0 || phone_number.len() == 0 {
		let response_data = "Please note that mandatory data was not supplied.";
		let response_data = generate_ussd_response_message(&response_data.to_string(), false);
		
		return response_data.to_string()
		
	}
	
	let phone_number = phone_number.replace("+","");
	
	let is_registered = db_layer::get_ussd_registered_client(&data, &phone_number);
	
	//TESTS ONLY
	/*
	let _message: String = String::from("Good luck on your plans");
	let _to: String = String::from("+2547xxxxxxxx");
	let _from: String = String::from("AFRICASTKNG"); 
	let user_name: String = String::from("lastemperor"); 
	let api_key: String = String::from("be27db49f6d6ff9ffeff2c3729d728d90d0f1d7573a2c16f7f1d27b9024174fa");
	let api_url: String = String::from("https://api.africastalking.com/version1/messaging");
	
	let _p = api_layer::send_sms_message(_message, _to, _from, user_name, api_key, api_url).await;
	match _p
	{
        Ok(x) => println!("send_sms_message status - successful. {:?}", x),
        Err(e) => println!("send_sms_message status. {:?}", e),
    }
	*/
	if is_registered {
		let response_data = process_client_requests(&data, &session_id, &phone_number, text);
		
		return response_data
	}
	else {
		let response_data = process_unregistered_client_requests(&data, &session_id, &phone_number, text);
		
		return response_data
	}
}

fn get_menu_data() -> String {
	let mut menu_data = String::from("");
	let welcome_message_1 = String::from("Welcome to Okoa Rent\\Mortgage Service, ");
	let welcome_message_2 = String::from("a Real Estate Industry Revolution.");
	let menu_1 = String::from("1. Okoa Rent");
	let menu_2 = String::from("2. Okoa Mortgage");
	let menu_3 = String::from("3. Check Balance");
	let menu_4 = String::from("4. Pay Back");
	let menu_5 = String::from("5. Get Statement");
	
	menu_data.push_str(&welcome_message_1);
	menu_data.push_str(&welcome_message_2);
	menu_data.push_str("\n");
		
	menu_data.push_str(&menu_1);
	menu_data.push_str("\n");
	menu_data.push_str(&menu_2);
	menu_data.push_str("\n");
	menu_data.push_str(&menu_3);
	menu_data.push_str("\n");
	menu_data.push_str(&menu_4);
	menu_data.push_str("\n");
	menu_data.push_str(&menu_5);
	
	menu_data
}

fn get_menu_data_unregistered_client() -> String {
	let mut menu_data = String::from("");
	let welcome_message_1 = String::from("Welcome to Okoa Rent\\Mortgage Service, ");
	let welcome_message_2 = String::from("a Real Estate Industry Revolution.");
	let menu_1 = String::from("1. Self Register");
		
	menu_data.push_str(&welcome_message_1);
	menu_data.push_str(&welcome_message_2);
	menu_data.push_str("\n");
	menu_data.push_str(&menu_1);
	
	menu_data
}

fn get_menu_1_sub_menu_data(data: &web::Data<Pool>, mobile_no: &String, text: &String) -> String {
		
	if text.replace(" ","").len() == 0 {
		let response_data = "Please note that you made a wrong selection.";
		let response_data = generate_ussd_response_message(&response_data.to_string(), false);
		
		return response_data.to_string()
		
	}
	else if !text.contains("*") {
		let mut sub_menu_data = String::from("");
		let sub_menu_1 = String::from("Enter Tenant Code");
		//let sub_menu_2 = String::from("Enter 00 to go to previous Menu");
		let sub_menu_2 = String::from("0:Back 00:Home");
		
		sub_menu_data.push_str(&sub_menu_1);
		sub_menu_data.push_str("\n");
		sub_menu_data.push_str(&sub_menu_2);
		
		sub_menu_data = generate_ussd_response_message(&sub_menu_data, true);

		return sub_menu_data.to_string()
	}
	else {
		let v: Vec<&str> = text.split('*').collect();
		
		//println!("get_menu_1_sub_menu_data v - {:?}", v);
		
		let vector_length = v.len();
		
		if vector_length == 0 {
			let response_data = "Please note that you made a wrong selection.";
			let response_data = generate_ussd_response_message(&response_data.to_string(), false);

			return response_data.to_string()
		}
		
		let wrong_selection_data = "Please note that you made a wrong selection.";
		
		let response_data = 
			match vector_length {
				2 => //Index 1
					{
						let tenant_code = String::from(v[1]);
						
						if tenant_code.replace(" ","").len() == 0 {
							let response_data = "Please note that you did not enter tenant code.";
							let response_data = generate_ussd_response_message(&response_data.to_string(), false);
			
							return response_data.to_string()
						}
						
						let mut sub_menu_data = String::from("");
						let sub_menu_1 = String::from("Enter House Code");
						//let sub_menu_2 = String::from("Enter 00 to go to previous Menu");
						let sub_menu_2 = String::from("0:Back 00:Home");
						
						sub_menu_data.push_str(&sub_menu_1);
						sub_menu_data.push_str("\n");
						sub_menu_data.push_str(&sub_menu_2);
						
						sub_menu_data = generate_ussd_response_message(&sub_menu_data, true);

						return sub_menu_data.to_string()
					},
				3 => //Index 2
					{
						let house_code = String::from(v[2]);
						
						if house_code.replace(" ","").len() == 0 {
							let response_data = "Please note that you did not enter house code.";
							let response_data = generate_ussd_response_message(&response_data.to_string(), false);
			
							return response_data.to_string()
						}
						
						let mut sub_menu_data = String::from("");
						let sub_menu_1 = String::from("Enter Amount");
						//let sub_menu_2 = String::from("Enter 00 to go to previous Menu");
						let sub_menu_2 = String::from("0:Back 00:Home");
						
						sub_menu_data.push_str(&sub_menu_1);
						sub_menu_data.push_str("\n");
						sub_menu_data.push_str(&sub_menu_2);
						
						sub_menu_data = generate_ussd_response_message(&sub_menu_data, true);

						return sub_menu_data.to_string()
					},
					/*	
			    4 => //Index 3
					{
						let amount = String::from(v[3]);
						
						if amount.replace(" ","").len() == 0 {
							let response_data = "Please note that you did not enter amount.";
			
							return response_data.to_string()
						}
						
						let mut sub_menu_data = String::from("");
						let sub_menu_1 = String::from("Enter PIN");
						
						sub_menu_data.push_str(&sub_menu_1);

						return sub_menu_data.to_string()
					},
					*/	
				4 => //Index 3
					{
						let amount = String::from(v[3]);
						
						if amount.replace(" ","").len() == 0 {
							let response_data = "Please note that you did not enter amount.";
							let response_data = generate_ussd_response_message(&response_data.to_string(), false);
			
							return response_data.to_string()
						}
						
						let is_valid = validate_numeric(&amount);//validate_minimum_amount
						
						if !is_valid {							
							let mut sub_menu_data = String::from("");
							let sub_menu_1 = String::from("Please note that you entered invalid amount.");
							let sub_menu_2 = String::from("0:Back 00:Home");
							
							sub_menu_data.push_str(&sub_menu_1);
							sub_menu_data.push_str("\n");
							sub_menu_data.push_str(&sub_menu_2);
							
							sub_menu_data = generate_ussd_response_message(&sub_menu_data, true);

							return sub_menu_data.to_string()
						}
						
						let is_valid = validate_minimum_amount(&amount);
						
						if !is_valid {					
							let mut sub_menu_data = String::from("");
							let sub_menu_1 = String::from("Please note that the minimum amount is Ksh 100.");
							let sub_menu_2 = String::from("0:Back 00:Home");
							
							sub_menu_data.push_str(&sub_menu_1);
							sub_menu_data.push_str("\n");
							sub_menu_data.push_str(&sub_menu_2);
							
							sub_menu_data = generate_ussd_response_message(&sub_menu_data, true);

							return sub_menu_data.to_string()
						}
						
						let is_valid = validate_maximum_amount(&amount);
						
						if !is_valid {
							let mut sub_menu_data = String::from("");
							let sub_menu_1 = String::from("Please note that the maximum amount is Ksh 15,000.");
							let sub_menu_2 = String::from("0:Back 00:Home");
							
							sub_menu_data.push_str(&sub_menu_1);
							sub_menu_data.push_str("\n");
							sub_menu_data.push_str(&sub_menu_2);
							
							sub_menu_data = generate_ussd_response_message(&sub_menu_data, true);

							return sub_menu_data.to_string()
						}
						
						let tenant_code = String::from(v[1]);
						let house_code = String::from(v[2]);
						
						let mut sub_menu_data = String::from("");
						let sub_menu_1 = String::from("Please confirm payment details.");
						let sub_menu_2 = String::from("1. Cancel");
						let sub_menu_3 = String::from("2. Send");
						
						sub_menu_data.push_str(&sub_menu_1);
						sub_menu_data.push_str("\n");
						sub_menu_data.push_str(&String::from("Okoa Rent for tenant code "));
						sub_menu_data.push_str(&tenant_code);
						sub_menu_data.push_str(&String::from(" and house code "));
						sub_menu_data.push_str(&house_code);
						sub_menu_data.push_str(&String::from(","));
						sub_menu_data.push_str("\n");
						sub_menu_data.push_str(&String::from("for amount of Ksh "));
						sub_menu_data.push_str(&amount);
						sub_menu_data.push_str(&String::from("."));
						sub_menu_data.push_str("\n");
						sub_menu_data.push_str(&sub_menu_2);
						sub_menu_data.push_str("\n");
						sub_menu_data.push_str(&sub_menu_3);
						
						sub_menu_data = generate_ussd_response_message(&sub_menu_data, true);

						return sub_menu_data.to_string()
					},	
				5 => //Index 4
					{	
						/*
						let pin = String::from(v[4]);
						
						if pin.replace(" ","").len() == 0 {
							let response_data = "Please note that you did not enter PIN.";
			
							return response_data.to_string()
						}
						
						let mut sub_menu_data = String::from("");
						let sub_menu_1 = String::from("1. Cancel");
						let sub_menu_2 = String::from("2. Send");
						
						sub_menu_data.push_str(&sub_menu_1);
						sub_menu_data.push_str("\n");
						sub_menu_data.push_str(&sub_menu_2);

						return sub_menu_data.to_string()
						*/
						let input = String::from(v[4]);
						
						if input.replace(" ","").len() == 0 {
							let response_data = wrong_selection_data.to_string();
							let response_data = generate_ussd_response_message(&response_data, false);
			
							return response_data.to_string()
						}
						
						if input.eq("1") {
							let mut sub_menu_data = String::from("Please note that you you cancelled the request.");
							sub_menu_data = generate_ussd_response_message(&sub_menu_data, false);
							
							return sub_menu_data.to_string()
						}
						else if input.eq("2") {
							let mut sub_menu_data = String::from("");
							let sub_menu_1 = String::from("Enter PIN");
							
							sub_menu_data.push_str(&sub_menu_1);
							
							sub_menu_data = generate_ussd_response_message(&sub_menu_data, true);

							return sub_menu_data.to_string()
						}
						else {
							let mut sub_menu_data = String::from("");
							let sub_menu_1 = wrong_selection_data.to_string();
							let sub_menu_2 = String::from("0:Back 00:Home");
							
							sub_menu_data.push_str(&sub_menu_1);
							sub_menu_data.push_str("\n");
							sub_menu_data.push_str(&sub_menu_2);
							
							sub_menu_data = generate_ussd_response_message(&sub_menu_data, true);

							return sub_menu_data.to_string()
						}
						
					},
				6 => //Index 5
					{
						let pin = String::from(v[5]);
						
						if pin.replace(" ","").len() == 0 {
							let response_data = "Please note that you did not enter PIN.";
							let response_data = generate_ussd_response_message(&response_data.to_string(), false);
			
							return response_data.to_string()
						}
						
						let mut sub_menu_data = String::from("");
						
						if pin.eq("1234") {
							//sub_menu_data = String::from("Dear Customer, please note that you will get a prompt to enter Mpesa Pin to complete the transaction.");
							let sub_menu_1 = String::from("Dear Customer, we acknowledge receipt of your Okoa Rent submission.");
							let sub_menu_2 = String::from("It is being processed and you will be notified once complete.");
							let sub_menu_3 = String::from("Thank you.");
							
							let tenant_code = String::from(v[1]);
							let house_code = String::from(v[2]);
							let amount_s = String::from(v[3]);
							
							let amount = 
								match amount_s.parse::<i32>() {
								  Ok(a) => a,
								  Err(e) => 0,
							};
							
							db_layer::create_okoa_rent_data(data, tenant_code, house_code, amount, mobile_no.to_string());
							
							sub_menu_data.push_str(&sub_menu_1);
							sub_menu_data.push_str("\n");
							sub_menu_data.push_str(&sub_menu_2);
							sub_menu_data.push_str("\n");
							sub_menu_data.push_str(&sub_menu_3);
							
							sub_menu_data = generate_ussd_response_message(&sub_menu_data, false);
							
							return sub_menu_data.to_string()
						}
						else {
							sub_menu_data = String::from("Please note that you entered an invalid PIN.");
							sub_menu_data = generate_ussd_response_message(&sub_menu_data, false);
							
							return sub_menu_data.to_string()
						}

					},	
					/*	
			    6 => //Index 5
					{
						let input = String::from(v[5]);
						
						if input.replace(" ","").len() == 0 {
							let response_data = wrong_selection_data.to_string();
			
							return response_data.to_string()
						}
						
						let mut sub_menu_data = String::from("");
						
						if input.eq("1") {
							sub_menu_data = String::from("Please note that you you cancelled the request.");
						}
						else if input.eq("2") {
							sub_menu_data = String::from("Dear Customer, please note that you will get a prompt to enter Mpesa Pin to complete the transaction.");
						}
						else {
							sub_menu_data = wrong_selection_data.to_string();
						}

						return sub_menu_data.to_string()
					},
					*/	
				_ => {
					let response_data = wrong_selection_data.to_string();
					let response_data = generate_ussd_response_message(&response_data, false);
			
					return response_data.to_string()
					},
        };
		
		return response_data
	}
}

fn get_menu_2_sub_menu_data(data: &web::Data<Pool>, mobile_no: &String, text: &String) -> String {
		
	if text.replace(" ","").len() == 0 {
		let response_data = "Please note that you made a wrong selection.";
		let response_data = generate_ussd_response_message(&response_data.to_string(), false);
		
		return response_data.to_string()
		
	}
	else if !text.contains("*") {
		let mut sub_menu_data = String::from("");
		let sub_menu_1 = String::from("Enter Mortgagor Code");
		//let sub_menu_2 = String::from("Enter 00 to go to previous Menu");
		let sub_menu_2 = String::from("0:Back 00:Home");
		
		sub_menu_data.push_str(&sub_menu_1);
		sub_menu_data.push_str("\n");
		sub_menu_data.push_str(&sub_menu_2);
		
		sub_menu_data = generate_ussd_response_message(&sub_menu_data, true);

		return sub_menu_data.to_string()
	}
	else {
		let v: Vec<&str> = text.split('*').collect();
		
		//println!("get_menu_1_sub_menu_data v - {:?}", v);
		
		let vector_length = v.len();
		
		if vector_length == 0 {
			let response_data = "Please note that you made a wrong selection.";
			let response_data = generate_ussd_response_message(&response_data.to_string(), false);

			return response_data.to_string()
		}
		
		let wrong_selection_data = "Please note that you made a wrong selection.";
		
		let response_data = 
			match vector_length {
				2 => //Index 1
					{
						let mortgagor_code = String::from(v[1]);
						
						if mortgagor_code.replace(" ","").len() == 0 {
							let response_data = "Please note that you did not enter mortgagor code.";
							let response_data = generate_ussd_response_message(&response_data.to_string(), false);
			
							return response_data.to_string()
						}
						
						let mut sub_menu_data = String::from("");
						let sub_menu_1 = String::from("Enter House Code");
						//let sub_menu_2 = String::from("Enter 00 to go to previous Menu");
						let sub_menu_2 = String::from("0:Back 00:Home");
						
						sub_menu_data.push_str(&sub_menu_1);
						sub_menu_data.push_str("\n");
						sub_menu_data.push_str(&sub_menu_2);
						
						sub_menu_data = generate_ussd_response_message(&sub_menu_data, true);

						return sub_menu_data.to_string()
					},
				3 => //Index 2
					{
						let house_code = String::from(v[2]);
						
						if house_code.replace(" ","").len() == 0 {
							let response_data = "Please note that you did not enter house code.";
							let response_data = generate_ussd_response_message(&response_data.to_string(), false);
			
							return response_data.to_string()
						}
						
						let mut sub_menu_data = String::from("");
						let sub_menu_1 = String::from("Enter Amount");
						//let sub_menu_2 = String::from("Enter 00 to go to previous Menu");
						let sub_menu_2 = String::from("0:Back 00:Home");
						
						sub_menu_data.push_str(&sub_menu_1);
						sub_menu_data.push_str("\n");
						sub_menu_data.push_str(&sub_menu_2);
						
						sub_menu_data = generate_ussd_response_message(&sub_menu_data, true);

						return sub_menu_data.to_string()
					},
				4 => //Index 3
					{
						let amount = String::from(v[3]);
						
						if amount.replace(" ","").len() == 0 {
							let response_data = "Please note that you did not enter amount.";
							let response_data = generate_ussd_response_message(&response_data.to_string(), false);
			
							return response_data.to_string()
						}
						
						let is_valid = validate_numeric(&amount);//validate_minimum_amount
						
						if !is_valid {
							let mut sub_menu_data = String::from("");
							let sub_menu_1 = String::from("Please note that you entered invalid amount.");
							let sub_menu_2 = String::from("0:Back 00:Home");
							
							sub_menu_data.push_str(&sub_menu_1);
							sub_menu_data.push_str("\n");
							sub_menu_data.push_str(&sub_menu_2);
							
							sub_menu_data = generate_ussd_response_message(&sub_menu_data, true);

							return sub_menu_data.to_string()
						}
						
						let is_valid = validate_minimum_amount(&amount);
						
						if !is_valid {
							let mut sub_menu_data = String::from("");
							let sub_menu_1 = String::from("Please note that the minimum amount is Ksh 100.");
							let sub_menu_2 = String::from("0:Back 00:Home");
							
							sub_menu_data.push_str(&sub_menu_1);
							sub_menu_data.push_str("\n");
							sub_menu_data.push_str(&sub_menu_2);
							
							sub_menu_data = generate_ussd_response_message(&sub_menu_data, true);

							return sub_menu_data.to_string()
						}
						
						let is_valid = validate_maximum_amount(&amount);
						
						if !is_valid {
							let mut sub_menu_data = String::from("");
							let sub_menu_1 = String::from("Please note that the maximum amount is Ksh 15,000.");
							let sub_menu_2 = String::from("0:Back 00:Home");
							
							sub_menu_data.push_str(&sub_menu_1);
							sub_menu_data.push_str("\n");
							sub_menu_data.push_str(&sub_menu_2);
							
							sub_menu_data = generate_ussd_response_message(&sub_menu_data, true);

							return sub_menu_data.to_string()
						}
						
						let mortgagor_code = String::from(v[1]);
						let house_code = String::from(v[2]);
						
						let mut sub_menu_data = String::from("");
						let sub_menu_1 = String::from("Please confirm payment details.");
						let sub_menu_2 = String::from("1. Cancel");
						let sub_menu_3 = String::from("2. Send");
						
						sub_menu_data.push_str(&sub_menu_1);
						sub_menu_data.push_str("\n");
						sub_menu_data.push_str(&String::from("Okoa Mortgage for mortgagor code "));
						sub_menu_data.push_str(&mortgagor_code);
						sub_menu_data.push_str(&String::from(" and house code "));
						sub_menu_data.push_str(&house_code);
						sub_menu_data.push_str(&String::from(","));
						sub_menu_data.push_str("\n");
						sub_menu_data.push_str(&String::from("for amount of Ksh "));
						sub_menu_data.push_str(&amount);
						sub_menu_data.push_str(&String::from("."));
						sub_menu_data.push_str("\n");
						sub_menu_data.push_str(&sub_menu_2);
						sub_menu_data.push_str("\n");
						sub_menu_data.push_str(&sub_menu_3);
						
						sub_menu_data = generate_ussd_response_message(&sub_menu_data, true);

						return sub_menu_data.to_string()
					},	
				5 => //Index 4
					{	
						let input = String::from(v[4]);
						
						if input.replace(" ","").len() == 0 {
							let response_data = wrong_selection_data.to_string();
							let response_data = generate_ussd_response_message(&response_data, false);
			
							return response_data.to_string()
						}
						
						if input.eq("1") {
							let mut sub_menu_data = String::from("Please note that you you cancelled the request.");
							sub_menu_data = generate_ussd_response_message(&sub_menu_data, false);
							
							return sub_menu_data.to_string()
						}
						else if input.eq("2") {
							let mut sub_menu_data = String::from("");
							let sub_menu_1 = String::from("Enter PIN");
							
							sub_menu_data.push_str(&sub_menu_1);
							
							sub_menu_data = generate_ussd_response_message(&sub_menu_data, true);

							return sub_menu_data.to_string()
						}
						else {
							let mut sub_menu_data = String::from("");
							let sub_menu_1 = wrong_selection_data.to_string();
							let sub_menu_2 = String::from("0:Back 00:Home");
							
							sub_menu_data.push_str(&sub_menu_1);
							sub_menu_data.push_str("\n");
							sub_menu_data.push_str(&sub_menu_2);
							
							sub_menu_data = generate_ussd_response_message(&sub_menu_data, true);

							return sub_menu_data.to_string()
						}
						
					},
				6 => //Index 5
					{
						let pin = String::from(v[5]);
						
						if pin.replace(" ","").len() == 0 {
							let response_data = "Please note that you did not enter PIN.";
							let response_data = generate_ussd_response_message(&response_data.to_string(), false);
			
							return response_data.to_string()
						}
						
						let mut sub_menu_data = String::from("");
						
						if pin.eq("1234") {
							//sub_menu_data = String::from("Dear Customer, please note that you will get a prompt to enter Mpesa Pin to complete the transaction.");
							let sub_menu_1 = String::from("Dear Customer, we acknowledge receipt of your Okoa Mortgage submission.");
							let sub_menu_2 = String::from("It is being processed and you will be notified once complete.");
							let sub_menu_3 = String::from("Thank you.");
							
							let mortgagor_code = String::from(v[1]);
							let house_code = String::from(v[2]);
							let amount_s = String::from(v[3]);
							
							let amount = 
								match amount_s.parse::<i32>() {
								  Ok(a) => a,
								  Err(e) => 0,
							};
							
							db_layer::create_okoa_mortgage_data(data, mortgagor_code, house_code, amount, mobile_no.to_string());
							
							sub_menu_data.push_str(&sub_menu_1);
							sub_menu_data.push_str("\n");
							sub_menu_data.push_str(&sub_menu_2);
							sub_menu_data.push_str("\n");
							sub_menu_data.push_str(&sub_menu_3);
							
							sub_menu_data = generate_ussd_response_message(&sub_menu_data, false);
							
							return sub_menu_data.to_string()
						}
						else {
							sub_menu_data = String::from("Please note that you entered an invalid PIN.");
							sub_menu_data = generate_ussd_response_message(&sub_menu_data, false);
							
							return sub_menu_data.to_string()
						}

					},
				_ => {
						let response_data = wrong_selection_data.to_string();
						let response_data = generate_ussd_response_message(&response_data, false);
		
						return response_data.to_string()
					},
        };
		
		return response_data
	}
}

fn get_menu_3_sub_menu_data(data: &web::Data<Pool>, mobile_no: &String, text: &String) -> String {
		
	if text.replace(" ","").len() == 0 {
		let response_data = "Please note that you made a wrong selection.";
		let response_data = generate_ussd_response_message(&response_data.to_string(), false);
		
		return response_data.to_string()
		
	}
	else if !text.contains("*") {
		let mut sub_menu_data = String::from("");
		let sub_menu_1 = String::from("Enter the OKOA RENT\\MORTGAGE Number");
		//let sub_menu_2 = String::from("Enter 00 to go to previous Menu");
		let sub_menu_2 = String::from("0:Back 00:Home");
		
		sub_menu_data.push_str(&sub_menu_1);
		sub_menu_data.push_str("\n");
		sub_menu_data.push_str(&sub_menu_2);
		
		sub_menu_data = generate_ussd_response_message(&sub_menu_data, true);

		return sub_menu_data.to_string()
	}
	else {
		let v: Vec<&str> = text.split('*').collect();
		
		//println!("get_menu_1_sub_menu_data v - {:?}", v);
		
		let vector_length = v.len();
		
		if vector_length == 0 {
			let response_data = "Please note that you made a wrong selection.";
			let response_data = generate_ussd_response_message(&response_data.to_string(), false);

			return response_data.to_string()
		}
		
		let wrong_selection_data = "Please note that you made a wrong selection.";
		
		let response_data = 
			match vector_length {
				2 => //Index 1
					{
						let rent_mortgage_code = String::from(v[1]);
						
						if rent_mortgage_code.replace(" ","").len() == 0 {
							let response_data = "Please note that you did not enter Okoa Rent\\Mortgage number.";
							let response_data = generate_ussd_response_message(&response_data.to_string(), false);
			
							return response_data.to_string()
						}
						
						let mut sub_menu_data = String::from("");
						let sub_menu_1 = String::from("Please confirm request details.");
						let sub_menu_2 = String::from("1. Cancel");
						let sub_menu_3 = String::from("2. Send");
						
						sub_menu_data.push_str(&sub_menu_1);
						sub_menu_data.push_str("\n");
						sub_menu_data.push_str(&String::from("Check Balance for Okoa Rent\\Mortgage number "));
						sub_menu_data.push_str(&rent_mortgage_code);
						sub_menu_data.push_str(&String::from("."));
						sub_menu_data.push_str("\n");
						sub_menu_data.push_str(&sub_menu_2);
						sub_menu_data.push_str("\n");
						sub_menu_data.push_str(&sub_menu_3);
						
						sub_menu_data = generate_ussd_response_message(&sub_menu_data, true);

						return sub_menu_data.to_string()
					},
					
				3 => //Index 2
					{	
						let input = String::from(v[2]);
						
						if input.replace(" ","").len() == 0 {
							let response_data = wrong_selection_data.to_string();
							let response_data = generate_ussd_response_message(&response_data, false);
			
							return response_data.to_string()
						}
						
						if input.eq("1") {
							let mut sub_menu_data = String::from("Please note that you you cancelled the request.");
							sub_menu_data = generate_ussd_response_message(&sub_menu_data, false);
							
							return sub_menu_data.to_string()
						}
						else if input.eq("2") {
							let mut sub_menu_data = String::from("");
							let sub_menu_1 = String::from("Enter PIN");
							
							sub_menu_data.push_str(&sub_menu_1);
							
							sub_menu_data = generate_ussd_response_message(&sub_menu_data, true);

							return sub_menu_data.to_string()
						}
						else {
							let mut sub_menu_data = String::from("");
							let sub_menu_1 = wrong_selection_data.to_string();
							let sub_menu_2 = String::from("0:Back 00:Home");
							
							sub_menu_data.push_str(&sub_menu_1);
							sub_menu_data.push_str("\n");
							sub_menu_data.push_str(&sub_menu_2);
							
							sub_menu_data = generate_ussd_response_message(&sub_menu_data, true);

							return sub_menu_data.to_string()
						}
						
					},
				4 => //Index 3
					{
						let pin = String::from(v[3]);
						
						if pin.replace(" ","").len() == 0 {
							let response_data = "Please note that you did not enter PIN.";
							let response_data = generate_ussd_response_message(&response_data.to_string(), false);
			
							return response_data.to_string()
						}
						
						let mut sub_menu_data = String::from("");
						
						if pin.eq("1234") {
							let sub_menu_1 = String::from("Dear Customer, your wallet account balance is Ksh 13,650.00. ");
							let sub_menu_2 = String::from("Transaction fee is Ksh 1.00. ");
							let sub_menu_3 = String::from("THANK YOU FOR USING OKOA RENT\\MORTGAGE. ");
							let sub_menu_4 = String::from("You can now access OKOA RENT\\MORTGAGE via App.");
							//let sub_menu_5 = String::from("Enter 00 to go to previous Menu");
							let sub_menu_5 = String::from("00:Home");
							
							let rent_mortgage_code = String::from(v[1]);
							
							db_layer::create_check_balance_data(data, rent_mortgage_code, mobile_no.to_string());
							
							//TESTS ONLY
							///*
							//let _message: String = String::from("Good luck on your plans");
							let mut _message = String::from("");
							let mut _to: String = String::from("");
							let _from: String = String::from("AFRICASTKNG");
							let user_name: String = String::from("lastemperor"); 
							let api_key: String = String::from("be27db49f6d6ff9ffeff2c3729d728d90d0f1d7573a2c16f7f1d27b9024174fa");
							let api_url: String = String::from("https://api.africastalking.com/version1/messaging");
							
							let msg_1 = String::from("Dear Customer, your wallet account balance is Ksh 13,650.00. ");
							let msg_2 = String::from("Transaction fee is Ksh 1.00. ");
							let msg_3 = String::from("THANK YOU FOR USING OKOA RENT\\MORTGAGE. ");
							let msg_4 = String::from("You can now access OKOA RENT\\MORTGAGE via App.");
							
							_message.push_str(&msg_1);
							_message.push_str(&msg_2);
							_message.push_str(&msg_3);
							_message.push_str(&msg_4);
							
							if !mobile_no.contains("+") {
								_to.push_str("+");
								_to.push_str(&mobile_no);
							}
							
							tokio::spawn(async move {
								// Process each request concurrently.
								let _p = api_layer::send_sms_message(_message, _to, _from, user_name, api_key, api_url).await;
							});
							
							/*
							let _p = api_layer::send_sms_message(_message, _to, _from, user_name, api_key, api_url).await;
							match _p
							{
								Ok(x) => println!("send_sms_message status - successful. {:?}", x),
								Err(e) => println!("send_sms_message status. {:?}", e),
							}
							*/
							//*/
							
							sub_menu_data.push_str(&sub_menu_1);
							sub_menu_data.push_str(&sub_menu_2);
							sub_menu_data.push_str(&sub_menu_3);
							sub_menu_data.push_str(&sub_menu_4);
							sub_menu_data.push_str("\n");
							sub_menu_data.push_str(&sub_menu_5);
							
							sub_menu_data = generate_ussd_response_message(&sub_menu_data, true);
							
							return sub_menu_data.to_string()
						}
						else {
							sub_menu_data = String::from("Please note that you entered an invalid PIN.");
							sub_menu_data = generate_ussd_response_message(&sub_menu_data, false);
							
							return sub_menu_data.to_string()
						}

					},
				_ => {
						let response_data = wrong_selection_data.to_string();
						let response_data = generate_ussd_response_message(&response_data, false);
		
						return response_data.to_string()
					},
        };
		
		return response_data
	}
}

fn get_menu_4_sub_menu_data(data: &web::Data<Pool>, mobile_no: &String, text: &String) -> String {
		
	if text.replace(" ","").len() == 0 {
		let response_data = "Please note that you made a wrong selection.";
		let response_data = generate_ussd_response_message(&response_data.to_string(), false);
		
		return response_data.to_string()
		
	}
	else if !text.contains("*") {
		let mut sub_menu_data = String::from("");
		let sub_menu_1 = String::from("Enter the OKOA RENT\\MORTGAGE Number");
		//let sub_menu_2 = String::from("Enter 00 to go to previous Menu");
		let sub_menu_2 = String::from("0:Back 00:Home");
		
		sub_menu_data.push_str(&sub_menu_1);
		sub_menu_data.push_str("\n");
		sub_menu_data.push_str(&sub_menu_2);
		
		sub_menu_data = generate_ussd_response_message(&sub_menu_data, true);

		return sub_menu_data.to_string()
	}
	else {
		let v: Vec<&str> = text.split('*').collect();
		
		//println!("get_menu_1_sub_menu_data v - {:?}", v);
		
		let vector_length = v.len();
		
		if vector_length == 0 {
			let response_data = "Please note that you made a wrong selection.";
			let response_data = generate_ussd_response_message(&response_data.to_string(), false);

			return response_data.to_string()
		}
		
		let wrong_selection_data = "Please note that you made a wrong selection.";
		
		let response_data = 
			match vector_length {
				2 => //Index 1
					{
						let rent_mortgage_code = String::from(v[1]);
						
						if rent_mortgage_code.replace(" ","").len() == 0 {
							let response_data = "Please note that you did not enter Okoa Rent\\Mortgage number.";
							let response_data = generate_ussd_response_message(&response_data.to_string(), false);
			
							return response_data.to_string()
						}
						
						let mut sub_menu_data = String::from("");
						let sub_menu_1 = String::from("Enter Amount");
						//let sub_menu_2 = String::from("Enter 00 to go to previous Menu");
						let sub_menu_2 = String::from("0:Back 00:Home");
						
						sub_menu_data.push_str(&sub_menu_1);
						sub_menu_data.push_str("\n");
						sub_menu_data.push_str(&sub_menu_2);
						
						sub_menu_data = generate_ussd_response_message(&sub_menu_data, true);

						return sub_menu_data.to_string()
					},
				3 => //Index 2
					{
						let amount = String::from(v[2]);
						
						if amount.replace(" ","").len() == 0 {
							let response_data = "Please note that you did not enter amount.";
							let response_data = generate_ussd_response_message(&response_data.to_string(), false);
			
							return response_data.to_string()
						}
						
						let is_valid = validate_numeric(&amount);//validate_minimum_amount
						
						if !is_valid {
							let mut sub_menu_data = String::from("");
							let sub_menu_1 = String::from("Please note that you entered invalid amount.");
							let sub_menu_2 = String::from("0:Back 00:Home");
							
							sub_menu_data.push_str(&sub_menu_1);
							sub_menu_data.push_str("\n");
							sub_menu_data.push_str(&sub_menu_2);
							
							sub_menu_data = generate_ussd_response_message(&sub_menu_data, true);

							return sub_menu_data.to_string()
						}
						
						let is_valid = validate_minimum_amount(&amount);
						
						if !is_valid {
							let mut sub_menu_data = String::from("");
							let sub_menu_1 = String::from("Please note that the minimum amount is Ksh 100.");
							let sub_menu_2 = String::from("0:Back 00:Home");
							
							sub_menu_data.push_str(&sub_menu_1);
							sub_menu_data.push_str("\n");
							sub_menu_data.push_str(&sub_menu_2);
							
							sub_menu_data = generate_ussd_response_message(&sub_menu_data, true);

							return sub_menu_data.to_string()
						}
						
						let is_valid = validate_maximum_amount(&amount);
						
						if !is_valid {
							let mut sub_menu_data = String::from("");
							let sub_menu_1 = String::from("Please note that the maximum amount is Ksh 15,000.");
							let sub_menu_2 = String::from("0:Back 00:Home");
							
							sub_menu_data.push_str(&sub_menu_1);
							sub_menu_data.push_str("\n");
							sub_menu_data.push_str(&sub_menu_2);
							
							sub_menu_data = generate_ussd_response_message(&sub_menu_data, true);

							return sub_menu_data.to_string()
						}
						
						let rent_mortgage_code = String::from(v[1]);
						
						let mut sub_menu_data = String::from("");
						let sub_menu_1 = String::from("Please confirm payment details.");
						let sub_menu_2 = String::from("1. Cancel");
						let sub_menu_3 = String::from("2. Send");
						
						sub_menu_data.push_str(&sub_menu_1);
						sub_menu_data.push_str("\n");
						sub_menu_data.push_str(&String::from("Pay Back for Okoa Rent\\Mortgage number "));
						sub_menu_data.push_str(&rent_mortgage_code);
						sub_menu_data.push_str(&String::from(","));
						sub_menu_data.push_str("\n");
						sub_menu_data.push_str(&String::from("amount of Ksh "));
						sub_menu_data.push_str(&amount);
						sub_menu_data.push_str(&String::from("."));
						sub_menu_data.push_str("\n");
						sub_menu_data.push_str(&sub_menu_2);
						sub_menu_data.push_str("\n");
						sub_menu_data.push_str(&sub_menu_3);
						
						sub_menu_data = generate_ussd_response_message(&sub_menu_data, true);

						return sub_menu_data.to_string()
					},	
				4 => //Index 3
					{	
						let input = String::from(v[3]);
						
						if input.replace(" ","").len() == 0 {
							let response_data = wrong_selection_data.to_string();
							let response_data = generate_ussd_response_message(&response_data, false);
			
							return response_data.to_string()
						}
						
						if input.eq("1") {
							let mut sub_menu_data = String::from("Please note that you you cancelled the request.");
							sub_menu_data = generate_ussd_response_message(&sub_menu_data, false);
							
							return sub_menu_data.to_string()
						}
						else if input.eq("2") {
							let mut sub_menu_data = String::from("");
							let sub_menu_1 = String::from("Enter PIN");
							
							sub_menu_data.push_str(&sub_menu_1);
							
							sub_menu_data = generate_ussd_response_message(&sub_menu_data, true);

							return sub_menu_data.to_string()
						}
						else {
							let mut sub_menu_data = String::from("");
							let sub_menu_1 = wrong_selection_data.to_string();
							let sub_menu_2 = String::from("0:Back 00:Home");
							
							sub_menu_data.push_str(&sub_menu_1);
							sub_menu_data.push_str("\n");
							sub_menu_data.push_str(&sub_menu_2);
							
							sub_menu_data = generate_ussd_response_message(&sub_menu_data, true);

							return sub_menu_data.to_string()
						}
						
					},
				5 => //Index 4
					{
						let pin = String::from(v[4]);
						
						if pin.replace(" ","").len() == 0 {
							let response_data = "Please note that you did not enter PIN.";
							let response_data = generate_ussd_response_message(&response_data.to_string(), false);
			
							return response_data.to_string()
						}
						
						let mut sub_menu_data = String::from("");
						
						if pin.eq("1234") {
							let sub_menu_1 = String::from("Dear Customer, please note that you will get a prompt to enter Mpesa Pin to complete the transaction.");
							let sub_menu_2 = String::from("Thank you.");
							
							let rent_mortgage_code = String::from(v[1]);
							let amount_s = String::from(v[2]);
							
							let amount = 
								match amount_s.parse::<i32>() {
								  Ok(a) => a,
								  Err(e) => 0,
							};
							
							db_layer::create_pay_back_data(data, rent_mortgage_code, amount, mobile_no.to_string());
							
							sub_menu_data.push_str(&sub_menu_1);
							sub_menu_data.push_str("\n");
							sub_menu_data.push_str(&sub_menu_2);
							
							sub_menu_data = generate_ussd_response_message(&sub_menu_data, false);
							
							return sub_menu_data.to_string()
						}
						else {
							sub_menu_data = String::from("Please note that you entered an invalid PIN.");
							sub_menu_data = generate_ussd_response_message(&sub_menu_data, false);
							
							return sub_menu_data.to_string()
						}

					},
				_ => {
					let response_data = wrong_selection_data.to_string();
					let response_data = generate_ussd_response_message(&response_data, false);
			
					return response_data.to_string()
					},
        };
		
		return response_data
	}
}

fn get_menu_5_sub_menu_data(data: &web::Data<Pool>, mobile_no: &String, text: &String) -> String {
		
	if text.replace(" ","").len() == 0 {
		let response_data = "Please note that you made a wrong selection.";
		let response_data = generate_ussd_response_message(&response_data.to_string(), false);
		
		return response_data.to_string()
		
	}
	else if !text.contains("*") {
		let mut sub_menu_data = String::from("");
		let sub_menu_1 = String::from("Please select type of statement.");
		let sub_menu_2 = String::from("1. Full Statement");
		let sub_menu_3 = String::from("2. Mini Statement");
		
		sub_menu_data.push_str(&sub_menu_1);
		sub_menu_data.push_str("\n");
		sub_menu_data.push_str(&sub_menu_2);
		sub_menu_data.push_str("\n");
		sub_menu_data.push_str(&sub_menu_3);
		
		sub_menu_data = generate_ussd_response_message(&sub_menu_data, true);

		return sub_menu_data.to_string()
	}
	else {
		let v: Vec<&str> = text.split('*').collect();
		
		//println!("get_menu_1_sub_menu_data v - {:?}", v);
		
		let vector_length = v.len();
		
		if vector_length == 0 {
			let response_data = "Please note that you made a wrong selection.";
			let response_data = generate_ussd_response_message(&response_data.to_string(), false);

			return response_data.to_string()
		}
		
		let wrong_selection_data = "Please note that you made a wrong selection.";
		
		let response_data = 
			match vector_length {
				2 => //Index 1
					{
						let input = String::from(v[1]);
						
						if input.replace(" ","").len() == 0 {
							let response_data = "Please note that you did not select type of statement.";
							let response_data = generate_ussd_response_message(&response_data.to_string(), false);
			
							return response_data.to_string()
						}
						
						let mut sub_menu_data = String::from("");
						let mut statement_type = String::from("");
						
						if input.eq("1") {
							statement_type = String::from("Full Statement");
						}
						else if input.eq("2") {
							statement_type = String::from("Mini Statement");
						}
						else {
							sub_menu_data = wrong_selection_data.to_string();
							sub_menu_data = generate_ussd_response_message(&sub_menu_data, false);
							
							return sub_menu_data.to_string()
						}
						
						let sub_menu_1 = String::from("Please confirm request details.");
						let sub_menu_2 = String::from("1. Cancel");
						let sub_menu_3 = String::from("2. Send");
						
						sub_menu_data.push_str(&sub_menu_1);
						sub_menu_data.push_str("\n");
						sub_menu_data.push_str(&String::from("Check "));
						sub_menu_data.push_str(&statement_type);
						sub_menu_data.push_str(&String::from("."));
						sub_menu_data.push_str("\n");
						sub_menu_data.push_str(&sub_menu_2);
						sub_menu_data.push_str("\n");
						sub_menu_data.push_str(&sub_menu_3);
						
						sub_menu_data = generate_ussd_response_message(&sub_menu_data, true);

						return sub_menu_data.to_string()
					},
					
				3 => //Index 2
					{	
						let input = String::from(v[2]);
						
						if input.replace(" ","").len() == 0 {
							let response_data = wrong_selection_data.to_string();
							let response_data = generate_ussd_response_message(&response_data, false);
			
							return response_data.to_string()
						}
						
						let mut sub_menu_data = String::from("");
						
						if input.eq("1") {
							sub_menu_data = String::from("Please note that you you cancelled the request.");
							sub_menu_data = generate_ussd_response_message(&sub_menu_data, false);
							
							return sub_menu_data.to_string()
						}
						else if input.eq("2") {
							let sub_menu_1 = String::from("Dear Customer, we acknowledge receipt of your Get Statement submission.");
							let sub_menu_2 = String::from("It is being processed and you will be notified once complete.");
							let sub_menu_3 = String::from("Thank you.");
							
							let input = String::from(v[1]);
							let mut full_statement: bool = false;
							let mut mini_statement: bool = false;
							
							if input.eq("1") {
								full_statement = true;
							}
							else if input.eq("2") {
								mini_statement = true;
							}
							else {
								full_statement = false;
								mini_statement = false;
							}
							
							db_layer::create_get_statement_data(data, full_statement, mini_statement, mobile_no.to_string());
							
							sub_menu_data.push_str(&sub_menu_1);
							sub_menu_data.push_str("\n");
							sub_menu_data.push_str(&sub_menu_2);
							sub_menu_data.push_str("\n");
							sub_menu_data.push_str(&sub_menu_3);
							
							sub_menu_data = generate_ussd_response_message(&sub_menu_data, false);
							
							return sub_menu_data.to_string()
						}
						else {
							/*
							sub_menu_data = wrong_selection_data.to_string();
							let sub_menu_data = generate_ussd_response_message(&sub_menu_data, false);
							
							return sub_menu_data.to_string()
							*/
							
							let mut sub_menu_data = String::from("");
							let sub_menu_1 = wrong_selection_data.to_string();
							let sub_menu_2 = String::from("0:Back 00:Home");
							
							sub_menu_data.push_str(&sub_menu_1);
							sub_menu_data.push_str("\n");
							sub_menu_data.push_str(&sub_menu_2);
							
							sub_menu_data = generate_ussd_response_message(&sub_menu_data, true);

							return sub_menu_data.to_string()
						}
						
					},
				_ => {
						let response_data = wrong_selection_data.to_string();
						let response_data = generate_ussd_response_message(&response_data, false);
		
						return response_data.to_string()
					},
        };
		
		return response_data
	}
}

fn get_menu_1_sub_menu_data_unregistered_client(data: &web::Data<Pool>, mobile_no: &String, text: &String) -> String {
		
	if text.replace(" ","").len() == 0 {
		let response_data = "Please note that you made a wrong selection.";
		let response_data = generate_ussd_response_message(&response_data.to_string(), false);
		
		return response_data.to_string()
		
	}
	else if !text.contains("*") {
		let mut sub_menu_data = String::from("");
		let sub_menu_1 = String::from("Enter National ID number");
		let sub_menu_2 = String::from("00:Home");
		
		sub_menu_data.push_str(&sub_menu_1);
		sub_menu_data.push_str("\n");
		sub_menu_data.push_str(&sub_menu_2);
		
		sub_menu_data = generate_ussd_response_message(&sub_menu_data, true);

		return sub_menu_data.to_string()
	}
	else {
		let v: Vec<&str> = text.split('*').collect();
		
		let vector_length = v.len();
		
		if vector_length == 0 {
			let response_data = "Please note that you made a wrong selection.";
			let response_data = generate_ussd_response_message(&response_data.to_string(), false);

			return response_data.to_string()
		}
		
		let wrong_selection_data = "Please note that you made a wrong selection.";
		
		let response_data = 
			match vector_length {
				2 => //Index 1
					{
						let national_id = String::from(v[1]);
						let national_id = national_id.replace(" ","");
						
						if national_id.len() == 0 {
							let response_data = "Please note that you did not enter national ID number.";
							let response_data = generate_ussd_response_message(&response_data.to_string(), false);
			
							return response_data.to_string()
						}	
						else if national_id.len() == 7 || national_id.len() == 8 {
						}
						else {
							let mut sub_menu_data = String::from("");
							let sub_menu_1 = String::from("Please note that national ID number must have 7 or 8 digits.");
							let sub_menu_2 = String::from("00:Home");
							
							sub_menu_data.push_str(&sub_menu_1);
							sub_menu_data.push_str("\n");
							sub_menu_data.push_str(&sub_menu_2);
							
							sub_menu_data = generate_ussd_response_message(&sub_menu_data, true);

							return sub_menu_data.to_string()
						}
							
						let valid_national_id = 
							match national_id.parse::<i64>() {
							  Ok(a) => { if a > 0 {true} else {false} },
							  Err(e) => false,
						};
						
						if !valid_national_id {
							let mut sub_menu_data = String::from("");
							let sub_menu_1 = String::from("Please note that national ID number must contain digits only.");
							let sub_menu_2 = String::from("00:Home");
							
							sub_menu_data.push_str(&sub_menu_1);
							sub_menu_data.push_str("\n");
							sub_menu_data.push_str(&sub_menu_2);
							
							sub_menu_data = generate_ussd_response_message(&sub_menu_data, true);

							return sub_menu_data.to_string()
						}
						
						let mut sub_menu_data = String::from("");
						let sub_menu_1 = String::from("Enter Full Names");
						let sub_menu_2 = String::from("eg Firstname Lastname");
						let sub_menu_3 = String::from("00:Home");
						
						sub_menu_data.push_str(&sub_menu_1);
						sub_menu_data.push_str("\n");
						sub_menu_data.push_str(&sub_menu_2);
						sub_menu_data.push_str("\n");
						sub_menu_data.push_str(&sub_menu_3);
						
						sub_menu_data = generate_ussd_response_message(&sub_menu_data, true);

						return sub_menu_data.to_string()
					},
				3 => //Index 2
					{
						let full_names = String::from(v[2]);
						
						if full_names.replace(" ","").len() == 0 {
							let response_data = "Please note that you did not enter full names.";
							let response_data = generate_ussd_response_message(&response_data.to_string(), false);
			
							return response_data.to_string()
						}
						
						if !full_names.contains(" ") {
			
							let mut sub_menu_data = String::from("");
							let sub_menu_1 = String::from("Please enter firstname and lastname separated with a space.");
							let sub_menu_2 = String::from("00:Home");
							
							sub_menu_data.push_str(&sub_menu_1);
							sub_menu_data.push_str("\n");
							sub_menu_data.push_str(&sub_menu_2);
							
							sub_menu_data = generate_ussd_response_message(&sub_menu_data, true);

							return sub_menu_data.to_string()
						}
						
						let mut sub_menu_data = String::from("");
						let sub_menu_1 = String::from("Enter House Code");
						let sub_menu_2 = String::from("00:Home");
						
						sub_menu_data.push_str(&sub_menu_1);
						sub_menu_data.push_str("\n");
						sub_menu_data.push_str(&sub_menu_2);
						
						sub_menu_data = generate_ussd_response_message(&sub_menu_data, true);

						return sub_menu_data.to_string()
					},
						
				4 => //Index 3
					{
						let house_code = String::from(v[3]);
						
						if house_code.replace(" ","").len() == 0 {
							let response_data = "Please note that you did not enter house code.";
							let response_data = generate_ussd_response_message(&response_data.to_string(), false);
			
							return response_data.to_string()
						}
						
						let national_id = String::from(v[1]);
						let full_names = String::from(v[2]);
						let full_names = full_names.to_uppercase();
						
						let mut sub_menu_data = String::from("");
						let sub_menu_1 = String::from("Please confirm self registration details.");
						let sub_menu_2 = String::from("1. Cancel");
						let sub_menu_3 = String::from("2. Send");
						
						sub_menu_data.push_str(&sub_menu_1);
						sub_menu_data.push_str("\n");
						sub_menu_data.push_str(&String::from("National ID: "));
						sub_menu_data.push_str(&national_id);
						sub_menu_data.push_str("\n");
						sub_menu_data.push_str(&String::from("Full Names: "));
						sub_menu_data.push_str(&full_names);
						sub_menu_data.push_str("\n");
						sub_menu_data.push_str(&String::from("House Code: "));
						sub_menu_data.push_str(&house_code);
						sub_menu_data.push_str(&String::from("."));
						sub_menu_data.push_str("\n");
						sub_menu_data.push_str(&sub_menu_2);
						sub_menu_data.push_str("\n");
						sub_menu_data.push_str(&sub_menu_3);
						
						sub_menu_data = generate_ussd_response_message(&sub_menu_data, true);

						return sub_menu_data.to_string()
					},	
				5 => //Index 4
					{	
						let input = String::from(v[4]);
						
						if input.replace(" ","").len() == 0 {
							let response_data = wrong_selection_data.to_string();
							let response_data = generate_ussd_response_message(&response_data, false);
			
							return response_data.to_string()
						}
						
						let mut sub_menu_data = String::from("");
						
						if input.eq("1") {
							sub_menu_data = String::from("Please note that you you cancelled the request.");
							sub_menu_data = generate_ussd_response_message(&sub_menu_data, false);
							
							return sub_menu_data.to_string()
						}
						else if input.eq("2") {
							let sub_menu_1 = String::from("Dear Customer, we acknowledge receipt of your self registration submission.");
							let sub_menu_2 = String::from("It is being processed and you will be notified once complete.");
							let sub_menu_3 = String::from("Thank you.");
							
							let national_id = String::from(v[1]);
							let full_names = String::from(v[2]);
							let house_code = String::from(v[3]);
							
							let full_names = full_names.to_uppercase();
							
							db_layer::create_self_registration_data(data, national_id, full_names, house_code, mobile_no.to_string());
							
							sub_menu_data.push_str(&sub_menu_1);
							sub_menu_data.push_str("\n");
							sub_menu_data.push_str(&sub_menu_2);
							sub_menu_data.push_str("\n");
							sub_menu_data.push_str(&sub_menu_3);
							
							sub_menu_data = generate_ussd_response_message(&sub_menu_data, false);
							
							return sub_menu_data.to_string()
						}
						else {
							let mut sub_menu_data = String::from("");
							let sub_menu_1 = wrong_selection_data.to_string();
							//let sub_menu_2 = String::from("0:Back 00:Home");
							let sub_menu_2 = String::from("00:Home");
							
							sub_menu_data.push_str(&sub_menu_1);
							sub_menu_data.push_str("\n");
							sub_menu_data.push_str(&sub_menu_2);
							
							sub_menu_data = generate_ussd_response_message(&sub_menu_data, true);

							return sub_menu_data.to_string()
						}
						
					},
						
				_ => {
					let response_data = wrong_selection_data.to_string();
					let response_data = generate_ussd_response_message(&response_data, false);
			
					return response_data.to_string()
					},
        };
		
		return response_data
	}
}

fn get_sub_menu_data(data: &web::Data<Pool>, mobile_no: &String, text: &String) -> String {
	
	if text.replace(" ","").len() == 0 {
		let response_data = "Please note that you made a wrong selection.";
		let response_data = generate_ussd_response_message(&response_data.to_string(), false);
		
		return response_data.to_string()
	}
	
	if text.contains("*") {
		let v: Vec<&str> = text.split('*').collect();
		
		//println!("v - {:?}", v);
		
		let vector_length = v.len();
		
		if vector_length == 0 {
			let response_data = "Please note that you made a wrong selection.";
			let response_data = generate_ussd_response_message(&response_data.to_string(), false);

			return response_data.to_string()
		}
		
		let wrong_selection_data = "Please note that you made a wrong selection.";
		let menu_selected = String::from(v[0]);
		
		let response_data = 
			match menu_selected.as_str() {
				"1" => //menu_1
						get_menu_1_sub_menu_data(data, mobile_no, text),
				"2" => //menu_2
						get_menu_2_sub_menu_data(data, mobile_no, text),
			    "3" => //menu_3
						get_menu_3_sub_menu_data(data, mobile_no, text),
				"4" => //menu_4
						get_menu_4_sub_menu_data(data, mobile_no, text),
			    "5" => //menu_5
						get_menu_5_sub_menu_data(data, mobile_no, text),
				_ => wrong_selection_data.to_string(),
        };
		
		return response_data
	}
	else {
		let wrong_selection_data = "Please note that you made a wrong selection.";
		
		//println!("get_sub_menu_data: text - {:?}", text);
	
		let response_data = 
			match text.as_str() {
				"1" => //menu_1
					get_menu_1_sub_menu_data(data, mobile_no, text),
				"2" => //menu_2
					get_menu_2_sub_menu_data(data, mobile_no, text),
			    "3" => //menu_3
					get_menu_3_sub_menu_data(data, mobile_no, text),
				"4" => //menu_4
					get_menu_4_sub_menu_data(data, mobile_no, text),
			    "5" => //menu_5
					get_menu_5_sub_menu_data(data, mobile_no, text),
				_ => wrong_selection_data.to_string(),
        };
		
		return response_data
	}
}

fn get_sub_menu_data_unregistered_client(data: &web::Data<Pool>, mobile_no: &String, text: &String) -> String {
	
	if text.replace(" ","").len() == 0 {
		let response_data = "Please note that you made a wrong selection.";
		let response_data = generate_ussd_response_message(&response_data.to_string(), false);
		
		return response_data.to_string()
	}
	
	if text.contains("*") {
		let v: Vec<&str> = text.split('*').collect();
		
		let vector_length = v.len();
		
		if vector_length == 0 {
			let response_data = "Please note that you made a wrong selection.";
			let response_data = generate_ussd_response_message(&response_data.to_string(), false);

			return response_data.to_string()
		}
		
		let wrong_selection_data = "Please note that you made a wrong selection.";
		let menu_selected = String::from(v[0]);
		
		let response_data = 
			match menu_selected.as_str() {
				"1" => //menu_1
						get_menu_1_sub_menu_data_unregistered_client(data, mobile_no, text),
				_ => wrong_selection_data.to_string(),
        };
		
		return response_data
	}
	else {
		let wrong_selection_data = "Please note that you made a wrong selection.";
	
		let response_data = 
			match text.as_str() {
				"1" => //menu_1
					get_menu_1_sub_menu_data_unregistered_client(data, mobile_no, text),
				_ => wrong_selection_data.to_string(),
        };
		
		return response_data
	}
}

fn generate_ussd_response_message(request_message: &String, is_proceed: bool) -> String {
	let mut response_message = String::from("");
	let con_message = String::from("CON");
	let end_message = String::from("END");
	
	if request_message.replace(" ","").len() == 0 {
		response_message = con_message;
		return response_message
	}
	
	if is_proceed {
		response_message.push_str(&con_message);
		response_message.push_str(&String::from(" "));
		response_message.push_str(&request_message);
	}
	else {
		response_message.push_str(&end_message);
		response_message.push_str(&String::from(" "));
		response_message.push_str(&request_message);
	}

	response_message
}

fn process_client_requests(data: &web::Data<Pool>, session_id: &String, phone_number: &String, text: &String) -> String {
	
	let text = text.replace("'","");
	let text = text.replace("--","");
	let text = text.replace(" ","");
	
	//let caller_action: String = create_ussd_session_details_2(data, session_id.to_string(), phone_number.to_string(), text.to_string());
	db_layer::create_ussd_session_details(data, session_id.to_string(), phone_number.to_string(), text.to_string());
	
	let caller_action: String = db_layer::get_ussd_session_details(data, &session_id, &phone_number);
	
	let caller_action = caller_action.replace(" ","");
	
	if caller_action.replace(" ","").len() == 0 {
		let response_data = get_menu_data();
		let response_data = generate_ussd_response_message(&response_data, true);
		
		return response_data
	}
	else {
		//Lets process sub menu selection by user
		let response_data = get_sub_menu_data(&data, &phone_number, &caller_action);
		
		return response_data
	}
}

fn process_unregistered_client_requests(data: &web::Data<Pool>, session_id: &String, phone_number: &String, text: &String) -> String {
	
	let text = text.replace("'","");
	let text = text.replace("--","");
	//let text = text.replace(" ","");
	let text = text.replace("  "," ");
	
	db_layer::create_ussd_session_details(data, session_id.to_string(), phone_number.to_string(), text.to_string());
	
	let caller_action: String = db_layer::get_ussd_session_details(data, &session_id, &phone_number);
	
	let caller_action = caller_action.replace("  "," ");
	
	if caller_action.replace(" ","").len() == 0 {
		let response_data = get_menu_data_unregistered_client();
		let response_data = generate_ussd_response_message(&response_data, true);
		
		return response_data
	}
	else {
		//Lets process sub menu selection by user
		let response_data = get_sub_menu_data_unregistered_client(&data, &phone_number, &caller_action);
		
		return response_data
	}
}

fn validate_numeric(input: &String) -> bool {
	let mut is_valid: bool = false;
	
	if input.replace(" ","").len() == 0 {return is_valid}
	
	let input_value = 
		match input.parse::<i32>() {
		  Ok(a) => a,
		  Err(e) => 0,
		};
		
	if input_value > 0 {is_valid = true}
	
	is_valid
}

fn validate_minimum_amount(input: &String) -> bool {
	let mut is_valid: bool = false;
	
	if input.replace(" ","").len() == 0 {return is_valid}
	
	let input_value = 
		match input.parse::<i32>() {
		  Ok(a) => a,
		  Err(e) => 0,
		};
	
	let minimum_amount: i32 = 100;	
	if input_value > 0 && input_value >= minimum_amount {is_valid = true}
	
	is_valid
}

fn validate_maximum_amount(input: &String) -> bool {
	let mut is_valid: bool = false;
	
	if input.replace(" ","").len() == 0 {return is_valid}
	
	let input_value = 
		match input.parse::<i32>() {
		  Ok(a) => a,
		  Err(e) => 0,
		};
	
	let maximum_amount: i32 = 15000;	
	if input_value > 0 && input_value <= maximum_amount {is_valid = true}
	
	is_valid
}

fn get_conn_url() -> String {
	let url = "mysql://ussd:arunga@2030!@localhost:3306/okoa_rent";
	//let url = "mysql://app1:23$)W.@9smtf!qp7@localhost:3306/okoa_rent"; cloud server
	String::from(url)
}

#[actix_web::main]
async fn main() {
	
	/*
	.bind("127.0.0.1:9247")? //accessible from the machine only
	.bind("0.0.0.0:9247")? //accessible from outside the machine itself
	*/
	
	let url = get_conn_url();
     
    let pool = match Pool::new(url) {
        Ok(pool) => pool,
        Err(e) => {
            println!("Failed to open DB connection. {:?}", e); return;
        }
    };
 
    let shared_data = web::Data::new(pool);
	
	let server = match HttpServer::new(move || {
        App::new()
            .app_data(shared_data.clone())
			.route("/", web::get().to(greet))
			.service(process_ussd_actions)
    }).bind("0.0.0.0:9247") {
        Ok(s) => s,
        Err(e) => {
            println!("Failed to bind port. {:?}", e);
            return;
        }
    };
	
    match server.run().await {
        Ok(_) => println!("Server exited normally."),
        Err(e) => println!("Server exited with error: {:?}", e),
    };
}