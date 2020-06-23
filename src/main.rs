use std::env;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::io::Write;
use serde_json::json;
use tokio::prelude::*;

use mojang_api::Error;

use reqwest::Body;
use reqwest::multipart;

use tokio_util::codec::BytesCodec;
use tokio_util::codec::FramedRead;

use uuid::Uuid;

use clap::{App, load_yaml};

use rand::prelude::*;


struct Arguments {
    filepath: String,    
}

fn parse_file(filepath: &str) -> Result<(String, String), &str>{
    let file_content = fs::read_to_string(filepath)
	.expect("Could not open file");


    let data: Vec<&str> = file_content.split("\n").collect();

    let len = data.len();
    if len < 2 {
	// There's probably a prettier way to do this
	// This probably shouldn't default to ("", "") either
	// and a warning message should be displayed

	return Err("Incorrect file")
    }
    
    let username = String::from(data[0]);
    let password = String::from(data[1]);
    
    Ok((username, password))
}

fn pick_random_from(dirpath: &str) -> Result<PathBuf, std::io::Error> {
    let mut entries = Vec::<PathBuf>::new();
    for entry in fs::read_dir(dirpath)? {
	let entry = entry?;
	entries.push(entry.path().canonicalize().unwrap());
    }
    let len: usize = entries.len();
    let mut rng = rand::thread_rng();
    let mut rngval: usize = rng.gen();
    rngval %= len;
    
    Ok(entries.remove(rngval))
}

fn pick_skin(path: &str) -> Result<PathBuf, std::io::Error> {
    let attr = fs::metadata(path)
	.expect("Skin file not found");
    if attr.is_dir() {
	return pick_random_from(path);
    } else if attr.is_file() {
	return Ok(PathBuf::from(path));
    }
    Err(io::Error::new(io::ErrorKind::Other, "Path is neither a file nor a dir"))
}

async fn upload_skin(token: &str, uuid: &Uuid, path: &str, model: &str) -> Result<(), String> {
    let client = reqwest::Client::new();
    let url = format!("https://api.mojang.com/user/profile/{}/skin",
		      str::replace(&uuid.to_string(), "-", ""));


    let skinpath = pick_skin(path)
	.expect("Could not pick file");
    
    let file = 	FramedRead::new(
	tokio::fs::File::open(&skinpath).await.unwrap(),
	BytesCodec::new()
    );
    

    let file_name: String = String::from(skinpath
	.file_name()
	.expect("Impossible file name")
	.to_str()
	.expect("No known conversion from filepath to string format"));

    println!("Chosen skin: {}", file_name);
    
    let part = multipart::Part::stream(Body::wrap_stream(file))
	.file_name(file_name);

    let form = multipart::Form::new()
	.text("model", String::from(model))
	.part("file", part);

    println!("{}", url);
    
    let mut packet = client.put(&url)
	.header("Authorization", format!("Bearer: {}", token))
	.multipart(form);
    
    let res = packet.send()
	.await;
    
    match &res {
	Ok(e) => println!("Request sent"),
	Err(e) => return Err(format!("Could not send request: {}", e))
    }

    // Check if the server sends a response
    // If that's the case then that means that the skin could not be changed
    let res2 = res.unwrap()
	.text()
	.await;

    match res2 {
	Ok(_) => {}, // Technically, this should be treated as a en error
	Err(_) => return Err(String::from("Could not send request"))
    }    
	
    Ok(())
}

async fn change_skin(uuid: &Uuid, token: &str, skinpath: &str, model: &str) -> Result<(), String> {
    
    upload_skin(token, uuid, skinpath, model).await?;
    Ok(())
}

async fn write_token_file(file_path: &str, uuid: &Uuid, token: &str) -> Result<(), std::io::Error> {
    let mut token_file = std::fs::File::create(file_path)?;
    token_file.write(uuid.to_string().as_bytes())?;
    token_file.write("\n".as_bytes());
    token_file.write(token.as_bytes())?;
    Ok(())
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let yaml = load_yaml!("cli.yml");
    let matches = App::from(yaml).get_matches();

    let token: String;
    let uuid: Uuid;
    
    if matches.value_of("token") == None {    
	let cred_path = match matches.value_of("cred") {
	    Some(x) => x,
	    None => panic!("Option to use token file has not been implemented yet")
	};

	let (username, password) = parse_file(cred_path)
	    .expect("Could not parse token file");

	
	let session = mojang_api::client_login(&username, &password)
	    .await.unwrap();
	let server_hash = mojang_api::server_hash("", [0u8; 16], &[1]);
       
	token = session.access_token;
	uuid = session.profile.uuid;    	
    } else {
	// Parse token file
	let token_path = matches.value_of("token").unwrap();
	let (uuidstr, tokenstr) = parse_file(token_path)
	    .expect("Could not parse token file");
	token = tokenstr;
	uuid = Uuid::parse_str(&uuidstr)
	    .expect("Wrong uuid format");
    }

    if matches.value_of("export") != None {
	let export_file = matches.value_of("export").unwrap();
	println!("Writing tokens to file");
	write_token_file(export_file, &uuid, &token).await;	
    }
    
    let skin_path = match matches.value_of("input") {
	Some(x) => x,
	None => panic!("Skin path has not been specified")
    };

    let model = match matches.value_of("model") {
	Some("slim") => "slim",
	_ => ""
    };
    
    
    match change_skin(&uuid, &token, &skin_path, &model).await {
	Ok(()) => println!("Skin successfully changed"),
	Err(x) => println!("Could not change skin {:?}", x)
    };

}
