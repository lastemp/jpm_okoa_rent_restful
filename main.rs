extern crate base64;

use actix_web::{get, post, web, App, HttpRequest, HttpServer, Responder};
use serde::{Serialize, Deserialize};
use base64::{decode};//encode
use std::str;
use mysql::*;
use mysql::prelude::*;

//, Result

#[derive(Deserialize)]
struct Info {
    username: String,
	posted_by: PostedBy,
}

#[derive(Deserialize)]
struct PostedBy {
    staff_name: String,
	job_level: String,
}

#[derive(Deserialize)]
struct SalesBatchData {
    batch_no: Option<String>,
    sales_data: Vec<SalesData>,
}

#[derive(Deserialize)]
struct SalesData {
	customer_sales_data: CustomerSalesData,
	vehicle_sales_data: Option<VehicleSalesData>,
	carpet_sales_data: Option<CarpetSalesData>,
}

#[derive(Deserialize)]
struct CustomerSalesData {
    cust_name: String,
	mobile_no: String,
	sales_amount: String,
	paid_amount: String,
	payment_mode: String,
}

#[derive(Deserialize)]
struct VehicleSalesData {
    vehicle_make: String,
	vehicle_model: String,
	vehicle_regno: String,
	sales_amount: String,
	payment_mode: String,
	interior_cleaning: bool,
	exterior_cleaning: bool,
	engine_cleaning: bool,
	undercarriage_cleaning: bool,
	employee_id: i32,
	employee_full_names: String,
}

#[derive(Deserialize)]
struct CarpetSalesData {
    carpet_size: String,
	carpet_colour: String,
	sales_amount: String,
	payment_mode: String,
	employee_id: i32,
	employee_full_names: String,
}

#[derive(Deserialize)]
struct VehicleMakeData {
    mobile_no: Option<String>,
	device_registration_token: Option<String>,
}

#[derive(Deserialize)]
struct VehicleModelData {
    mobile_no: Option<String>,
	vehicle_make: Option<String>,
}

#[derive(Deserialize)]
struct CarpetTypeSizeData {
    mobile_no: Option<String>,
	device_registration_token: Option<String>,
}

#[derive(Deserialize)]
struct VehicleCleaningTypeCostData {
    mobile_no: Option<String>,
	device_registration_token: Option<String>,
}

#[derive(Deserialize)]
struct CarpetCleaningTypeCostData {
    mobile_no: Option<String>,
	device_registration_token: Option<String>,
}

#[derive(Deserialize)]
struct CarpetTypeColourData {
    mobile_no: Option<String>,
	device_registration_token: Option<String>,
}

#[derive(Deserialize)]
struct HistorySalesData {
    mobile_no: Option<String>,
	device_registration_token: Option<String>,
}

#[derive(Deserialize)]
struct SearchSalesItems {
    mobile_no: Option<bool>,
	customer_name: Option<bool>,
	vehicle_regno: Option<bool>,
}

#[derive(Deserialize)]
struct SearchHistorySalesData {
    search_data: Option<String>,
	search_by: SearchSalesItems,
}

#[derive(Deserialize)]
struct EmployeesData {
    mobile_no: Option<String>,
	device_registration_token: Option<String>,
}

#[derive(Deserialize)]
struct SalesCommissionData {
    mobile_no: Option<String>,
	device_registration_token: Option<String>,
}

#[derive(Deserialize)]
struct SearchSalesCommissionItems {
    employee_id: Option<bool>,
	employee_full_names: Option<bool>,
}

#[derive(Deserialize)]
struct SearchSalesCommissionData {
    search_data: Option<String>,
	search_by: SearchSalesCommissionItems,
}

#[derive(Deserialize)]
struct InputData {
    sessionId: Option<String>,
	phoneNumber: Option<String>,
	networkCode: Option<String>,
	serviceCode: Option<String>,
	text: Option<String>,
}

enum ProcessingStatus {
	Zero,
	One,
	Two,
}


#[derive(Serialize)]
struct Measurement {
    temperature: f32,
}

#[derive(Serialize)]
struct ResponseData {
    status_code: u32,
	status_description: String,
}

#[derive(Serialize)]
struct ResponseData1 {
    status_code: u32,
	status_description: String,
    person_data: Vec<PersonDetails>,
}

#[derive(Serialize)]
struct PersonDetails {
    username: String,
	location: String,
	beneficiary: BeneficiaryDetails,
	staff_name: String,
	job_level: String,
}

#[derive(Serialize)]
//#[derive(Debug)]
struct BeneficiaryDetails {
    full_name: String,
	relationship: String,
}

#[derive(Serialize)]
struct VehicleMakeResponseData {
	message_data: String,
    status_code: u32,
	status_description: String,
	cost_data: Vec<VehicleCleaningTypeCostDetails>,
}

#[derive(Serialize)]
struct VehicleModelResponseData {
	message_data: String,
    status_code: u32,
	status_description: String,
}

#[derive(Serialize)]
struct CarpetTypeSizeResponseData {
	message_data: String,
    status_code: u32,
	status_description: String,
	cost_data: Vec<CarpetCleaningTypeCostDetails>,
}

#[derive(Serialize)]
struct VehicleCleaningTypeCostResponseData {
    status_code: u32,
	status_description: String,
    cost_data: Vec<VehicleCleaningTypeCostDetails>,
}

#[derive(Serialize)]
struct VehicleCleaningTypeCostDetails {
    cleaning_type_name: String,
	amount: u32,
}

#[derive(Serialize)]
struct CarpetCleaningTypeCostResponseData {
    status_code: u32,
	status_description: String,
    cost_data: Vec<CarpetCleaningTypeCostDetails>,
}

#[derive(Serialize)]
struct CarpetCleaningTypeCostDetails {
    cleaning_size_name: String,
	amount: u32,
}

#[derive(Serialize)]
struct CarpetTypeColourResponseData {
	message_data: String,
    status_code: u32,
	status_description: String,
}

#[derive(Serialize)]
struct HistoryVehicleSalesData {
    vehicle_make: String,
	vehicle_model: String,
	vehicle_regno: String,
	sales_amount: u32,
	payment_mode: String,
	interior_cleaning: bool,
	exterior_cleaning: bool,
	engine_cleaning: bool,
	undercarriage_cleaning: bool,
	transaction_date: String,
}

#[derive(Serialize)]
struct HistoryCarpetSalesData {
    carpet_size: String,
	carpet_colour: String,
	sales_amount: u32,
	payment_mode: String,
	transaction_date: String,
}

#[derive(Serialize)]
struct HistoryCustomerSalesData {
    cust_name: String,
	mobile_no: String,
	//cleaning_service: String,
}

#[derive(Serialize)]
struct HistorySalesResponseData {
	customer_sales_data: HistoryCustomerSalesData,
	carpet_sales_data: Vec<HistoryCarpetSalesData>,
	vehicle_sales_data: Vec<HistoryVehicleSalesData>,
}

#[derive(Serialize)]
struct HistorySalesBatchData {
    batch_no: String,
    sales_data: HistorySalesResponseData,
}

#[derive(Serialize)]
struct HistorySalesBatchResponseData {
    status_code: u32,
	status_description: String,
	sales_batch_data: Vec<HistorySalesBatchData>,
}

#[derive(Serialize)]
struct EmployeeRegisteredDetails {
    full_names: String,
	id: u32,
}

#[derive(Serialize)]
struct EmployeesRegisteredResponseData {
    status_code: u32,
	status_description: String,
	employees_data: Vec<EmployeeRegisteredDetails>,
}

#[derive(Serialize)]
struct SalesCommissionDetails {
	batch_no: u32,
    cleaning_service: String,
	cleaning_service_type: String,
	cleaning_amount: i32,
	commission_percentage: i32,
	commission_amount: i32,
	employee_full_names: String,
	transaction_date: String,
}

#[derive(Serialize)]
struct SalesCommissionResponseData {
    status_code: u32,
	status_description: String,
	sales_commission_data: Vec<SalesCommissionDetails>,
}

#[derive(Debug, PartialEq, Eq)]
struct SalesBatchDataTable {
    batch_no: Option<i32>,
	cust_name: String,
    mobile_no: String,
    cleaning_service: String,
    sales_amount: i32,
	paid_amount: i32,
    payment_mode: String,
}

#[derive(Debug, PartialEq, Eq)]
struct SalesDataTable {
    batch_no: i32,
    cleaning_service: String,
	carpet_size: String,
    carpet_colour: String,
    vehicle_make: String,
    vehicle_model: String,
    vehicle_regno: String,
    interior_cleaning: bool,
    exterior_cleaning: bool,
    engine_cleaning: bool,
    undercarriage_cleaning: bool,
    sales_amount: i32,
	employee_id: i32,
	employee_full_names: String,
}

#[derive(Debug, PartialEq, Eq)]
struct ClientApiResponseDetails {
    status_code: u32,
	status_description: String,
}
/*	
let url = get_conn_url();

let pool = Pool::new(url)?;

let mut conn = pool.get_conn()?;
*/

#[get("/hello")]
async fn hello_world() -> impl Responder {
    "Hello World!"
}

#[get("/temp")]
async fn current_temperature() -> impl Responder {
    web::Json(Measurement { temperature: 42.3 })
}

//async fn get_person(info: web::Json<Info>) -> Result<String> {
/// deserialize `Info` from request's body
#[post("/person")]
async fn get_person(info: web::Json<Info>) -> impl Responder {
	//let user_name: String = String::from(info.username);
	//let user_name: String = info.username.clone();
	//let user_name: &String = &info.username;
	let user_name = &info.username;
	let my_staff_name = &info.posted_by.staff_name;
	let my_job_level = &info.posted_by.job_level;
	let location_name = get_location();
	let my_beneficiary = BeneficiaryDetails { full_name: String::from("Moses Weta"), relationship: String::from("Son") };
	let my_beneficiary1 = BeneficiaryDetails { full_name: String::from("Benta Shiraku"), relationship: String::from("Daughter") };
	let my_beneficiary2 = BeneficiaryDetails { full_name: String::from("Paul Owino"), relationship: String::from("Son") };
    //Ok(format!("Welcome {}!", info.username))
	//web::Json(PersonDetails { username: user_name.to_string(), location: location_name, beneficiary: my_beneficiary, staff_name: my_staff_name.to_string(), job_level: my_job_level.to_string() })
	let mut x = Vec::new();
	let my_person = PersonDetails { username: user_name.to_string(), location: location_name, beneficiary: my_beneficiary, staff_name: my_staff_name.to_string(), job_level: my_job_level.to_string() };
	let my_person1 = PersonDetails { username: String::from("walter"), location: String::from("westlands"), beneficiary: my_beneficiary1, staff_name: my_staff_name.to_string(), job_level: my_job_level.to_string() };
	//let my_person2 = PersonDetails { username: String::from("mary"), location: String::from("ngong"), beneficiary: my_beneficiary2, staff_name: my_staff_name.to_string(), job_level: my_job_level.to_string() };
	let my_person2 = PersonDetails { username: user_name.to_string(), location: String::from("ngong"), beneficiary: my_beneficiary2, staff_name: my_staff_name.to_string(), job_level: my_job_level.to_string() };
	//println!("my_beneficiary borrowed in {:?}", my_beneficiary);
	x.push(my_person);
	x.push(my_person1);
	x.push(my_person2);
	//web::Json(x)
	//let my_response_data = ResponseData { status_code: 0, status_description: String::from("Successful"), person_data: x };
	let my_response_data = ResponseData1 { status_code: ProcessingStatus::Zero as u32, status_description: String::from("Successful"), person_data: x };
	web::Json(my_response_data)
}

fn get_location() -> String {
	let local_name = String::from("Dandora");
	local_name
}

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

