use std::env;
use std::fs;
use std::convert::TryFrom;
use yaml_rust::{Yaml, YamlLoader};

// Path of our files, relative to $HOME
const ROOT_DIR: &str = ".config/beebot";

// Path of our config file, relative to ROOT_DIR
const CFG_FILE: &str = "config.yaml";


#[cfg_attr(debug_assertions, derive(Debug))]
pub struct Config {
    pub root_dir: String,
    pub token: String,
    pub jwt_lifetime: u64,
    pub jwt_key: Vec<u8>,
    pub smtp: Smtp,
    pub guilds: Vec<Guild>,
}

#[cfg_attr(debug_assertions, derive(Debug))]
pub struct Smtp {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
}

#[cfg_attr(debug_assertions, derive(Debug))]
pub struct Guild {
    pub id: u64,
    pub grants: Vec<Grant>,
}

#[cfg_attr(debug_assertions, derive(Debug))]
pub struct Grant {
    pub emails: String,
    pub roles: Vec<u64>,
}


pub fn load_config() -> Config {
    // Parse YAML config file
    let root_dir = match env::var_os("HOME") {
        Some(val) => format!("{}/{}", val.to_str().unwrap(), ROOT_DIR),
        None => panic!("HOME is not defined"),
    };
    let fname = format!("{}/{}", root_dir, CFG_FILE);
    let contents = fs::read_to_string(&fname)
        .expect(format!("Error reading {}", fname).as_str());
    let yamls = YamlLoader::load_from_str(&contents)
        .expect(format!("Error parsing {}", fname).as_str());
    let yaml = &yamls[0];

    // Read configuration
    let token = match yaml["token"].as_str() {
        Some(val) => String::from(val),
        None => panic!("<token> not defined in config file"),
    };

    let jwt_lifetime = match yaml["jwt_lifetime"] {
        Yaml::Integer(val) => match u64::try_from(val) {
            Ok(val) => val,
            Err(e) => panic!("<jwt_lifetime> is not valid: {}", e),
        },
        Yaml::BadValue => 60,
        _ => panic!("<jwt_lifetime> should be an integer"),
    };

    let jwt_key = match yaml["jwt_key"].as_str() {
        Some(val) => base64::decode(val)
            .expect("<jwt_key> is not valid base64"),
        None => panic!("<jwt_key> not defined in config file"),
    };
    if jwt_key.len() < 16 {
        panic!("<jwt_key> is too short");
    }

    if yaml["smtp"].is_badvalue() {
        panic!("<smtp> is not defined");
    }
    let smtp = get_smtp(&yaml["smtp"]);

    let mut guilds = Vec::new();
    if yaml["guilds"].is_badvalue() {
        panic!("<guilds> is not defined");
    }
    for guild in yaml["guilds"].as_vec().unwrap_or(&Vec::new()) {
        guilds.push(get_guild(guild));
    }

    Config {
        root_dir,
        token,
        jwt_lifetime,
        jwt_key,
        smtp,
        guilds,
    }
}


fn get_smtp(yaml: &Yaml) -> Smtp {
    let host = match yaml["host"].as_str() {
        Some(val) => String::from(val),
        None => panic!("<host> is not defined"),
    };
    let port = match yaml["port"].as_i64() {
        Some(val) => u16::try_from(val)
            .expect("<port> is invalid"),
        None => panic!("<port> is not defined"),
    };
    let user = match yaml["user"].as_str() {
        Some(val) => String::from(val),
        None => panic!("<user> is not defined"),
    };
    let password = match yaml["password"].as_str() {
        Some(val) => String::from(val),
        None => panic!("<password> is not defined"),
    };

    Smtp {
        host,
        port,
        user,
        password,
    }
}


fn get_grant(yaml: &Yaml) -> Grant {
    let emails = match yaml["emails"].as_str() {
        Some(val) => String::from(val),
        None => panic!("<emails> is not defined"),
    };
    let mut roles = Vec::new();
    for role in yaml["roles"].as_vec().unwrap_or(&Vec::new()) {
        if let Some(id_str) = role.as_str() {
            let id = match id_str.parse::<u64>() {
                Ok(val) => val,
                Err(e) => panic!("{} is not a valid role id: {}", id_str, e),
            };
            roles.push(id);
        }
    }

    Grant {
        emails,
        roles,
    }
}


fn get_guild(yaml: &Yaml) -> Guild {
    let id_str = match yaml["id"].as_str() {
        Some(val) => String::from(val),
        None => panic!("<id> is not defined"),
    };
    let id = match id_str.parse::<u64>() {
        Ok(val) => val,
        Err(e) => panic!("{} is not a valid guild id: {}", id_str, e),
    };
    let mut grants = Vec::new();
    for grant in yaml["grants"].as_vec().unwrap_or(&Vec::new()) {
        grants.push(get_grant(grant));
    }

    Guild {
        id,
        grants,
    }
}
