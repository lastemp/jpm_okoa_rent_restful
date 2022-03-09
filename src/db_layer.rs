use mysql::*;
use mysql::prelude::*;
use actix_web::{web};

pub fn create_ussd_session_details(data: &web::Data<Pool>, session_id: String, caller_number: String, caller_action: String) -> bool {
	let mut successful: bool = false;
	
	match data
        .get_conn()
		.and_then(|mut conn| insert_ussd_session_details(&mut conn, session_id, caller_number, caller_action))
    {
        Ok(x) => {
			successful = true;
        },
        Err(e) => println!("Failed to open DB connection. create_ussd_session_details {:?}", e),
    }
	
	successful
}
/*
fn create_ussd_session_details_2(data: &web::Data<Pool>, session_id: String, caller_number: String, caller_action: String) -> String {
	let mut caller_action_new: String = String::from("");
	
	match data
        .get_conn()
		.and_then(|mut conn| insert_ussd_session_details_2(&mut conn, session_id, caller_number, caller_action))
    {
        Ok(data) => {
			//println!("data - {:?}", data);
			caller_action_new = data;
        },
        Err(e) => println!("Failed to open DB connection. create_ussd_session_details {:?}", e),
    }
	
	caller_action_new
}
*/
pub fn create_okoa_rent_data(data: &web::Data<Pool>, tenant_code: String, house_code: String, amount: i32, mobile_no: String) -> bool  {
	let mut successful: bool = false;
	
	match data
        .get_conn()
		.and_then(|mut conn| insert_okoa_rent_data(&mut conn, tenant_code, house_code, amount, mobile_no))
    {
        Ok(x) => {
			if x > 0 {
				successful = true;
			}
			else {
				successful = false;
			}
        },
        Err(e) => println!("Failed to open DB connection. {:?}", e),
    }
	
	successful
}

pub fn create_okoa_mortgage_data(data: &web::Data<Pool>, mortgagor_code: String, house_code: String, amount: i32, mobile_no: String) -> bool  {
	let mut successful: bool = false;
	
	match data
        .get_conn()
		.and_then(|mut conn| insert_okoa_mortgage_data(&mut conn, mortgagor_code, house_code, amount, mobile_no))
    {
        Ok(x) => {
			if x > 0 {
				successful = true;
			}
			else {
				successful = false;
			}
        },
        Err(e) => println!("Failed to open DB connection. {:?}", e),
    }
	
	successful
}

pub fn create_check_balance_data(data: &web::Data<Pool>, rent_mortgage_code: String, mobile_no: String) -> bool  {
	let mut successful: bool = false;
	
	match data
        .get_conn()
		.and_then(|mut conn| insert_check_balance_data(&mut conn, rent_mortgage_code, mobile_no))
    {
        Ok(x) => {
			if x > 0 {
				successful = true;
			}
			else {
				successful = false;
			}
        },
        Err(e) => println!("Failed to open DB connection. {:?}", e),
    }
	
	successful
}

pub fn create_get_statement_data(data: &web::Data<Pool>, full_statement: bool, mini_statement: bool, mobile_no: String) -> bool  {
	let mut successful: bool = false;
	
	match data
        .get_conn()
		.and_then(|mut conn| insert_get_statement_data(&mut conn, full_statement, mini_statement, mobile_no))
    {
        Ok(x) => {
			if x > 0 {
				successful = true;
			}
			else {
				successful = false;
			}
        },
        Err(e) => println!("Failed to open DB connection. {:?}", e),
    }
	
	successful
}

pub fn create_pay_back_data(data: &web::Data<Pool>, rent_mortgage_code: String, amount: i32, mobile_no: String) -> bool  {
	let mut successful: bool = false;
	
	match data
        .get_conn()
		.and_then(|mut conn| insert_pay_back_data(&mut conn, rent_mortgage_code, amount, mobile_no))
    {
        Ok(x) => {
			if x > 0 {
				successful = true;
			}
			else {
				successful = false;
			}
        },
        Err(e) => println!("Failed to open DB connection. {:?}", e),
    }
	
	successful
}

pub fn create_self_registration_data(data: &web::Data<Pool>, national_id_no: String, full_names: String, house_code: String, mobile_no: String) -> bool  {
	let mut successful: bool = false;
	
	match data
        .get_conn()
		.and_then(|mut conn| insert_self_registration_data(&mut conn, national_id_no, full_names, house_code, mobile_no))
    {
        Ok(x) => {
			if x > 0 {
				successful = true;
			}
			else {
				successful = false;
			}
        },
        Err(e) => println!("Failed to open DB connection. {:?}", e),
    }
	
	successful
}