/// deserialize `VehicleMakeData` from request's body
#[post("/getvehiclemakedata")]
async fn get_vehicle_make_data(vehicle_make_data: web::Json<VehicleMakeData>, req: HttpRequest) -> impl Responder {
	let k = String::from(""); //Default value for string variables.
	let api_function = String::from("get_vehicle_make_data"); 
	
	let client_api_response = validate_client_api(req, api_function);
	let status_code = client_api_response.status_code;
	let status_description = client_api_response.status_description;
	
	//println!("channel_type - {:?}", channel_type);
	let mobile_no = &vehicle_make_data.mobile_no.as_ref().unwrap_or(&k);
	let vehicle_make = String::from("ALFA ROMEO|ANY|ASHOK|AUDI|BACKHOE|BAJAJ|BEDFORD|BEIBEN|BEIFANG|BHACHU|BMW|BOBCAT|BOMAG|BULLDOZER|BUS|CADILLAC|CAM|CANTER|CASE|CAT|CHEVROLET|CHRYSLER|CITROEN|CMC|CRANE|DAEWOO|DAF|DAIHATSU|DODGE|DOLL|DOZER|DUMPER|EICHER|EXCAVATOR|FAW|FERRARI|FIAT|FORD|FOTON|GEELEY|GRADER|GREATWALL|HAMM|HANS KENYA|HINO|HITACHI|HONDA|HOWO|HUMMER|HYUNDAI|ISUZU|IVECO|JAC|JAGUAR|JCB|JEEP|JMC|JOHN-DEERE|KEHAR|KIA|KLUGER|KOMATSU|LANCER|LANDROVER|LEEBOY|LEXUS|LEYLAND|LEYLANDDAF|LIEBHERR|LOADER|LORRY|M/CYCLE|MACK|MAHINDRA|MAN|MARUTI|MASSEY|MAZDA|MERCEDES|MINI|MITSUBISHI|MIXER|MORRIS|NEWHOLLAND|NIS_DIE|NISSAN|OCEAN|OPEL|PACER|PEUGEOT|PORSCHE|PRIMEMOVER|PUCH|RANDON|RENAULT|ROLLER|ROLLS|ROVER|SAAB|SAILOR|SCANIA|SDLG|SHACMAN|SHOVEL|SINO|SKODA|SONALIKA|SSANG YONG|SUBARU|SUZUKI|TADANO|TANKER|TATA|TEREX|TIGER|TIGGO|TIPPER|TOYOTA|TRACTOR|TRAILER|TRUCK|TUKTUK|TVS|UD|VAUXHALL|VOLKSWAGEN|VOLVO|WUZHENG|XINKAI|YAMAHA|YARI|");
	let mut k = Vec::new();
	let interior_cleaning_name = String::from("interior");
	let exterior_cleaning_name = String::from("exterior");
	let engine_cleaning_name = String::from("engine");
	let under_carriage_cleaning_name = String::from("undercarriage");

	let interior_cleaning_cost = 200;
	let exterior_cleaning_cost = 300;
	let engine_cleaning_cost = 150;
	let under_carriage_cleaning_cost = 210;

	let interior_item = VehicleCleaningTypeCostDetails { cleaning_type_name: interior_cleaning_name, amount: interior_cleaning_cost };
	k.push(interior_item);
	let exterior_item = VehicleCleaningTypeCostDetails { cleaning_type_name: exterior_cleaning_name, amount: exterior_cleaning_cost };
	k.push(exterior_item);
	let engine_item = VehicleCleaningTypeCostDetails { cleaning_type_name: engine_cleaning_name, amount: engine_cleaning_cost };
	k.push(engine_item);
	let under_carriage_item = VehicleCleaningTypeCostDetails { cleaning_type_name: under_carriage_cleaning_name, amount: under_carriage_cleaning_cost };
	k.push(under_carriage_item);
	/*
	let x = String::from(" ");
	let a = format!("{}{}", String::from("mobile_no - "), mobile_no);
	let b = format!("{}{}", String::from("vehicle_make - "), vehicle_make);
	let c = format!("{}{}", String::from("vehicle_cleaning_type_cost - "), k.len().to_string());
	let d = format!("{}{}{}{}{}{}", a, x, b, x, c, x);
	//println!("details is {:?}", d);
	*/
	let response_data = VehicleMakeResponseData {message_data: vehicle_make.to_string(), status_code: ProcessingStatus::Zero as u32, status_description: String::from("Successful"), cost_data: k };
	web::Json(response_data)
}

/// deserialize `VehicleModelData` from request's body
#[post("/getvehiclemodeldata")]
async fn get_vehicle_model_data(vehicle_model_data: web::Json<VehicleModelData>, req: HttpRequest) -> impl Responder {
	let k = String::from(""); //Default value for string variables.
	let api_function = String::from("get_vehicle_model_data"); 
	
	let client_api_response = validate_client_api(req, api_function);
	let status_code = client_api_response.status_code;
	let status_description = client_api_response.status_description;
	
	//println!("channel_type - {:?}", channel_type);
	let mobile_no = &vehicle_model_data.mobile_no.as_ref().unwrap_or(&k);
	//let mut vehicle_make = &vehicle_model_data.vehicle_make.as_ref().unwrap_or(&k);
	let vehicle_make = &vehicle_model_data.vehicle_make.as_ref().unwrap_or(&k);
	//let vehicle_model = String::from("AUDI|AUDI-A2|AUDI-A3|AUDI-A3 SE AUTO|AUDI-A4 AVANT S|AUDI-A4 AVANT W|AUDI-A4 FSI S-L|AUDI-A4 TURBO Q|AUDI-A6 SE AUTO|AUDI-A6 STDI|AUDI-A6 TURBO|AUDI-A6 TURBO S|AUDI-A8 SE TDI|AUDI-AUDI 500|AUDI-Q5|AUDI-Q5 3.2L AU|AUDI-Q7|");
	let mut vehicle_model = String::from("");
	
	//let t = vehicle_make.to_lowercase().eq(String::from("audi"))
	
	let a_1 = String::from("audi");
	let a_2 = String::from("bajaj");
	let a_3 = String::from("bmw");
	
	vehicle_model =
		{
			if vehicle_make.to_lowercase().eq(&a_1) {
				String::from("AUDI|AUDI-A2|AUDI-A3|AUDI-A3 SE AUTO|AUDI-A4 AVANT S|AUDI-A4 AVANT W|AUDI-A4 FSI S-L|AUDI-A4 TURBO Q|AUDI-A6 SE AUTO|AUDI-A6 STDI|AUDI-A6 TURBO|AUDI-A6 TURBO S|AUDI-A8 SE TDI|AUDI-AUDI 500|AUDI-Q5|AUDI-Q5 3.2L AU|AUDI-Q7|")
			}
			else if vehicle_make.to_lowercase().eq(&a_2){
				String::from("BAJAJ BM 150X|BAJAJ QUTE|BAJAJ TUKTUK|")
			}
			else if vehicle_make.to_lowercase().eq(&a_3){
				String::from("BMW|BMW 316I|BMW 5|BMW 650GS|BMW ABA-VA20|BMW IGT|BMW-116D F20SH|BMW-116D N47 U|BMW-116I|BMW-116I E81 N|BMW-116I F20SH|BMW-118D N47 U|BMW-118D- N47|BMW-118I E88 C|BMW-118I N13 F|BMW-118I N46 U|BMW-120D CP|BMW-120D N47 U|BMW-120I E82 C|BMW-120I N46 U|BMW-125I N52 U|BMW-130I MANUA|BMW-130I N52 U|BMW-135I N54 U|BMW-135I N55 U|BMW-316I E90 L|BMW-318I I N46|BMW-318I N46 P|BMW-320D|BMW-320D N47 K|BMW-320ED|BMW-320I|BMW-320I E93 C|BMW-320I N46 P|BMW-325I AUTO|BMW-325I MANUA|BMW-325I N52 D|BMW-325I N52 K|BMW-325I N52 P|BMW-325I N53 C|BMW-330 CI CON|BMW-330D|BMW-330D N57 K|BMW-330I  N52|BMW-330I AUTO|BMW-330I MANUA|BMW-335I|BMW-335I N54 K|BMW-335I N54 P|BMW-335I N55 D|BMW-520D AUTO|BMW-520I|BMW-523I|BMW-525D|BMW-525I|BMW-525I AUTO|BMW-528I AUTO|BMW-530D N57 F|BMW-530I MANUA|BMW-535D N57 S|BMW-535I F07 G|BMW-535I N55 F|BMW-550I N63 F|BMW-550I N63 S|BMW-630I  E63|BMW-630I  E64|BMW-650I  N62|BMW-730D|BMW-730I F02 L|BMW-730IAUTO|BMW-730LD N57|BMW-735I|BMW-740I N54 K|BMW-740LI N54|BMW-745IA|BMW-750I N63 K|BMW-750I XDRIV|BMW-750LI N63|BMW-750LI XDRI|BMW-760I N74 K|BMW-760LI N74|BMW-BMW MINI C|BMW-BMW MOTOR CYCLE|BMW-BMW Z3 ROA|BMW-F650 GS 218|BMW-F800 GS 219|BMW-F800 R 217|BMW-F800 ST 234|BMW-G650 GS SERTA|BMW-G650GS 188|BMW-K1300 R 518|BMW-K1300 S 308|BMW-K1600 GT 601|BMW-K1600 GTL 602|BMW-M3 S65 DX9|BMW-M3 S65 KG9|BMW-M3 S65 PM9|BMW-M6 S85 EH9|BMW-M6 S85 EK9|BMW-R1200 GS 450|BMW-R1200 R 400|BMW-R1200 R GS ADV|BMW-R1200 RT 430|BMW-R1200RT 430|BMW-R900 RT 330|BMW-S1000 RR 524|BMW-X1|BMW-X1 SDRIVEN|BMW-X1 XDRIVEN|BMW-X3|BMW-X3 XDRIVE2|BMW-X3 XDRIVE3|BMW-X3 XDRIVEN|BMW-X3 XRIVE30|BMW-X5|BMW-X5 3.0D|BMW-X5 351|BMW-X5 M S63 G|BMW-X5 XDRIVE5|BMW-X5 XDRIVEN|BMW-X6|BMW-X6 M N63 G|BMW-X6 XDRIVE5|BMW-Z4 E89 ROA|")
			}
			else{
				String::from("")
			}
		};
	
	/*
	let vehicle_model = 
	match vehicle_make {
            String::from("audi") => 
				String::from("AUDI|AUDI-A2|AUDI-A3|AUDI-A3 SE AUTO|AUDI-A4 AVANT S|AUDI-A4 AVANT W|AUDI-A4 FSI S-L|AUDI-A4 TURBO Q|AUDI-A6 SE AUTO|AUDI-A6 STDI|AUDI-A6 TURBO|AUDI-A6 TURBO S|AUDI-A8 SE TDI|AUDI-AUDI 500|AUDI-Q5|AUDI-Q5 3.2L AU|AUDI-Q7|"),
			String::from("toyota") => String::from("toyota"),
            _ => String::from("none"),
        };
	
	match vehicle_make {
            c1 => println!("This is a match 1!"),
			c2 => println!("This is a match 2!"),
            _ => println!("Match failed"),
        }
	*/
	/*
	let x = String::from(" ");
	let a = format!("{}{}", String::from("mobile_no - "), mobile_no);
	let b = format!("{}{}", String::from("vehicle_make - "), vehicle_make);
	let c = format!("{}{}", String::from("vehicle_model - "), vehicle_model);
	let d = format!("{}{}{}{}{}{}", a, x, b, x, c, x);
	println!("details is {:?}", d);
	*/
	let response_data = VehicleModelResponseData {message_data: vehicle_model.to_string(), status_code: ProcessingStatus::Zero as u32, status_description: String::from("Successful")};
	web::Json(response_data)
}

/// deserialize `CarpetTypeSizeData` from request's body
#[post("/getcarpettypesizedata")]
async fn get_carpet_type_size_data(carpet_type_size_data: web::Json<CarpetTypeSizeData>, req: HttpRequest) -> impl Responder {
	let k = String::from(""); //Default value for string variables.
	let api_function = String::from("get_carpet_type_size_data"); 
	
	let client_api_response = validate_client_api(req, api_function);
	let status_code = client_api_response.status_code;
	let status_description = client_api_response.status_description;
	
	//println!("channel_type - {:?}", channel_type);
	let mobile_no = &carpet_type_size_data.mobile_no.as_ref().unwrap_or(&k);
	let carpet_type_size = String::from("CARPET SIZE|5 by 8|6 by 9|7 by 10|8 by 11|");
	let mut k = Vec::new();
	let a_cleaning_size_name = String::from("5by8");
	let b_cleaning_size_name = String::from("6by9");
	let c_cleaning_size_name = String::from("7by10");
	let d_size_cleaning_name = String::from("8by11");
	
	let a_cleaning_size_cost = 600;
	let b_cleaning_size_cost = 700;
	let c_cleaning_size_cost = 800;
	let d_cleaning_size_cost = 900;
	
	let a_item = CarpetCleaningTypeCostDetails { cleaning_size_name: a_cleaning_size_name, amount: a_cleaning_size_cost };
	k.push(a_item);
	let b_item = CarpetCleaningTypeCostDetails { cleaning_size_name: b_cleaning_size_name, amount: b_cleaning_size_cost };
	k.push(b_item);
	let c_item = CarpetCleaningTypeCostDetails { cleaning_size_name: c_cleaning_size_name, amount: c_cleaning_size_cost };
	k.push(c_item);
	let d_item = CarpetCleaningTypeCostDetails { cleaning_size_name: d_size_cleaning_name, amount: d_cleaning_size_cost };
	k.push(d_item);
	/*
	let x = String::from(" ");
	let a = format!("{}{}", String::from("mobile_no - "), mobile_no);
	let b = format!("{}{}", String::from("carpet_type_size - "), carpet_type_size);
	let c = format!("{}{}", String::from("vehicle_cleaning_type_cost - "), k.len().to_string());
	let d = format!("{}{}{}{}{}{}", a, x, b, x, c, x);
	println!("details is {:?}", d);
	*/
	let response_data = CarpetTypeSizeResponseData {message_data: carpet_type_size.to_string(), status_code: ProcessingStatus::Zero as u32, status_description: String::from("Successful"), cost_data: k };
	web::Json(response_data)
}

