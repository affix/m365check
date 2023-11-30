use std::{collections::HashMap, fs::File, path::Path};
use std::io::{self, BufRead};
use serde::{Deserialize,Serialize};
use colored::*;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// E-mail address of a single user to check
    /// if no option is provided, the program will read from stdin
    #[arg(short, long, default_value_t)]
    email: String,

    /// File containing a list of e-mail addresses to check
    /// One e-mail address per line
    #[arg(short, long, default_value_t)]
    file: String,
}



#[derive(Debug, Serialize, Deserialize)]
struct UserData {
    #[serde(rename = "Username")]
    username: String,
    #[serde(rename = "Display")]
    display: String,
    #[serde(rename = "IfExistsResult")]
    if_exists_result: i32,
    #[serde(rename = "IsUnmanaged")]
    is_unmanaged: bool,
    #[serde(rename = "ThrottleStatus")]
    throttle_status: i32,
    #[serde(rename = "Credentials")]
    credentials: CredentialsData,
    #[serde(rename = "EstsProperties")]
    ests_properties: EstsPropertiesData,
    #[serde(rename = "IsSignupDisallowed")]
    is_signup_disallowed: bool,
    #[serde(rename = "apiCanary")]
    api_canary: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct CredentialsData {
    #[serde(rename = "PrefCredential")]
    pref_credential: i32,
    #[serde(rename = "HasPassword")]
    has_password: bool,
    #[serde(rename = "RemoteNgcParams")]
    remote_ngc_params: Option<serde_json::Value>,
    #[serde(rename = "FidoParams")]
    fido_params: Option<serde_json::Value>,
    #[serde(rename = "SasParams")]
    sas_params: Option<serde_json::Value>,
    #[serde(rename = "CertAuthParams")]
    cert_auth_params: Option<serde_json::Value>,
    #[serde(rename = "GoogleParams")]
    google_params: Option<serde_json::Value>,
    #[serde(rename = "FacebookParams")]
    facebook_params: Option<serde_json::Value>,
    #[serde(rename = "OtcNotAutoSent")]
    otc_not_auto_sent: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct EstsPropertiesData {
    #[serde(rename = "DomainType")]
    domain_type: i32,
}

fn validate_user(user: String) {
    let mut user_json = HashMap::new();
    user_json.insert("Username", user);

    let client = reqwest::blocking::Client::new();
    let res = client
        .post("https://login.microsoftonline.com/common/GetCredentialType")
        .json(&user_json)
        .send();

    match res {
        Ok(resp) => {
             let data = resp.json::<UserData>().unwrap();
             if data.if_exists_result == 0 {
                 println!("{} : {}", "[+] Found existing user".green().bold(), data.username.green().bold());
             } else {
                 println!("{} : {}", "[-] User does not exist".red(), data.username.red());
             }
        },
        Err(e) => println!("{:#?}", e),   
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() -> io::Result<()> {
    println!("{}", "Microsoft 365 User Checker".bold().purple());
    println!("{}", "Written by Keiran \"Affix\" Smith".bold().purple());
    println!("{}", "https://github.com/Affix/m365check".bold().purple());

    let args = Args::parse();

    if args.email != "" {
        validate_user(args.email.trim().to_string());
    } else if args.file != "" {
        if let Ok(lines) = read_lines(args.file) {
            for line in lines {
                if let Ok(user) = line {
                    validate_user(user);
                }
            }
        }
    } else {
        println!("{}", "Using STDIN ".bold().cyan());
        let mut buffer = String::new();
        let stdin = io::stdin(); 
        stdin.read_line(&mut buffer)?;
        let user = String::from(buffer.trim());
        validate_user(user)
    }
 
    Ok(())

}