pub fn create_outgoing_sms_message_data(data: &web::Data<Pool>, sms_message: String, _to: String, _from: String, _message: String, message_id: String, _number: String, status_code: i32, _status: String, _cost: String) -> bool  {
	let mut successful: bool = false;
	
	match data
        .get_conn()
		.and_then(|mut conn| insert_outgoing_sms_message_data(&mut conn, sms_message, _to, _from, _message, message_id, _number, status_code, _status, _cost))
    {
        Ok(x) => {
			if x > 0 {
				successful = true;
			}
			else {
				successful = false;
			}
        },
        Err(e) => println!("Failed to open DB connection. {:?}", e),
    }
	
	successful
}

pub fn get_ussd_session_details(data: &web::Data<Pool>, session_id: &String, phone_number: &String) -> String  {
	let mut caller_action: String = String::from("");
	
	match data
        .get_conn()
		.and_then(|mut conn| select_ussd_session_details(&mut conn, session_id.to_string(), phone_number.to_string()))
    {
        Ok(x) => {
			caller_action = x;
        },
        Err(e) => println!("Failed to open DB connection. {:?}", e),
    }
	
	caller_action
}

pub fn get_ussd_registered_client(data: &web::Data<Pool>, phone_number: &String) -> bool {
	let mut is_registered: bool = false;

	match data
        .get_conn()
		.and_then(|mut conn| select_registered_client_details(&mut conn, phone_number.to_string()))
    {
        Ok(x) => {
			if x > 0 {is_registered = true;} else {is_registered = false;}
        },
        Err(e) => println!("Failed to open DB connection. {:?}", e),
    }
	
	is_registered
}

fn insert_ussd_session_details(
    conn: &mut PooledConn, session_id: String, caller_number: String, caller_action: String) -> std::result::Result<u64, mysql::error::Error> {
	
	let is_active: i32 = 1;

	conn.exec_drop(
        "call insertussdsessiondetails (:mysessionid, :myisactive, :mycallernumber, :mycalleraction);",
        params! {
            "mysessionid" => session_id,
            "myisactive" => is_active,
            "mycallernumber" => caller_number,
			"mycalleraction" => caller_action,
        },
    )
	.and_then(|_| Ok(1))
}
/*
fn insert_ussd_session_details_2(
    conn: &mut PooledConn, session_id: String, caller_number: String, caller_action: String) -> std::result::Result<String, mysql::error::Error> {
	
	let is_active: i32 = 1;
	let mut caller_action_new: String = String::from("");
	
	//"call insertussdsessiondetails_out (:mysessionid, :myisactive, :mycallernumber, :mycalleraction, :mycalleraction_new);",
	//conn.exec_drop(
	conn.exec_map(
        "call insertussdsessiondetails_out (:mysessionid, :myisactive, :mycallernumber, :mycalleraction, :mycalleraction_new);",
        params! {
            "mysessionid" => session_id,
            "myisactive" => is_active,
            "mycallernumber" => caller_number,
			"mycalleraction" => caller_action,
			"mycalleraction_new" => String::from(""),
        },
		|(mycalleraction_new)| {
            caller_action_new = mycalleraction_new;
        },
    )
	.and_then(|_| Ok(caller_action_new))
}
*/
fn insert_okoa_rent_data(
    conn: &mut PooledConn, tenant_code: String, house_code: String, amount: i32, mobile_no: String) -> std::result::Result<u64, mysql::error::Error> {
	
	// Now let's insert data to the database
	conn.exec_drop(
        "insert into incomingokoarentdatarequests (tenantcode, housecode, amount, mobileno) values (:tenant_code, :house_code, :amount, :mobile_no);",
        params! {
            "tenant_code" => tenant_code,
            "house_code" => house_code,
            "amount" => amount,
            "mobile_no" => mobile_no,
        },
    )
	.and_then(|_| Ok(conn.last_insert_id()))
}

fn insert_okoa_mortgage_data(
    conn: &mut PooledConn, mortgagor_code: String, house_code: String, amount: i32, mobile_no: String) -> std::result::Result<u64, mysql::error::Error> {
	
	// Now let's insert data to the database
	conn.exec_drop(
        "insert into incomingokoamortgagedatarequests (mortgagorcode, housecode, amount, mobileno) values (:mortgagor_code, :house_code, :amount, :mobile_no);",
        params! {
            "mortgagor_code" => mortgagor_code,
            "house_code" => house_code,
            "amount" => amount,
            "mobile_no" => mobile_no,
        },
    )
	.and_then(|_| Ok(conn.last_insert_id()))
}