/// deserialize `CarpetTypeColourData` from request's body
#[post("/getcarpettypecolourdata")]
async fn get_carpet_type_colour_data(carpet_type_colour_data: web::Json<CarpetTypeColourData>, req: HttpRequest) -> impl Responder {
	let k = String::from(""); //Default value for string variables.
	let api_function = String::from("get_carpet_type_colour_data"); 
	
	let client_api_response = validate_client_api(req, api_function);
	let status_code = client_api_response.status_code;
	let status_description = client_api_response.status_description;
	
	//println!("channel_type - {:?}", channel_type);
	let mobile_no = &carpet_type_colour_data.mobile_no.as_ref().unwrap_or(&k);
	let carpet_type_colour = String::from("CARPET COLOUR|WHITE|BLACK|RED|BLUE|YELLOW|ORANGE|PURPLE|GREEN|MIXTURE");
	/*
	let x = String::from(" ");
	let a = format!("{}{}", String::from("mobile_no - "), mobile_no);
	let b = format!("{}{}", String::from("carpet_type_colour - "), carpet_type_colour);
	let c = format!("{}{}{}{}", a, x, b, x);
	println!("details is {:?}", c);
	*/
	let response_data = CarpetTypeColourResponseData {message_data: carpet_type_colour.to_string(), status_code: ProcessingStatus::Zero as u32, status_description: String::from("Successful") };
	web::Json(response_data)
}

/// deserialize `VehicleCleaningTypeCostData` from request's body
#[post("/getvehiclecleaningtypecostdata")]
async fn get_vehicle_cleaning_type_cost_data(vehicle_cleaning_type_cost_data: web::Json<VehicleCleaningTypeCostData>, req: HttpRequest) -> impl Responder {
	let k = String::from(""); //Default value for string variables.
	let api_function = String::from("get_vehicle_cleaning_type_cost_data"); 
	
	let client_api_response = validate_client_api(req, api_function);
	let status_code = client_api_response.status_code;
	let status_description = client_api_response.status_description;
	
	//println!("channel_type - {:?}", channel_type);
	let mobile_no = &vehicle_cleaning_type_cost_data.mobile_no.as_ref().unwrap_or(&k);
	let mut k = Vec::new();
	let interior_cleaning_name = String::from("interior");
	let exterior_cleaning_name = String::from("exterior");
	let engine_cleaning_name = String::from("engine");
	let under_carriage_cleaning_name = String::from("undercarriage");
	
	let interior_cleaning_cost = 200;
	let exterior_cleaning_cost = 300;
	let engine_cleaning_cost = 150;
	let under_carriage_cleaning_cost = 210;
	
	let interior_item = VehicleCleaningTypeCostDetails { cleaning_type_name: interior_cleaning_name, amount: interior_cleaning_cost };
	k.push(interior_item);
	let exterior_item = VehicleCleaningTypeCostDetails { cleaning_type_name: exterior_cleaning_name, amount: exterior_cleaning_cost };
	k.push(exterior_item);
	let engine_item = VehicleCleaningTypeCostDetails { cleaning_type_name: engine_cleaning_name, amount: engine_cleaning_cost };
	k.push(engine_item);
	let under_carriage_item = VehicleCleaningTypeCostDetails { cleaning_type_name: under_carriage_cleaning_name, amount: under_carriage_cleaning_cost };
	k.push(under_carriage_item);
	/*
	let x = String::from(" ");
	let a = format!("{}{}", String::from("mobile_no - "), mobile_no);
	let b = format!("{}{}", String::from("vehicle_cleaning_type_cost - "), k.len().to_string());
	let c = format!("{}{}{}{}", a, x, b, x);
	println!("details is {:?}", c);
	*/
	let response_data = VehicleCleaningTypeCostResponseData { status_code: ProcessingStatus::Zero as u32, status_description: String::from("Successful"), cost_data: k };
	web::Json(response_data)
}

/// deserialize `CarpetCleaningTypeCostData` from request's body
#[post("/getcarpetcleaningtypecostdata")]
async fn get_carpet_cleaning_type_cost_data(carpet_cleaning_type_cost_data: web::Json<CarpetCleaningTypeCostData>, req: HttpRequest) -> impl Responder {
	let k = String::from(""); //Default value for string variables.
	let api_function = String::from("get_carpet_cleaning_type_cost_data"); 
	
	let client_api_response = validate_client_api(req, api_function);
	let status_code = client_api_response.status_code;
	let status_description = client_api_response.status_description;
	
	//println!("channel_type - {:?}", channel_type);
	let mobile_no = &carpet_cleaning_type_cost_data.mobile_no.as_ref().unwrap_or(&k);
	let mut k = Vec::new();
	let a_cleaning_size_name = String::from("5by8");
	let b_cleaning_size_name = String::from("6by9");
	let c_cleaning_size_name = String::from("7by10");
	let d_size_cleaning_name = String::from("8by11");
	
	let a_cleaning_size_cost = 600;
	let b_cleaning_size_cost = 700;
	let c_cleaning_size_cost = 800;
	let d_cleaning_size_cost = 900;
	
	let a_item = CarpetCleaningTypeCostDetails { cleaning_size_name: a_cleaning_size_name, amount: a_cleaning_size_cost };
	k.push(a_item);
	let b_item = CarpetCleaningTypeCostDetails { cleaning_size_name: b_cleaning_size_name, amount: b_cleaning_size_cost };
	k.push(b_item);
	let c_item = CarpetCleaningTypeCostDetails { cleaning_size_name: c_cleaning_size_name, amount: c_cleaning_size_cost };
	k.push(c_item);
	let d_item = CarpetCleaningTypeCostDetails { cleaning_size_name: d_size_cleaning_name, amount: d_cleaning_size_cost };
	k.push(d_item);
	/*
	let x = String::from(" ");
	let a = format!("{}{}", String::from("mobile_no - "), mobile_no);
	let b = format!("{}{}", String::from("carpet_cleaning_type_cost - "), k.len().to_string());
	let c = format!("{}{}{}{}", a, x, b, x);
	println!("details is {:?}", c);
	*/
	let response_data = CarpetCleaningTypeCostResponseData { status_code: ProcessingStatus::Zero as u32, status_description: String::from("Successful"), cost_data: k };
	web::Json(response_data)
}

/// deserialize `SalesBatchData` from request's body
#[post("/addsalesdata")]
async fn add_sales_data(sales_batch_data: web::Json<SalesBatchData>, req: HttpRequest, data: web::Data<Pool>) -> impl Responder {
	let k = String::from(""); //Default value for string variables.
	let api_function = String::from("add_sales_data"); 
	
	let client_api_response = validate_client_api(req, api_function);
	let status_code = client_api_response.status_code;
	let status_description = client_api_response.status_description;
	
	//println!("channel_type - {:?}", channel_type);
	let batch_no = &sales_batch_data.batch_no.as_ref().unwrap_or(&k);
	let sales_batch_data = &sales_batch_data.sales_data;
	
	let sales_batch_data_table = get_sales_batch_data(sales_batch_data);

	let batch_no: i32 = create_sales_batch_data(&data, sales_batch_data_table);
	
	let sales_data_table = get_sales_data(sales_batch_data, batch_no);
	let successful: bool = create_sales_data(&data, sales_data_table);
	//let successful_1: bool = create_sales_commission_data(data, batch_no, employee_id, employee_full_names);
	let successful_1: bool = create_sales_commission_data(data, batch_no);
	
	let response_data = ResponseData { status_code: ProcessingStatus::Zero as u32, status_description: String::from("Successful")};
	web::Json(response_data)
}

/// deserialize `HistorySalesData` from request's body
#[post("/getallsalesdata")]
async fn get_all_sales_data(history_sales_data: web::Json<HistorySalesData>, req: HttpRequest, data: web::Data<Pool>) -> impl Responder {
	let k = String::from(""); //Default value for string variables.
	let api_function = String::from("get_all_sales_data"); 
	
	let client_api_response = validate_client_api(req, api_function);
	let status_code = client_api_response.status_code;
	let status_description = client_api_response.status_description;
	
	let response_data = get_history_sales_batch_data(&data);
	web::Json(response_data)
}

/// deserialize `SearchHistorySalesData` from request's body
#[post("/getsearchsalesdata")]
async fn get_search_sales_data(search_history_sales_data: web::Json<SearchHistorySalesData>, req: HttpRequest, data: web::Data<Pool>) -> impl Responder {
	let k = String::from(""); //Default value for string variables.
	let j: bool = false;
	let api_function = String::from("get_search_sales_data"); 
	
	let client_api_response = validate_client_api(req, api_function);
	let status_code = client_api_response.status_code;
	let status_description = client_api_response.status_description;
	
	let search_data = &search_history_sales_data.search_data.as_ref().unwrap_or(&k);
	let search_by_key = &search_history_sales_data.search_by;
	
	let is_mobile_no = &search_by_key.mobile_no.as_ref().unwrap_or(&j);
	let is_customer_name = &search_by_key.customer_name.as_ref().unwrap_or(&j);
	let is_vehicle_regno = &search_by_key.vehicle_regno.as_ref().unwrap_or(&j);
		
	let response_data = get_history_search_sales_batch_data(search_data, is_mobile_no, is_customer_name, is_vehicle_regno, &data);
	web::Json(response_data)
}

/// deserialize `EmployeesData` from request's body
#[post("/getallemployeesdata")]
async fn get_all_employees_data(employees_data: web::Json<EmployeesData>, req: HttpRequest, data: web::Data<Pool>) -> impl Responder {
	//let k = String::from(""); //Default value for string variables.
	let api_function = String::from("get_all_employees_data"); 
	
	let client_api_response = validate_client_api(req, api_function);
	let status_code = client_api_response.status_code;
	let status_description = client_api_response.status_description;
	
	let response_data = get_employees_registered_data(&data);
	web::Json(response_data)
}

/// deserialize `SalesCommissionData` from request's body
#[post("/getallsalescommissiondata")]
async fn get_all_sales_commission_data(sales_commission_data: web::Json<SalesCommissionData>, req: HttpRequest, data: web::Data<Pool>) -> impl Responder {
	//let k = String::from(""); //Default value for string variables.
	let api_function = String::from("get_all_sales_commission_data"); 
	
	let client_api_response = validate_client_api(req, api_function);
	let status_code = client_api_response.status_code;
	let status_description = client_api_response.status_description;
	
	let response_data = get_sales_commission_data(&data);
	web::Json(response_data)
}

/// 

#[post("/getsearchsalescommissiondata")]
async fn get_search_sales_commission_data(search_sales_commission_data: web::Json<SearchSalesCommissionData>, req: HttpRequest, data: web::Data<Pool>) -> impl Responder {
	let k = String::from(""); //Default value for string variables.
	let j: bool = false;
	let api_function = String::from("get_search_sales_commission_data"); 
	
	let client_api_response = validate_client_api(req, api_function);
	let status_code = client_api_response.status_code;
	let status_description = client_api_response.status_description;
	
	println!("get_search_sales_commission_data: status_code - {:?}", status_code);
	println!("get_search_sales_commission_data: status_description - {:?}", status_description);
	
	let search_data = &search_sales_commission_data.search_data.as_ref().unwrap_or(&k);
	let search_by_key = &search_sales_commission_data.search_by;
	
	let is_employee_id = &search_by_key.employee_id.as_ref().unwrap_or(&j);
	let is_employee_full_names = &search_by_key.employee_full_names.as_ref().unwrap_or(&j);
		
	let response_data = get_search_entry_sales_commission_data(search_data, is_employee_id, is_employee_full_names, &data);
	web::Json(response_data)
}

