use std::{
    fs::File,
    io::{BufRead, BufReader},
    collections::HashMap,
};

use crate::config::Config;


//  {
//      "<email>": {
//          <guild_id>: [
//              <role_id>
//          ]
//      }
//  }
pub type Grants = HashMap<String, UserGrants>;
pub type UserGrants = HashMap<u64, Vec<u64>>;


pub fn load_grants(cfg: &Config) -> Grants {
    let mut grants = HashMap::new();
    for cfg_guild in &cfg.guilds {
        for cfg_grant in &cfg_guild.grants {
            for cfg_email in load_emails(cfg, &cfg_grant.emails) {
                let guilds = grants.entry(cfg_email).or_insert(HashMap::new());
                let roles = guilds.entry(cfg_guild.id).or_insert(Vec::new());
                for role in cfg_grant.roles.iter() {
                    // Only add role if it's not already listed
                    if roles.iter().find(|&&x| x == *role).is_none() {
                        roles.push(*role);
                    }
                }
            }
        }
    }

    grants
}


fn load_emails(cfg: &Config, filename: &str) -> Vec<String> {
    let path = format!{"{}/{}", cfg.root_dir, filename};
    let file = match File::open(&path) {
        Ok(val) => val,
        Err(_) => panic!("Cannot open {}", path),
    };
    match BufReader::new(file).lines().collect() {
        Ok(val) => val,
        Err(_) => panic!("Error reading {}", path),
    }
}