fn insert_check_balance_data(
    conn: &mut PooledConn, rent_mortgage_code: String, mobile_no: String) -> std::result::Result<u64, mysql::error::Error> {
	
	// Now let's insert data to the database
	conn.exec_drop(
        "insert into incomingcheckbalancedatarequests (rentmortgagecode, mobileno) values (:rent_mortgage_code, :mobile_no);",
        params! {
            "rent_mortgage_code" => rent_mortgage_code,
            "mobile_no" => mobile_no,
        },
    )
	.and_then(|_| Ok(conn.last_insert_id()))
}

fn insert_get_statement_data(
    conn: &mut PooledConn, full_statement: bool, mini_statement: bool, mobile_no: String) -> std::result::Result<u64, mysql::error::Error> {
	
	// Now let's insert data to the database
	conn.exec_drop(
        "insert into incominggetstatementdatarequests (fullstatement, ministatement, mobileno) values (:full_statement, :mini_statement, :mobile_no);",
        params! {
            "full_statement" => full_statement,
			"mini_statement" => mini_statement,
            "mobile_no" => mobile_no,
        },
    )
	.and_then(|_| Ok(conn.last_insert_id()))
}

fn insert_pay_back_data(
    conn: &mut PooledConn, rent_mortgage_code: String, amount: i32, mobile_no: String) -> std::result::Result<u64, mysql::error::Error> {
	
	// Now let's insert data to the database
	conn.exec_drop(
        "insert into incomingpaybackdatarequests (rentmortgagecode, amount, mobileno) values (:rent_mortgage_code, :amount, :mobile_no);",
        params! {
            "rent_mortgage_code" => rent_mortgage_code,
            "amount" => amount,
            "mobile_no" => mobile_no,
        },
    )
	.and_then(|_| Ok(conn.last_insert_id()))
}

fn insert_self_registration_data(
    conn: &mut PooledConn, national_id_no: String, full_names: String, house_code: String, mobile_no: String) -> std::result::Result<u64, mysql::error::Error> {
	
	// Now let's insert data to the database
	conn.exec_drop(
        "insert into incomingselfregistrationdatarequests (nationalidno, fullnames, housecode, mobileno) values (:national_id_no, :full_names, :house_code, :mobile_no);",
        params! {
            "national_id_no" => national_id_no,
            "full_names" => full_names,
            "house_code" => house_code,
            "mobile_no" => mobile_no,
        },
    )
	.and_then(|_| Ok(conn.last_insert_id()))
}

fn insert_outgoing_sms_message_data(
    conn: &mut PooledConn, sms_message: String, _to: String, _from: String, _message: String, message_id: String, _number: String, status_code: i32, _status: String, _cost: String) -> std::result::Result<u64, mysql::error::Error> {
	
	// Now let's insert data to the database
	conn.exec_drop(
        "insert into outgoingsmsmessagedatarequests (smsmessage, recipientmobileno, senderidname, message, messageid, recipientnumber, statuscode, statusmessage, cost) values (:sms_message, :_to, :_from, :_message, :message_id, :_number, :status_code, :_status, :_cost);",
        params! {
            "sms_message" => sms_message,
            "_to" => _to,
            "_from" => _from,
            "_message" => _message,
			"message_id" => message_id,
			"_number" => _number,
			"status_code" => status_code,
			"_status" => _status,
			"_cost" => _cost,
        },
    )
	.and_then(|_| Ok(conn.last_insert_id()))
}

fn select_ussd_session_details(
    conn: &mut PooledConn, session_id: String, caller_number: String) -> std::result::Result<String, mysql::error::Error> {
	let mut caller_action_new: String = String::from("");
	
    conn.exec_map(
        "select coalesce(calleraction,'') as caller_action from ussdsessiondetails where coalesce(sessionid,'') = :sessionid and coalesce(callernumber,'') = :callernumber and coalesce(duplicateentry,0) = 0 and coalesce(deleted,0) = 0 limit 1;",
		params! {
				"sessionid" => session_id,
				"callernumber" => caller_number,
			},
        |caller_action|
            caller_action_new = caller_action,
    )
	.and_then(|_| Ok(caller_action_new))
	
}

fn select_registered_client_details(
    conn: &mut PooledConn, mobile_number: String) -> std::result::Result<u64, mysql::error::Error> {
	let mut my_count: u64 = 0;
	
    conn.exec_map(
        "select count(id) as count from incomingselfregistrationdatarequests where coalesce(mobileno,'') = :mobileno and coalesce(duplicateentry,0) = 0 and coalesce(deleted,0) = 0;",
		params! {
				"mobileno" => mobile_number,
			},
        |count|
            my_count = count,
    )
	.and_then(|_| Ok(my_count))
	
}