// This handler is only called if:
// - request headers declare the content type as `application/x-www-form-urlencoded`
// - request payload is deserialized into a `InputData` struct from the URL encoded format
#[post("/processussdactions")]
async fn process_ussd_actions(form: web::Form<InputData>, data: web::Data<Pool>) -> String {
	let k = String::from(""); //Default value for string variables.
	let session_id = &form.sessionId.as_ref().unwrap_or(&k);
	let phone_number = &form.phoneNumber.as_ref().unwrap_or(&k);
	let network_code = &form.networkCode.as_ref().unwrap_or(&k);
	let service_code = &form.serviceCode.as_ref().unwrap_or(&k);
	let text = &form.text.as_ref().unwrap_or(&k);
	
	let phone_number = phone_number.replace("+","");
	
	let is_registered = get_ussd_registered_client(&phone_number);
	
	if is_registered {
		let response_data = process_client_requests(&data, session_id, &phone_number, service_code, text);
		
		return response_data
	}
	else {
		let response_data = process_unregistered_client_requests(session_id, &phone_number, service_code, text);
		
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
	//menu_data.push_str("\n");
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
	let menu_1 = String::from("Please enter your National ID number to proceed.");
		
	menu_data.push_str(&welcome_message_1);
	menu_data.push_str("\n");
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
	else if text.contains("*") == false {
		let mut sub_menu_data = String::from("");
		let sub_menu_1 = String::from("Enter Tenant Code");
		//let sub_menu_2 = String::from("Enter 00 to go to previous Menu");
		let sub_menu_2 = String::from("0:Back 00:Home");
		
		sub_menu_data.push_str(&sub_menu_1);
		sub_menu_data.push_str("\n");
		sub_menu_data.push_str(&sub_menu_2);
		
		let sub_menu_data = generate_ussd_response_message(&sub_menu_data, true);

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
						
						let sub_menu_data = generate_ussd_response_message(&sub_menu_data, true);

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
						
						let sub_menu_data = generate_ussd_response_message(&sub_menu_data, true);

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
							let response_data = "Please note that you entered invalid amount.";
							let response_data = generate_ussd_response_message(&response_data.to_string(), false);
			
							return response_data.to_string()
						}
						
						let is_valid = validate_minimum_amount(&amount);
						
						if !is_valid {
							let response_data = "Please note that the minimum amount is Ksh 100.";
							let response_data = generate_ussd_response_message(&response_data.to_string(), false);
			
							return response_data.to_string()
						}
						
						let is_valid = validate_maximum_amount(&amount);
						
						if !is_valid {
							let response_data = "Please note that the maximum amount is Ksh 15,000.";
							let response_data = generate_ussd_response_message(&response_data.to_string(), false);
			
							return response_data.to_string()
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
						
						let sub_menu_data = generate_ussd_response_message(&sub_menu_data, true);

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
						
						let mut sub_menu_data = String::from("");
						
						if input.eq("1") {
							sub_menu_data = String::from("Please note that you you cancelled the request.");
							let sub_menu_data = generate_ussd_response_message(&sub_menu_data, false);
							return sub_menu_data.to_string()
						}
						else if input.eq("2") {
							let mut sub_menu_data = String::from("");
							let sub_menu_1 = String::from("Enter PIN");
							
							sub_menu_data.push_str(&sub_menu_1);
							
							let sub_menu_data = generate_ussd_response_message(&sub_menu_data, true);

							return sub_menu_data.to_string()
						}
						else {
							sub_menu_data = wrong_selection_data.to_string();
							let sub_menu_data = generate_ussd_response_message(&sub_menu_data, false);
							
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
							
							create_okoa_rent_data(data, tenant_code, house_code, amount, mobile_no.to_string());
							
							sub_menu_data.push_str(&sub_menu_1);
							sub_menu_data.push_str("\n");
							sub_menu_data.push_str(&sub_menu_2);
							sub_menu_data.push_str("\n");
							sub_menu_data.push_str(&sub_menu_3);
							
							let sub_menu_data = generate_ussd_response_message(&sub_menu_data, false);
							
							return sub_menu_data.to_string()
						}
						else {
							sub_menu_data = String::from("Please note that you entered an invalid PIN.");
							let sub_menu_data = generate_ussd_response_message(&sub_menu_data, false);
							
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
	else if text.contains("*") == false {
		let mut sub_menu_data = String::from("");
		let sub_menu_1 = String::from("Enter Mortgagor Code");
		//let sub_menu_2 = String::from("Enter 00 to go to previous Menu");
		let sub_menu_2 = String::from("0:Back 00:Home");
		
		sub_menu_data.push_str(&sub_menu_1);
		sub_menu_data.push_str("\n");
		sub_menu_data.push_str(&sub_menu_2);
		
		let sub_menu_data = generate_ussd_response_message(&sub_menu_data, true);

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
						
						let sub_menu_data = generate_ussd_response_message(&sub_menu_data, true);

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
						
						let sub_menu_data = generate_ussd_response_message(&sub_menu_data, true);

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
							let response_data = "Please note that you entered invalid amount.";
							let response_data = generate_ussd_response_message(&response_data.to_string(), false);
			
							return response_data.to_string()
						}
						
						let is_valid = validate_minimum_amount(&amount);
						
						if !is_valid {
							let response_data = "Please note that the minimum amount is Ksh 100.";
							let response_data = generate_ussd_response_message(&response_data.to_string(), false);
			
							return response_data.to_string()
						}
						
						let is_valid = validate_maximum_amount(&amount);
						
						if !is_valid {
							let response_data = "Please note that the maximum amount is Ksh 15,000.";
							let response_data = generate_ussd_response_message(&response_data.to_string(), false);
			
							return response_data.to_string()
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
						
						let sub_menu_data = generate_ussd_response_message(&sub_menu_data, true);

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
							let sub_menu_data = generate_ussd_response_message(&sub_menu_data, false);
							return sub_menu_data.to_string()
						}
						else if input.eq("2") {
							let mut sub_menu_data = String::from("");
							let sub_menu_1 = String::from("Enter PIN");
							
							sub_menu_data.push_str(&sub_menu_1);
							
							let sub_menu_data = generate_ussd_response_message(&sub_menu_data, true);

							return sub_menu_data.to_string()
						}
						else {
							sub_menu_data = wrong_selection_data.to_string();
							let sub_menu_data = generate_ussd_response_message(&sub_menu_data, false);
							
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
							
							create_okoa_mortgage_data(data, mortgagor_code, house_code, amount, mobile_no.to_string());
							
							sub_menu_data.push_str(&sub_menu_1);
							sub_menu_data.push_str("\n");
							sub_menu_data.push_str(&sub_menu_2);
							sub_menu_data.push_str("\n");
							sub_menu_data.push_str(&sub_menu_3);
							
							let sub_menu_data = generate_ussd_response_message(&sub_menu_data, false);
							
							return sub_menu_data.to_string()
						}
						else {
							sub_menu_data = String::from("Please note that you entered an invalid PIN.");
							let sub_menu_data = generate_ussd_response_message(&sub_menu_data, false);
							
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
	else if text.contains("*") == false {
		let mut sub_menu_data = String::from("");
		let sub_menu_1 = String::from("Enter the OKOA RENT\\MORTGAGE Number");
		//let sub_menu_2 = String::from("Enter 00 to go to previous Menu");
		let sub_menu_2 = String::from("0:Back 00:Home");
		
		sub_menu_data.push_str(&sub_menu_1);
		sub_menu_data.push_str("\n");
		sub_menu_data.push_str(&sub_menu_2);
		
		let sub_menu_data = generate_ussd_response_message(&sub_menu_data, true);

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
						
						let sub_menu_data = generate_ussd_response_message(&sub_menu_data, true);

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
							let sub_menu_data = generate_ussd_response_message(&sub_menu_data, false);
							return sub_menu_data.to_string()
						}
						else if input.eq("2") {
							let mut sub_menu_data = String::from("");
							let sub_menu_1 = String::from("Enter PIN");
							
							sub_menu_data.push_str(&sub_menu_1);
							
							let sub_menu_data = generate_ussd_response_message(&sub_menu_data, true);

							return sub_menu_data.to_string()
						}
						else {
							sub_menu_data = wrong_selection_data.to_string();
							let sub_menu_data = generate_ussd_response_message(&sub_menu_data, false);
							
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
							let sub_menu_5 = String::from("0:Back 00:Home");
							
							let rent_mortgage_code = String::from(v[1]);
							
							create_check_balance_data(data, rent_mortgage_code, mobile_no.to_string());
							
							sub_menu_data.push_str(&sub_menu_1);
							sub_menu_data.push_str(&sub_menu_2);
							sub_menu_data.push_str(&sub_menu_3);
							sub_menu_data.push_str(&sub_menu_4);
							sub_menu_data.push_str("\n");
							sub_menu_data.push_str(&sub_menu_5);
							
							let sub_menu_data = generate_ussd_response_message(&sub_menu_data, true);
							
							return sub_menu_data.to_string()
						}
						else {
							sub_menu_data = String::from("Please note that you entered an invalid PIN.");
							let sub_menu_data = generate_ussd_response_message(&sub_menu_data, false);
							
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
	else if text.contains("*") == false {
		let mut sub_menu_data = String::from("");
		let sub_menu_1 = String::from("Enter the OKOA RENT\\MORTGAGE Number");
		//let sub_menu_2 = String::from("Enter 00 to go to previous Menu");
		let sub_menu_2 = String::from("0:Back 00:Home");
		
		sub_menu_data.push_str(&sub_menu_1);
		sub_menu_data.push_str("\n");
		sub_menu_data.push_str(&sub_menu_2);
		
		let sub_menu_data = generate_ussd_response_message(&sub_menu_data, true);

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
						
						let sub_menu_data = generate_ussd_response_message(&sub_menu_data, true);

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
							let response_data = "Please note that you entered invalid amount.";
							let response_data = generate_ussd_response_message(&response_data.to_string(), false);
			
							return response_data.to_string()
						}
						
						let is_valid = validate_minimum_amount(&amount);
						
						if !is_valid {
							let response_data = "Please note that the minimum amount is Ksh 100.";
							let response_data = generate_ussd_response_message(&response_data.to_string(), false);
			
							return response_data.to_string()
						}
						
						let is_valid = validate_maximum_amount(&amount);
						
						if !is_valid {
							let response_data = "Please note that the maximum amount is Ksh 15,000.";
							let response_data = generate_ussd_response_message(&response_data.to_string(), false);
			
							return response_data.to_string()
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
						
						let sub_menu_data = generate_ussd_response_message(&sub_menu_data, true);

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
						
						let mut sub_menu_data = String::from("");
						
						if input.eq("1") {
							sub_menu_data = String::from("Please note that you you cancelled the request.");
							let sub_menu_data = generate_ussd_response_message(&sub_menu_data, false);
							return sub_menu_data.to_string()
						}
						else if input.eq("2") {
							let mut sub_menu_data = String::from("");
							let sub_menu_1 = String::from("Enter PIN");
							
							sub_menu_data.push_str(&sub_menu_1);
							
							let sub_menu_data = generate_ussd_response_message(&sub_menu_data, true);

							return sub_menu_data.to_string()
						}
						else {
							sub_menu_data = wrong_selection_data.to_string();
							let sub_menu_data = generate_ussd_response_message(&sub_menu_data, false);
							
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
							
							create_pay_back_data(data, rent_mortgage_code, amount, mobile_no.to_string());
							
							sub_menu_data.push_str(&sub_menu_1);
							sub_menu_data.push_str("\n");
							sub_menu_data.push_str(&sub_menu_2);
							
							let sub_menu_data = generate_ussd_response_message(&sub_menu_data, false);
							
							return sub_menu_data.to_string()
						}
						else {
							sub_menu_data = String::from("Please note that you entered an invalid PIN.");
							let sub_menu_data = generate_ussd_response_message(&sub_menu_data, false);
							
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
	else if text.contains("*") == false {
		let mut sub_menu_data = String::from("");
		let sub_menu_1 = String::from("Please select type of statement.");
		let sub_menu_2 = String::from("1. Full Statement");
		let sub_menu_3 = String::from("2. Mini Statement");
		
		sub_menu_data.push_str(&sub_menu_1);
		sub_menu_data.push_str("\n");
		sub_menu_data.push_str(&sub_menu_2);
		sub_menu_data.push_str("\n");
		sub_menu_data.push_str(&sub_menu_3);
		
		let sub_menu_data = generate_ussd_response_message(&sub_menu_data, true);

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
							let sub_menu_data = generate_ussd_response_message(&sub_menu_data, false);
							
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
						
						let sub_menu_data = generate_ussd_response_message(&sub_menu_data, true);

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
							let sub_menu_data = generate_ussd_response_message(&sub_menu_data, false);
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
							
							create_get_statement_data(data, full_statement, mini_statement, mobile_no.to_string());
							
							sub_menu_data.push_str(&sub_menu_1);
							sub_menu_data.push_str("\n");
							sub_menu_data.push_str(&sub_menu_2);
							sub_menu_data.push_str("\n");
							sub_menu_data.push_str(&sub_menu_3);
							
							let sub_menu_data = generate_ussd_response_message(&sub_menu_data, false);
							
							return sub_menu_data.to_string()
						}
						else {
							sub_menu_data = wrong_selection_data.to_string();
							let sub_menu_data = generate_ussd_response_message(&sub_menu_data, false);
							
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

fn get_ussd_registered_client(phone_number: &String) -> bool {
	let mut is_registered: bool = false;
	let a_1 = String::from("254723083761");
	
	if phone_number.replace(" ","").len() == 0 {return is_registered}
	
	//println!("get_ussd_registered_client: is_registered 1 - {:?}", is_registered.to_string());
	
	if phone_number.to_lowercase().eq(&a_1) {
		is_registered = true;
	}
	
	//println!("get_ussd_registered_client: is_registered 2 - {:?}", is_registered.to_string());
	
	is_registered
}

fn process_client_requests(data: &web::Data<Pool>, session_id: &String, phone_number: &String, service_code: &String, text: &String) -> String {
	
	let text = text.replace("'","");
	let text = text.replace("--","");
	let text = text.replace(" ","");
	//let caller_action: String = create_ussd_session_details_2(data, session_id.to_string(), phone_number.to_string(), text.to_string());
	create_ussd_session_details(data, session_id.to_string(), phone_number.to_string(), text.to_string());
	let caller_action: String = get_ussd_session_details(data, &session_id, &phone_number);
	
	let caller_action = caller_action.replace(" ","");
	
	if caller_action.replace(" ","").len() == 0 {
		let response_data = get_menu_data();
		let response_data = generate_ussd_response_message(&response_data, true);
		
		return response_data
	}
	else {
		//Lets process sub menu selection by user
		//let response_data = get_sub_menu_data(text);
		let response_data = get_sub_menu_data(&data, &phone_number, &caller_action);
		
		return response_data
	}
}

fn process_unregistered_client_requests(session_id: &String, phone_number: &String, service_code: &String, text: &String) -> String {
	
	if text.replace(" ","").len() == 0 {
		let response_data = get_menu_data_unregistered_client();
		let response_data = generate_ussd_response_message(&response_data, true);
		
		return response_data
	}
	else {
		//Lets process menu selection by user
		let response_data = String::from("Please select submenu to proceed");
		
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

fn get_ussd_session_details(data: &web::Data<Pool>, session_id: &String, phone_number: &String) -> String  {
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

fn get_carpet_sales_data_1() -> HistoryCarpetSalesData {
	let carpet_size: String = String::from("6 by 9");
	let carpet_colour: String = String::from("PURPLE");
	let carpet_sales_amount = 120;
	let carpet_payment_mode: String = String::from("m-pesa");
	let carpet_transaction_date: String = String::from("10-03-2021, 07:29 pm");
	let carpet_sales_data = HistoryCarpetSalesData { carpet_size: carpet_size, carpet_colour: carpet_colour, sales_amount: carpet_sales_amount, payment_mode: carpet_payment_mode, transaction_date: carpet_transaction_date };
	carpet_sales_data
}
fn get_carpet_sales_data_2() -> HistoryCarpetSalesData {
	let carpet_size: String = String::from("5 by 8");
	let carpet_colour: String = String::from("BLUE");
	let carpet_sales_amount = 130;
	let carpet_payment_mode: String = String::from("cash");
	let carpet_transaction_date: String = String::from("12-03-2021, 02:15 pm");
	let carpet_sales_data = HistoryCarpetSalesData { carpet_size: carpet_size, carpet_colour: carpet_colour, sales_amount: carpet_sales_amount, payment_mode: carpet_payment_mode, transaction_date: carpet_transaction_date };
	carpet_sales_data
}
fn get_vehicle_sales_data_1() -> HistoryVehicleSalesData {
	let vehicle_make: String = String::from("BMW");
	let vehicle_model: String = String::from("BMW 316I");
	let vehicle_regno: String = String::from("KAB 123X");
	let vehicle_sales_amount = 350;
	let vehicle_payment_mode: String = String::from("cash");
	let interior_cleaning: bool = true;
	let exterior_cleaning: bool = false;
	let engine_cleaning: bool = true;
	let undercarriage_cleaning: bool = false;
	let vehicle_transaction_date: String = String::from("12-03-2021, 01:00 pm");
	let vehicle_sales_data = HistoryVehicleSalesData { vehicle_make: vehicle_make, vehicle_model: vehicle_model, vehicle_regno: vehicle_regno, sales_amount: vehicle_sales_amount, payment_mode: vehicle_payment_mode, interior_cleaning: interior_cleaning, exterior_cleaning: exterior_cleaning, engine_cleaning: engine_cleaning, undercarriage_cleaning: undercarriage_cleaning, transaction_date: vehicle_transaction_date };
	vehicle_sales_data
}
fn get_vehicle_sales_data_2() -> HistoryVehicleSalesData {
	let vehicle_make: String = String::from("AUDI");
	let vehicle_model: String = String::from("AUDI-A3");
	let vehicle_regno: String = String::from("KAC 003V");
	let vehicle_sales_amount = 340;
	let vehicle_payment_mode: String = String::from("m-pesa");
	let interior_cleaning: bool = false;
	let exterior_cleaning: bool = true;
	let engine_cleaning: bool = false;
	let undercarriage_cleaning: bool = true;
	let vehicle_transaction_date: String = String::from("12-03-2021, 03:00 pm");
	let vehicle_sales_data = HistoryVehicleSalesData { vehicle_make: vehicle_make, vehicle_model: vehicle_model, vehicle_regno: vehicle_regno, sales_amount: vehicle_sales_amount, payment_mode: vehicle_payment_mode, interior_cleaning: interior_cleaning, exterior_cleaning: exterior_cleaning, engine_cleaning: engine_cleaning, undercarriage_cleaning: undercarriage_cleaning, transaction_date: vehicle_transaction_date };
	vehicle_sales_data
}
fn get_customer_sales_data_1() -> HistoryCustomerSalesData {
	let cust_name: String = String::from("nicole");
	let mobile_no: String = String::from("254723083761");
	let customer_sales_data = HistoryCustomerSalesData { cust_name: cust_name, mobile_no: mobile_no };
	customer_sales_data
}
fn get_customer_sales_data_2() -> HistoryCustomerSalesData {
	let cust_name: String = String::from("paul");
	let mobile_no: String = String::from("254723083760");
	let customer_sales_data = HistoryCustomerSalesData { cust_name: cust_name, mobile_no: mobile_no };
	customer_sales_data
}

fn get_conn_url() -> String {
	let url = "mysql://ussd:arunga@2030!@localhost:3306/okoa_rent";
	//let url = "mysql://app1:23$)W.@9smtf!qp7@localhost:3306/okoa_rent"; cloud server
	String::from(url)
}

fn create_sales_batch_data(data: &web::Data<Pool>, sales_batch_data: SalesBatchDataTable) -> i32  {
	let mut batch_no: i32 = 0;
	
	match data
        .get_conn()
		.and_then(|mut conn| insert_sales_batch_data(&mut conn, sales_batch_data))
    {
        Ok(sales_batch_no) => {
            //println!("Successful to open DB connection."),
			//println!("Successful insert to DB connection. {:?}", sales_batch_id);
			batch_no = sales_batch_no as i32;
        },
        Err(e) => println!("Failed to open DB connection. {:?}", e),
    }
	
	batch_no
}

fn create_sales_data(data: &web::Data<Pool>, sales_data: Vec<SalesDataTable>) -> bool {
	let mut successful: bool = false;
	
	match data
        .get_conn()
		.and_then(|mut conn| insert_sales_data(&mut conn, sales_data))
    {
        Ok(sales_no) => {
            //println!("Successful to open DB connection."),
			//println!("Successful insert to DB connection. {:?}", sales_no);
			successful = true;
        },
        Err(e) => println!("Failed to open DB connection. {:?}", e),
    }
	
	successful
}

fn insert_sales_batch_data(
    conn: &mut PooledConn, sales_batch_data: SalesBatchDataTable) -> std::result::Result<u64, mysql::error::Error> {
	
	//let mut batch_no: i32 = 0;
	
	// Now let's insert sales batch data to the database
	//let my_result =
	conn.exec_drop(
        "insert into incomingsalesbatchdatarequests (cust_name, mobile_no, cleaning_service, sales_amount, paid_amount, payment_mode) values (:cust_name, :mobile_no, :cleaning_service, :sales_amount, :paid_amount, :payment_mode);",
        params! {
            "cust_name" => sales_batch_data.cust_name,
            "mobile_no" => sales_batch_data.mobile_no,
            "cleaning_service" => sales_batch_data.cleaning_service,
            "sales_amount" => sales_batch_data.sales_amount,
			"paid_amount" => sales_batch_data.paid_amount,
			"payment_mode" => sales_batch_data.payment_mode,
        },
    )
	.and_then(|_| Ok(conn.last_insert_id()))
	/*
	let batch_no: i32 =
		match my_result
		{
			Ok(s) => {
				//batch_no = i32::try_from(s);
				s as i32
			},
			Err(e) => {
				//batch_no = i32::try_from(s);
				0
			},
		};
	
	batch_no
	*/
}

fn insert_sales_data(
    conn: &mut PooledConn, sales_data: Vec<SalesDataTable>) -> std::result::Result<u64, mysql::error::Error> {
	
	// Now let's insert sales data to the database
	conn.exec_batch(
		r"insert into incomingsalesdatarequests (batch_no, cleaning_service, carpet_size, carpet_colour, vehicle_make, vehicle_model, vehicle_regno, interior_cleaning, exterior_cleaning, engine_cleaning, undercarriage_cleaning, sales_amount, employee_id, employee_full_names)
		  values (:batch_no, :cleaning_service, :carpet_size, :carpet_colour, :vehicle_make, :vehicle_model, :vehicle_regno, :interior_cleaning, :exterior_cleaning, :engine_cleaning, :undercarriage_cleaning, :sales_amount, :employee_id, :employee_full_names);",
		sales_data.iter().map(|s| params! {
			"batch_no" => s.batch_no,
			"cleaning_service" => &s.cleaning_service,
			"carpet_size" => &s.carpet_size,
			"carpet_colour" => &s.carpet_colour,
			"vehicle_make" => &s.vehicle_make,
			"vehicle_model" => &s.vehicle_model,
			"vehicle_regno" => &s.vehicle_regno,
			"interior_cleaning" => s.interior_cleaning,
			"exterior_cleaning" => s.exterior_cleaning,
			"engine_cleaning" => s.engine_cleaning,
			"undercarriage_cleaning" => s.undercarriage_cleaning,
			"sales_amount" => s.sales_amount,
			"employee_id" => s.employee_id,
			"employee_full_names" => &s.employee_full_names,
		})
	)
	.and_then(|_| Ok(1))
	
}

fn insert_sales_commission_data(
    conn: &mut PooledConn, batch_no: i32) -> std::result::Result<u64, mysql::error::Error> {
	
	//let mut batch_no: i32 = 0;
	//employee_id: i32, employee_full_names: String
	
	// Now let's insert sales commission data to the database
	//"call insertsalescommissiondetails (:mybatch_no, :myemployee_id, :myemployee_full_names);",
	conn.exec_drop(
        "call insertsalescommissiondetails (:mybatch_no);",
        params! {
            "mybatch_no" => batch_no,
            //"myemployee_id" => employee_id,
            //"myemployee_full_names" => employee_full_names,
        },
    )
	.and_then(|_| Ok(1))
}

fn insert_ussd_session_details(
    conn: &mut PooledConn, session_id: String, caller_number: String, caller_action: String) -> std::result::Result<u64, mysql::error::Error> {
	
	let is_active: i32 = 1;

	conn.exec_drop(
        "call insertussdsessiondetails_test_5 (:mysessionid, :myisactive, :mycallernumber, :mycalleraction);",
        params! {
            "mysessionid" => session_id,
            "myisactive" => is_active,
            "mycallernumber" => caller_number,
			"mycalleraction" => caller_action,
        },
    )
	.and_then(|_| Ok(1))
}

fn insert_ussd_session_details_2(
    conn: &mut PooledConn, session_id: String, caller_number: String, caller_action: String) -> std::result::Result<String, mysql::error::Error> {
	
	let is_active: i32 = 1;
	let mut caller_action_new: String = String::from("");
	
	println!("insert_ussd_session_details_2. caller_action - {:?}", &caller_action);
	
	//"call insertussdsessiondetails_test_4 (:mysessionid, :myisactive, :mycallernumber, :mycalleraction, :mycalleraction_new);",
	//conn.exec_drop(
	conn.exec_map(
        "call insertussdsessiondetails_test_4 (:mysessionid, :myisactive, :mycallernumber, :mycalleraction, :mycalleraction_new);",
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

fn create_sales_commission_data(data: web::Data<Pool>, batch_no: i32) -> bool {
	let mut successful: bool = false;
	
	match data
        .get_conn()
		.and_then(|mut conn| insert_sales_commission_data(&mut conn, batch_no))
    {
        Ok(sales_no) => {
			successful = true;
        },
        Err(e) => println!("Failed to open DB connection. {:?}", e),
    }
	
	successful
}

fn create_ussd_session_details(data: &web::Data<Pool>, session_id: String, caller_number: String, caller_action: String) -> bool {
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

fn create_okoa_rent_data(data: &web::Data<Pool>, tenant_code: String, house_code: String, amount: i32, mobile_no: String) -> bool  {
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

fn create_okoa_mortgage_data(data: &web::Data<Pool>, mortgagor_code: String, house_code: String, amount: i32, mobile_no: String) -> bool  {
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

fn create_check_balance_data(data: &web::Data<Pool>, rent_mortgage_code: String, mobile_no: String) -> bool  {
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

fn create_get_statement_data(data: &web::Data<Pool>, full_statement: bool, mini_statement: bool, mobile_no: String) -> bool  {
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

fn create_pay_back_data(data: &web::Data<Pool>, rent_mortgage_code: String, amount: i32, mobile_no: String) -> bool  {
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

fn select_ussd_session_details(
    conn: &mut PooledConn, session_id: String, caller_number: String) -> std::result::Result<String, mysql::error::Error> {
	let mut caller_action_new: String = String::from("");
	
    conn.exec_map(
        "select coalesce(calleraction,'') as calleraction from ussdsessiondetails where coalesce(sessionid,'') = :sessionid and coalesce(callernumber,'') = :callernumber and coalesce(duplicateentry,0) = 0 and coalesce(deleted,0) = 0 limit 1;",
		params! {
				"sessionid" => session_id,
				"callernumber" => caller_number,
			},
        |(calleraction)| {
            caller_action_new = calleraction;
        },
    )
	.and_then(|_| Ok(1));
	
	/*
	conn.exec_map(
	"select coalesce(calleraction,'') as calleractionnew from ussdsessiondetails where coalesce(sessionid,'') = :sessionid and coalesce(callernumber,'') = :callernumber and coalesce(duplicateentry,0) = 0 and coalesce(deleted,0) = 0 limit 1;",
	params! {
			"sessionid" => session_id,
			"callernumber" => caller_number,
		},
	|(calleractionnew)| { 
			caller_action_new = calleractionnew;
		},
	)
	.and_then(|_| Ok(1));
	*/
	
	Ok(caller_action_new)
	
}

fn select_incoming_sales_batch_data_requests(
    conn: &mut PooledConn) -> std::result::Result<Vec<HistorySalesBatchData>, mysql::error::Error> {
	let mut sales_batch_data = Vec::new();
	
	//let selected_data: Vec<SalesBatchDataTable> = conn
    conn.query_map(
        "select batch_no, cust_name, mobile_no, cleaning_service, sales_amount, paid_amount, payment_mode from incomingsalesbatchdatarequests order by batch_no desc limit 10;",
        |(batch_no, cust_name, mobile_no, cleaning_service, sales_amount, paid_amount, payment_mode)| {
            let a = SalesBatchDataTable { batch_no, cust_name, mobile_no, cleaning_service, sales_amount, paid_amount, payment_mode };
			sales_batch_data.push(a);
        },
    )
	.and_then(|_| Ok(1));
	
	let mut vec_history_sales_batch_data = Vec::new();
	let k: i32 = 0;
	
	for sales_data in sales_batch_data.iter() {
		let cust_name = sales_data.cust_name.to_string();
		let mobile_no = sales_data.mobile_no.to_string();
		
		let batch_no = sales_data.batch_no.as_ref().unwrap_or(&k);
		
		let customer_sales_data = HistoryCustomerSalesData { cust_name: cust_name, mobile_no: mobile_no };
		let carpet_sales_data = select_incoming_carpet_sales_data_requests(conn, batch_no);
		let vehicle_sales_data = select_incoming_vehicle_sales_data_requests(conn, batch_no);
		
		let history_sales_response_data = HistorySalesResponseData {customer_sales_data: customer_sales_data, carpet_sales_data: carpet_sales_data, vehicle_sales_data: vehicle_sales_data };
				
		let history_sales_batch_data = HistorySalesBatchData {batch_no: batch_no.to_string(), sales_data: history_sales_response_data };
		
		
		vec_history_sales_batch_data.push(history_sales_batch_data);

	}
	
	Ok(vec_history_sales_batch_data)
	
}

fn select_incoming_sales_batch_data_requests_old(
    conn: &mut PooledConn) -> Vec<SalesBatchDataTable> {
	let mut selected_data = Vec::new();
	
	//let selected_data: Vec<SalesBatchDataTable> = conn
    conn.query_map(
        "select batch_no, cust_name, mobile_no, cleaning_service, sales_amount, paid_amount, payment_mode from incomingsalesbatchdatarequests order by batch_no asc limit 10",
        |(batch_no, cust_name, mobile_no, cleaning_service, sales_amount, paid_amount, payment_mode)| {
            let a = SalesBatchDataTable { batch_no, cust_name, mobile_no, cleaning_service, sales_amount, paid_amount, payment_mode };
			selected_data.push(a);
        },
    )
	.and_then(|_| Ok(1));
	
	selected_data
	
}

fn select_incoming_search_sales_batch_data_requests(search_data: &String,
    is_mobile_no: &bool, is_customer_name: &bool, is_vehicle_regno: &bool, conn: &mut PooledConn) -> std::result::Result<Vec<HistorySalesBatchData>, mysql::error::Error> {
	let mut sales_batch_data = Vec::new();

	//println!("search_data is {:?}", search_data);
	//println!("is_mobile_no is {:?}", is_mobile_no);
	//println!("is_vehicle_regno is {:?}", is_regno);
	
	//(*) is the dereferencing operator
	//We use it to get the actual value at the address of variable is_vehicle_regno
	let is_regno = *is_vehicle_regno;
	
	if !is_regno {
		conn.exec_map(
		//"select batch_no, cust_name, mobile_no, cleaning_service, sales_amount, paid_amount, payment_mode from incomingsalesbatchdatarequests where cust_name = :search_data",
		"select batch_no, cust_name, mobile_no, cleaning_service, sales_amount, paid_amount, payment_mode from incomingsalesbatchdatarequests where (case when :is_mobile_no = 1 then mobile_no = :search_data else cust_name = :search_data end) order by batch_no desc limit 10;",
		params! {
				"search_data" => search_data,
				"is_mobile_no" => is_mobile_no,
				//"is_customer_name" => is_customer_name,
				//"is_vehicle_regno" => is_vehicle_regno,
			},
		|(batch_no, cust_name, mobile_no, cleaning_service, sales_amount, paid_amount, payment_mode)| { 
		  let a = 
		  SalesBatchDataTable {
				batch_no: batch_no,
				cust_name: cust_name,
				mobile_no: mobile_no,
				cleaning_service: cleaning_service,
				sales_amount: sales_amount,
				paid_amount: paid_amount,
				payment_mode: payment_mode,
			};
			sales_batch_data.push(a);
			},
		)
		.and_then(|_| Ok(1));
	}
	else{
		//let cleaning_service: String = String::from("vehicle");
		
		conn.exec_map(
		//"select a.batch_no, a.cust_name, a.mobile_no, a.cleaning_service, a.sales_amount, a.paid_amount, a.payment_mode from incomingsalesbatchdatarequests a inner join incomingsalesdatarequests b on a.batch_no = b.batch_no where b.vehicle_regno = :search_data and b.cleaning_service = :cleaning_service order by a.batch_no asc limit 10;",
		"select a.batch_no, a.cust_name, a.mobile_no, a.cleaning_service, a.sales_amount, a.paid_amount, a.payment_mode from incomingsalesbatchdatarequests a inner join incomingsalesdatarequests b on a.batch_no = b.batch_no where b.vehicle_regno = :search_data order by a.batch_no asc limit 10;",
		params! {
				"search_data" => search_data,
				//"cleaning_service" => cleaning_service,
			},
		|(batch_no, cust_name, mobile_no, cleaning_service, sales_amount, paid_amount, payment_mode)| { 
		  let a = 
		  SalesBatchDataTable {
				batch_no: batch_no,
				cust_name: cust_name,
				mobile_no: mobile_no,
				cleaning_service: cleaning_service,
				sales_amount: sales_amount,
				paid_amount: paid_amount,
				payment_mode: payment_mode,
			};
			sales_batch_data.push(a);
			},
		)
		.and_then(|_| Ok(1));
	}
	
	//println!("sales_batch_data len is {:?}", sales_batch_data.len());
	
	let mut vec_history_sales_batch_data = Vec::new();
	let k: i32 = 0;
	
	for sales_data in sales_batch_data.iter() {
		let cust_name = sales_data.cust_name.to_string();
		let mobile_no = sales_data.mobile_no.to_string();
		
		let batch_no = sales_data.batch_no.as_ref().unwrap_or(&k);
		/*
		let customer_sales_data = HistoryCustomerSalesData { cust_name: cust_name, mobile_no: mobile_no };
		let carpet_sales_data = select_incoming_carpet_sales_data_requests(conn, batch_no);
		let vehicle_sales_data = select_incoming_vehicle_sales_data_requests(conn, batch_no);
		
		let history_sales_response_data = HistorySalesResponseData {customer_sales_data: customer_sales_data, carpet_sales_data: carpet_sales_data, vehicle_sales_data: vehicle_sales_data };
		*/
		let customer_sales_data = HistoryCustomerSalesData { cust_name: cust_name, mobile_no: mobile_no };
		
		let carpet_sales_data =
		if is_regno {
			//if client searched for vehicle, don't show carpet data
			Vec::new()
		}
		else {
			select_incoming_carpet_sales_data_requests(conn, batch_no)
		};
		
		let vehicle_sales_data = select_incoming_vehicle_sales_data_requests(conn, batch_no);
		
		let history_sales_response_data = HistorySalesResponseData {customer_sales_data: customer_sales_data, carpet_sales_data: carpet_sales_data, vehicle_sales_data: vehicle_sales_data };
		
		let history_sales_batch_data = HistorySalesBatchData {batch_no: batch_no.to_string(), sales_data: history_sales_response_data };
		
		
		vec_history_sales_batch_data.push(history_sales_batch_data);

	}
	
	Ok(vec_history_sales_batch_data)
	
}

fn select_incoming_carpet_sales_data_requests(
    conn: &mut PooledConn, batch_no: &i32) -> Vec<HistoryCarpetSalesData> {
	let mut selected_data = Vec::new();
	let payment_mode: String = String::from("");
	let transaction_date: String = String::from("");
	let cleaning_service: String = String::from("carpet");
	
	//println!("batch_no is {:?}", batch_no);
	
    conn.exec_map(
    "select carpet_size, carpet_colour, sales_amount, date_format(transaction_date, '%d-%m-%Y') transaction_date from incomingsalesdatarequests where batch_no = :batch_no and cleaning_service = :cleaning_service;",
	params! {
            "batch_no" => batch_no,
            "cleaning_service" => cleaning_service,
        },
    |(carpet_size, carpet_colour, sales_amount, transaction_date)| { 
	  let a = 
      HistoryCarpetSalesData {
            carpet_size: carpet_size,
            carpet_colour: carpet_colour,
            sales_amount: sales_amount,
            payment_mode: payment_mode.to_string(),
            transaction_date: transaction_date,
        };
		selected_data.push(a);
		},
	)
	.and_then(|_| Ok(1));
	//}
	/*
	println!("Vector selected_data length: {}", selected_data.len());
	
	for s in selected_data.iter() {
		println!("carpet_size - {:?}", &s.carpet_size.to_string());
		println!("sales_amount - {:?}", &s.sales_amount.to_string());
		println!("transaction_date - {:?}", &s.transaction_date.to_string());
	}	
	*/
	selected_data
	
}

fn select_incoming_carpet_sales_data_requests_old(
    conn: &mut PooledConn, batch_no: &i32) -> Vec<HistoryCarpetSalesData> {
	let mut selected_data = Vec::new();
	let payment_mode: String = String::from("");
	
    conn.query_map(
        "select carpet_size, carpet_colour, sales_amount, transaction_date from incomingsalesdatarequests where batch_no = :batch_no and cleaning_service = 'carpet'",
        |(carpet_size, carpet_colour, sales_amount, payment_mode, transaction_date)| {
            let a = HistoryCarpetSalesData { carpet_size, carpet_colour, sales_amount, payment_mode, transaction_date };
			selected_data.push(a);
        },
    )
	.and_then(|_| Ok(1));
	
	selected_data
	
}

fn select_incoming_vehicle_sales_data_requests(
    conn: &mut PooledConn, batch_no: &i32) -> Vec<HistoryVehicleSalesData> {
	let mut selected_data = Vec::new();
	let payment_mode: String = String::from("");
	let transaction_date: String = String::from("");
	let cleaning_service: String = String::from("vehicle");
	
	//println!("batch_no is {:?}", batch_no);
	
    conn.exec_map(
    "select vehicle_make, vehicle_model, vehicle_regno, sales_amount, interior_cleaning, exterior_cleaning, engine_cleaning, undercarriage_cleaning, date_format(transaction_date, '%d-%m-%Y') transaction_date from incomingsalesdatarequests where batch_no = :batch_no and cleaning_service = :cleaning_service;",
	params! {
            "batch_no" => batch_no,
            "cleaning_service" => cleaning_service,
        },
    |(vehicle_make, vehicle_model, vehicle_regno, sales_amount, interior_cleaning, exterior_cleaning, engine_cleaning, undercarriage_cleaning, transaction_date)| { 
	  let a =
	  HistoryVehicleSalesData { 
			vehicle_make: vehicle_make, 
			vehicle_model: vehicle_model, 
			vehicle_regno: vehicle_regno, 
			sales_amount: sales_amount, 
			payment_mode: payment_mode.to_string(), 
			interior_cleaning: interior_cleaning, 
			exterior_cleaning: exterior_cleaning, 
			engine_cleaning: engine_cleaning, 
			undercarriage_cleaning: undercarriage_cleaning, 
			transaction_date: transaction_date };	
		
		selected_data.push(a);
		},
	)
	.and_then(|_| Ok(1));
	//}
	/*
	println!("Vector selected_data length: {}", selected_data.len());
	
	for s in selected_data.iter() {
		println!("vehicle_make - {:?}", &s.vehicle_make.to_string());
		println!("vehicle_regno - {:?}", &s.vehicle_regno.to_string());
		println!("sales_amount - {:?}", &s.sales_amount.to_string());
		println!("transaction_date - {:?}", &s.transaction_date.to_string());
	}	
	*/
	selected_data
	
}

fn select_incoming_vehicle_sales_data_requests_old(
    conn: &mut PooledConn, batch_no: &i32) -> Vec<HistoryVehicleSalesData> {
	let mut selected_data = Vec::new();
	let payment_mode: String = String::from("");
	
    conn.query_map(
        "select vehicle_make, vehicle_model, vehicle_regno, sales_amount, interior_cleaning, exterior_cleaning, engine_cleaning, undercarriage_cleaning, transaction_date from incomingsalesdatarequests where batch_no = batch_no and cleaning_service = 'vehicle'",
        |(vehicle_make, vehicle_model, vehicle_regno, sales_amount, payment_mode, interior_cleaning, exterior_cleaning, engine_cleaning, undercarriage_cleaning, transaction_date)| {
            let a = HistoryVehicleSalesData { vehicle_make, vehicle_model, vehicle_regno, sales_amount, payment_mode, interior_cleaning, exterior_cleaning, engine_cleaning, undercarriage_cleaning, transaction_date };
			selected_data.push(a);
        },
    )
	.and_then(|_| Ok(1));
	
	selected_data
	
}

fn get_sales_batch_data(sales_batch_data: &Vec<SalesData>) -> SalesBatchDataTable  {
	let mut sales_batch_data_table = SalesBatchDataTable { batch_no: None, cust_name: String::from(""), mobile_no: String::from(""), cleaning_service: String::from(""), sales_amount: 0, paid_amount: 0, payment_mode: String::from("") };
	
	let mut cust_name = String::from("");
	let mut mobile_no = String::from("");
	let mut sales_amount = 0;
	let mut paid_amount = 0;
	let mut sales_amount_s = String::from("");
	let mut paid_amount_s = String::from("");
	let mut payment_mode = String::from("");
	//let mut sales_amount_v = String::from("");
	//let mut sales_amount_c = String::from("");
	let vehicle_sales_data = VehicleSalesData { vehicle_make: String::from(""), vehicle_model: String::from(""), vehicle_regno: String::from(""), sales_amount: String::from(""), payment_mode: String::from(""), interior_cleaning: false, exterior_cleaning: false, engine_cleaning: false, undercarriage_cleaning: false, employee_id: 0, employee_full_names: String::from("") };
	let carpet_sales_data = CarpetSalesData { carpet_size: String::from(""), carpet_colour: String::from(""), sales_amount: String::from(""), payment_mode: String::from(""), employee_id: 0, employee_full_names: String::from("") };
	
	for sales_data in sales_batch_data.iter() {
		cust_name = sales_data.customer_sales_data.cust_name.to_string();
		mobile_no = sales_data.customer_sales_data.mobile_no.to_string();
		payment_mode = sales_data.customer_sales_data.payment_mode.to_string();
		sales_amount_s = sales_data.customer_sales_data.sales_amount.to_string();
		paid_amount_s = sales_data.customer_sales_data.paid_amount.to_string();
		/*
		sales_amount_v = sales_data.vehicle_sales_data.as_ref().unwrap_or(&vehicle_sales_data).sales_amount.to_string();
		//carpet_size = sales_data.carpet_sales_data.as_ref().unwrap_or(&carpet_sales_data).carpet_size.to_string();
		sales_amount_c = sales_data.carpet_sales_data.as_ref().unwrap_or(&carpet_sales_data).sales_amount.to_string();
		
		let vehicle_amount = 
		match sales_amount_v.parse::<i32>() {
		  Ok(a) => a,
		  Err(e) => 0,
		};
		
		let carpet_amount = 
		match sales_amount_c.parse::<i32>() {
		  Ok(a) => a,
		  Err(e) => 0,
		};
		
		sales_amount = vehicle_amount + carpet_amount; //test only
		*/
		let sales_amount = 
		match sales_amount_s.parse::<i32>() {
		  Ok(a) => a,
		  Err(e) => 0,
		};
		
		let paid_amount = 
		match paid_amount_s.parse::<i32>() {
		  Ok(a) => a,
		  Err(e) => 0,
		};
		//Assign values to struct variable
		sales_batch_data_table = SalesBatchDataTable { batch_no: None, cust_name: cust_name, mobile_no: mobile_no, cleaning_service: String::from(""), sales_amount: sales_amount, paid_amount: paid_amount, payment_mode: payment_mode };

	}
	
	sales_batch_data_table
}

fn select_employees_registered_details_requests(
    conn: &mut PooledConn) -> std::result::Result<Vec<EmployeeRegisteredDetails>, mysql::error::Error> {
	let mut employees_registered_data = Vec::new();
	
    conn.query_map(
        "select id,full_names from employeesregistereddetails where employee_type_code = 1 and activated = 1 and duplicate_entry = 0 and deleted = 0 order by full_names asc;",
        |(id, full_names)| {
            let a = EmployeeRegisteredDetails { id, full_names };
			employees_registered_data.push(a);
        },
    )
	.and_then(|_| Ok(1));
	
	Ok(employees_registered_data)
	
}

fn select_sales_commission_details_requests(
    conn: &mut PooledConn) -> std::result::Result<Vec<SalesCommissionDetails>, mysql::error::Error> {
	let mut sales_commission_data = Vec::new();
	
    conn.query_map(
        "select batch_no, cleaning_service, cleaning_service_type, cleaning_amount, commission_percentage, commission_amount, employee_full_names, date_format(transaction_date, '%d-%m-%Y') transaction_date from salescommissiondata order by id asc;",
        |(batch_no, cleaning_service, cleaning_service_type, cleaning_amount, commission_percentage, commission_amount, employee_full_names, transaction_date)| {
            let a = SalesCommissionDetails { batch_no, cleaning_service, cleaning_service_type, cleaning_amount, commission_percentage, commission_amount, employee_full_names, transaction_date };
			sales_commission_data.push(a);
        },
    )
	.and_then(|_| Ok(1));
	
	Ok(sales_commission_data)
	
}

fn select_search_sales_commission_details_requests(search_data: &String,
    is_employee_id: &bool, is_employee_full_names: &bool, conn: &mut PooledConn) -> std::result::Result<Vec<SalesCommissionDetails>, mysql::error::Error> {
	let mut sales_commission_data = Vec::new();
	
    conn.exec_map(
        "select batch_no, cleaning_service, cleaning_service_type, cleaning_amount, commission_percentage, commission_amount, employee_full_names, date_format(transaction_date, '%d-%m-%Y') transaction_date from salescommissiondata where (case when :is_employee_id = 1 then employee_id = :search_data else employee_full_names = :search_data end) order by id asc;",
		params! {
			"search_data" => search_data,
			"is_employee_id" => is_employee_id,
		},
        |(batch_no, cleaning_service, cleaning_service_type, cleaning_amount, commission_percentage, commission_amount, employee_full_names, transaction_date)| {
            let a = SalesCommissionDetails { batch_no, cleaning_service, cleaning_service_type, cleaning_amount, commission_percentage, commission_amount, employee_full_names, transaction_date };
			sales_commission_data.push(a);
        },
    )
	.and_then(|_| Ok(1));
	
	Ok(sales_commission_data)
	
}

fn get_sales_data(sales_batch_data: &Vec<SalesData>, batch_no: i32) -> Vec<SalesDataTable>  {
	let mut sales_data_table = Vec::new();
	let mut vehicle_make = String::from("");
	let mut vehicle_model = String::from("");
	let mut vehicle_regno = String::from("");
	let mut sales_amount_v = String::from("");
	let mut sales_amount_c = String::from("");
	let mut carpet_size = String::from("");
	let mut carpet_colour = String::from("");
	let mut interior_cleaning: bool = false;
	let mut exterior_cleaning: bool = false;
	let mut engine_cleaning: bool = false;
	let mut undercarriage_cleaning: bool = false;
	let mut employee_id_vehicle: i32 = 0;
	let mut employee_full_names_vehicle: String = String::from("");
	let mut employee_id_carpet: i32 = 0;
	let mut employee_full_names_carpet: String = String::from("");
	let vehicle_sales_data = VehicleSalesData { vehicle_make: String::from(""), vehicle_model: String::from(""), vehicle_regno: String::from(""), sales_amount: String::from(""), payment_mode: String::from(""), interior_cleaning: false, exterior_cleaning: false, engine_cleaning: false, undercarriage_cleaning: false, employee_id: 0, employee_full_names: String::from("") };
	let carpet_sales_data = CarpetSalesData { carpet_size: String::from(""), carpet_colour: String::from(""), sales_amount: String::from(""), payment_mode: String::from(""), employee_id: 0, employee_full_names: String::from("") };
	let mut is_valid_vehicle_data: bool = false;
	let mut is_valid_carpet_data: bool = false;
	
	for sales_data in sales_batch_data.iter() {
		is_valid_vehicle_data = false;
		is_valid_carpet_data = false;
		
		interior_cleaning = false;
		exterior_cleaning = false;
		engine_cleaning = false;
		undercarriage_cleaning = false;
		
		employee_id_vehicle = 0;
		employee_full_names_vehicle = String::from("");
		employee_id_carpet = 0;
		employee_full_names_carpet = String::from("");
		
		vehicle_make = sales_data.vehicle_sales_data.as_ref().unwrap_or(&vehicle_sales_data).vehicle_make.to_string();
		vehicle_model = sales_data.vehicle_sales_data.as_ref().unwrap_or(&vehicle_sales_data).vehicle_model.to_string();
		vehicle_regno = sales_data.vehicle_sales_data.as_ref().unwrap_or(&vehicle_sales_data).vehicle_regno.to_string();
		sales_amount_v = sales_data.vehicle_sales_data.as_ref().unwrap_or(&vehicle_sales_data).sales_amount.to_string();
		carpet_size = sales_data.carpet_sales_data.as_ref().unwrap_or(&carpet_sales_data).carpet_size.to_string();
		carpet_colour = sales_data.carpet_sales_data.as_ref().unwrap_or(&carpet_sales_data).carpet_colour.to_string();
		sales_amount_c = sales_data.carpet_sales_data.as_ref().unwrap_or(&carpet_sales_data).sales_amount.to_string();
		employee_id_carpet = sales_data.carpet_sales_data.as_ref().unwrap_or(&carpet_sales_data).employee_id;
		employee_full_names_carpet = sales_data.carpet_sales_data.as_ref().unwrap_or(&carpet_sales_data).employee_full_names.to_string();
		
		interior_cleaning = sales_data.vehicle_sales_data.as_ref().unwrap_or(&vehicle_sales_data).interior_cleaning;
		exterior_cleaning = sales_data.vehicle_sales_data.as_ref().unwrap_or(&vehicle_sales_data).exterior_cleaning;
		engine_cleaning = sales_data.vehicle_sales_data.as_ref().unwrap_or(&vehicle_sales_data).engine_cleaning;
		undercarriage_cleaning = sales_data.vehicle_sales_data.as_ref().unwrap_or(&vehicle_sales_data).undercarriage_cleaning;
		employee_id_vehicle = sales_data.vehicle_sales_data.as_ref().unwrap_or(&vehicle_sales_data).employee_id;
		employee_full_names_vehicle = sales_data.vehicle_sales_data.as_ref().unwrap_or(&vehicle_sales_data).employee_full_names.to_string();
		
		let vehicle_amount = 
		match sales_amount_v.parse::<i32>() {
		  Ok(a) => a,
		  Err(e) => 0,
		};
		
		let carpet_amount = 
		match sales_amount_c.parse::<i32>() {
		  Ok(a) => a,
		  Err(e) => 0,
		};
			  
		if carpet_size.replace(" ","").len() > 0 && carpet_colour.replace(" ","").len() > 0 {
			is_valid_carpet_data = true;
		}
		
		if vehicle_make.replace(" ","").len() > 0 && vehicle_regno.replace(" ","").len() > 0 {
			is_valid_vehicle_data = true;
		}
		
		if is_valid_carpet_data {
			//Assign values to struct variable
			let sales_data_1 = SalesDataTable { batch_no: batch_no, cleaning_service: String::from("carpet"), carpet_size: carpet_size, carpet_colour: carpet_colour, 
			  vehicle_make: String::from(""), vehicle_model: String::from(""), vehicle_regno: String::from(""), interior_cleaning: false, exterior_cleaning: false, engine_cleaning: false, undercarriage_cleaning: false,
			  sales_amount: carpet_amount, employee_id: employee_id_carpet, employee_full_names: employee_full_names_carpet };
			  
			  sales_data_table.push(sales_data_1);
		}
		
		if is_valid_vehicle_data {
			//Assign values to struct variable
			let sales_data_2 = SalesDataTable { batch_no: batch_no, cleaning_service: String::from("vehicle"), carpet_size: String::from(""), carpet_colour: String::from(""), 
			  vehicle_make: vehicle_make, vehicle_model: vehicle_model, vehicle_regno: vehicle_regno, interior_cleaning: interior_cleaning, exterior_cleaning: exterior_cleaning, engine_cleaning: engine_cleaning, undercarriage_cleaning: undercarriage_cleaning,
			  sales_amount: vehicle_amount, employee_id: employee_id_vehicle, employee_full_names: employee_full_names_vehicle };
			  
			  sales_data_table.push(sales_data_2);
		}

	}
	
	sales_data_table
}

fn get_history_sales_batch_data(data: &web::Data<Pool>) -> HistorySalesBatchResponseData  {
	let mut vec_history_sales_batch_data = Vec::new();
	
	match data
        .get_conn()
		.and_then(|mut conn| select_incoming_sales_batch_data_requests(&mut conn))
    {
        Ok(s) => {
            //println!("Successful to open DB connection."),
			//println!("Successful insert to DB connection. {:?}", sales_batch_id);
			vec_history_sales_batch_data = s;
        },
        Err(e) => println!("Failed to open DB connection. {:?}", e),
    }
	
	//Assign values to struct variable
	let output_data = HistorySalesBatchResponseData {status_code: ProcessingStatus::Zero as u32, status_description: String::from("Successful"), sales_batch_data: vec_history_sales_batch_data };
	
	output_data
}
/*
fn get_history_sales_batch_data_old(data: web::Data<Pool>) -> HistorySalesBatchResponseData  {
	let mut vec_history_sales_batch_data = Vec::new();
	let k: i32 = 0;
	
	let conn = get_database_connection(data);
	let sales_batch_data = select_incoming_sales_batch_data_requests(conn);
	
	for sales_data in sales_batch_data.iter() {
		let cust_name = sales_data.cust_name.to_string();
		let mobile_no = sales_data.mobile_no.to_string();
		
		let batch_no = sales_data.batch_no.as_ref().unwrap_or(&k);
		
		let customer_sales_data = HistoryCustomerSalesData { cust_name: cust_name, mobile_no: mobile_no };
		let carpet_sales_data = select_incoming_carpet_sales_data_requests(conn, batch_no);
		let vehicle_sales_data = select_incoming_vehicle_sales_data_requests(conn, batch_no);
		
		let history_sales_response_data = HistorySalesResponseData {customer_sales_data: customer_sales_data, carpet_sales_data: carpet_sales_data, vehicle_sales_data: vehicle_sales_data };
				
		let history_sales_batch_data = HistorySalesBatchData {batch_no: batch_no.to_string(), sales_data: history_sales_response_data };
		
		
		vec_history_sales_batch_data.push(history_sales_batch_data);

	}
	
	//Assign values to struct variable
	let output_data = HistorySalesBatchResponseData {status_code: ProcessingStatus::Zero as u32, status_description: String::from("Successful"), sales_batch_data: vec_history_sales_batch_data };
	
	output_data
}
*/
fn get_history_search_sales_batch_data(search_data: &String,
    is_mobile_no: &bool, is_customer_name: &bool, is_vehicle_regno: &bool, 
	data: &web::Data<Pool>) -> HistorySalesBatchResponseData  {
	let mut vec_history_sales_batch_data = Vec::new();
	
	match data
        .get_conn()
		.and_then(|mut conn| select_incoming_search_sales_batch_data_requests(search_data, is_mobile_no, is_customer_name, is_vehicle_regno, &mut conn))
    {
        Ok(s) => {
			vec_history_sales_batch_data = s;
        },
        Err(e) => println!("Failed to open DB connection. {:?}", e),
    }
	
	//Assign values to struct variable
	let output_data = HistorySalesBatchResponseData {status_code: ProcessingStatus::Zero as u32, status_description: String::from("Successful"), sales_batch_data: vec_history_sales_batch_data };
	
	output_data
}

fn get_employees_registered_data(data: &web::Data<Pool>) -> EmployeesRegisteredResponseData  {
	let mut vec_employees_registered_data = Vec::new();
	
	match data
        .get_conn()
		.and_then(|mut conn| select_employees_registered_details_requests(&mut conn))
    {
        Ok(s) => {
			vec_employees_registered_data = s;
        },
        Err(e) => println!("Failed to open DB connection. {:?}", e),
    }
	
	//Assign values to struct variable
	let output_data = EmployeesRegisteredResponseData {status_code: ProcessingStatus::Zero as u32, status_description: String::from("Successful"), employees_data: vec_employees_registered_data };
	
	output_data
}

fn get_sales_commission_data(data: &web::Data<Pool>) -> SalesCommissionResponseData  {
	let mut vec_sales_commission_data = Vec::new();
	
	match data
        .get_conn()
		.and_then(|mut conn| select_sales_commission_details_requests(&mut conn))
    {
        Ok(s) => {
			vec_sales_commission_data = s;
        },
        Err(e) => println!("Failed to open DB connection. {:?}", e),
    }
	
	//Assign values to struct variable
	let output_data = SalesCommissionResponseData {status_code: ProcessingStatus::Zero as u32, status_description: String::from("Successful"), sales_commission_data: vec_sales_commission_data };
	
	output_data
}

fn get_search_entry_sales_commission_data(search_data: &String,
    is_employee_id: &bool, is_employee_full_names: &bool, 
	data: &web::Data<Pool>) -> SalesCommissionResponseData  {
	let mut vec_sales_commission_data = Vec::new();
	
	match data
        .get_conn()
		.and_then(|mut conn| select_search_sales_commission_details_requests(search_data, is_employee_id, is_employee_full_names, &mut conn))
    {
        Ok(s) => {
			vec_sales_commission_data = s;
        },
        Err(e) => println!("Failed to open DB connection. {:?}", e),
    }
	
	//Assign values to struct variable
	let output_data = SalesCommissionResponseData {status_code: ProcessingStatus::Zero as u32, status_description: String::from("Successful"), sales_commission_data: vec_sales_commission_data };
	
	output_data
}

fn validate_client_api(req: HttpRequest, api_function: String) -> ClientApiResponseDetails  {
	
	let mut client_ip = String::from("");
	let mut authorization = String::from("");
	let mut channel_type = String::from("");
	let mut app_ver_code = String::from("");
	let mut app_id_tok = String::from("");
	let mut dev_id = String::from("");
	let mut dev_tok_regno = String::from("");
	let mut auth_token = String::from("");
	let mut user_name = String::from("");
	let mut pass_word = String::from("");
	let mut status_description = String::from("Error occured during processing, please try again.");
	let mut status_code = ProcessingStatus::One as u32;
	
	if !req.headers().is_empty() {
		if let Some(val) = req.peer_addr() {
			client_ip = val.ip().to_string()
		}
		if req.headers().contains_key("authorization") {
			let m = req.headers().get("authorization").unwrap();
			authorization = m.to_str().unwrap().to_string();
			//println!("m authorization - {:?}", m);
			if !authorization.is_empty() {
				if authorization.to_lowercase().contains("bearer") {
					//println!("bearer found");
					let v: Vec<&str> = authorization.split(' ').collect();
					//println!("v - {:?}", v);
					let s = v.len();
					if s == 2 {
						auth_token = String::from(v[1]);
						//println!("auth_token - {:?}", auth_token);
						let bytes = decode(auth_token).unwrap();
						let m_auth_token = str::from_utf8(&bytes).unwrap().to_string();
						//println!("auth_token bytes 2 - {:?}", m_auth_token);
						if !m_auth_token.is_empty() {
							if m_auth_token.contains(":") {
								let w: Vec<&str> = m_auth_token.split(':').collect();
								//println!("w - {:?}", w);
								let t = w.len();
								if t == 2 {
									user_name = String::from(w[0]);
									pass_word = String::from(w[1]);
								}
							}
							//println!("user_name - {:?}", user_name);
							//println!("pass_word - {:?}", pass_word);
						}
					}
				}
			}
		}
		if req.headers().contains_key("channeltype") {
			let m = req.headers().get("channeltype").unwrap();
			channel_type = m.to_str().unwrap().to_string();
			//println!("m channel_type - {:?}", m);
		}
		if req.headers().contains_key("appvercode") {
			let m = req.headers().get("appvercode").unwrap();
			app_ver_code = m.to_str().unwrap().to_string();
			//println!("m app_ver_code - {:?}", m);
		}
		if req.headers().contains_key("appidtok") {
			let m = req.headers().get("appidtok").unwrap();
			app_id_tok = m.to_str().unwrap().to_string();
			//println!("m app_id_tok - {:?}", m);
		}
		if req.headers().contains_key("devid") {
			let m = req.headers().get("devid").unwrap();
			dev_id = m.to_str().unwrap().to_string();
			//println!("m dev_id - {:?}", m);
		}
		if req.headers().contains_key("devtokregno") {
			let m = req.headers().get("devtokregno").unwrap();
			dev_tok_regno = m.to_str().unwrap().to_string();
			//println!("m dev_tok_regno - {:?}", m);
		}
	}
	
	if client_ip.len() > 0 && channel_type.len() > 0 && user_name.len() > 0 && pass_word.len() > 0 && api_function.len() > 0 {
		if channel_type.to_lowercase().eq(&String::from("mobileapp")) {
			status_code = ProcessingStatus::Zero as u32;
			status_description = String::from("Successful");
		}
	}
	
	//println!("validate_client_api: status_code - {:?}", status_code);
	//println!("validate_client_api: status_description - {:?}", status_description);
	
	//Assign values to struct variable
	let output_data = ClientApiResponseDetails {status_code: status_code, status_description: status_description };
	
	output_data
}
/*
fn get_database_connection(data: web::Data<Pool>) -> &'static mut PooledConn {
	
	let mut conn: PooledConn;
	match data
        .get_conn()
		//.and_then(|mut conn| &mut conn)
    {
        Ok(c) => {
            conn = c;
        },
		Err(e) => {
            println!("Failed to open DB connection. {:?}", e);
        },
    };
	
	&conn
	
	//let mut conn = data.get_conn()?;
	
}
*/

#[actix_web::main]
async fn main() {
	//async fn main() -> std::io::Result<()> {
	/*
    HttpServer::new(|| {
        App::new()
		    .app_data(shared_data.clone())
		    .service(hello_world)
            .service(current_temperature)
			.service(get_person)
			.service(get_vehicle_make_data)
			.service(get_vehicle_model_data)
			.service(get_carpet_type_size_data)
			.service(get_carpet_type_colour_data)
			.service(get_vehicle_cleaning_type_cost_data)
			.service(get_carpet_cleaning_type_cost_data)
			.service(add_sales_data)
			.service(get_all_sales_data)
			.service(get_search_sales_data)
            .route("/", web::get().to(greet))
            .route("/{name}", web::get().to(greet))
    })
    //.bind("127.0.0.1:8080")?
	//.bind("192.168.3.22:9247")?
	//.bind("127.0.0.1:9247")? //accessible from the machine only
	.bind("0.0.0.0:9247")? //accessible from outside the machine itself
    .run()
    .await
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
		    //.service(hello_world)
            //.service(current_temperature)
			.service(add_sales_data)
			//.service(get_person)
			.service(get_vehicle_make_data)
			.service(get_vehicle_model_data)
			.service(get_carpet_type_size_data)
			.service(get_carpet_type_colour_data)
			.service(get_vehicle_cleaning_type_cost_data)
			.service(get_carpet_cleaning_type_cost_data)
			.service(get_all_sales_data)
			.service(get_search_sales_data)
			.service(get_all_employees_data)
			.service(get_all_sales_commission_data)
			.service(get_search_sales_commission_data)
			.service(process_ussd_actions)
            .route("/", web::get().to(greet))
            //.route("/{name}", web::get().to(greet))
    }).bind("0.0.0.0:9247") {
        Ok(s) => s,
        Err(e) => {
            println!("Failed to bind port. {:?}", e);
            return;
        }
    };
	//println!("[info] ActixWebHttpServer - Listening for HTTP on /0:0:0:0:0:0:0:0:9247")
    match server.run().await {
        Ok(_) => println!("Server exited normally."),
        Err(e) => println!("Server exited with error: {:?}", e),
    };
}