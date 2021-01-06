mod config;
mod grants;
mod jwt;
mod email;

use crate::config::*;
use crate::grants::*;
use crate::jwt::*;
use crate::email::*;

use std::collections::HashSet;

use serenity::{
    prelude::{
        TypeMapKey,
    },
    client::{
        Client,
        Context,
        EventHandler
    },
    framework::standard::{
        StandardFramework,
        CommandGroup,
        CommandResult,
        help_commands,
        Args,
        HelpOptions,
        macros::{
            command,
            group,
            help
        },
    },  
    model::prelude::{
        Message,
        UserId,
        RoleId,
        PartialGuild,
    }
};


#[group]
#[commands(iam, grant)]
struct General;

struct Handler;
impl EventHandler for Handler {}

struct DataConfig;
impl TypeMapKey for DataConfig {
    type Value = Config;
}

struct DataGrants;
impl TypeMapKey for DataGrants {
    type Value = Grants;
}


#[help]
async fn my_help(
    context: &Context,
    msg: &Message,
    args: Args,
    help_options: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    owners: HashSet<UserId>
) -> CommandResult {
    let _ = help_commands::with_embeds(context, msg, args, help_options, groups, owners).await;
    Ok(())
}


// !iam <email>
#[command]
#[only_in(dm)]
#[usage("<email>")]
#[example("jean.martin@example.com")]
async fn iam(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let response;
    let email = args.rest();
    let data = ctx.data.read().await;
    let peer_grants = data.get::<DataGrants>().unwrap().get(email);
    if peer_grants == None || peer_grants.unwrap().is_empty() {
        response = Some(format!("Aucun rôle n'est défini pour l'adresse \"{0}\"", email));
    } else {
        // Generate and send JWT
        let cfg = data.get::<DataConfig>().unwrap();
        let jwt = jwt_encode(*msg.author.id.as_u64(), &peer_grants.unwrap(), &cfg.jwt_key, cfg.jwt_lifetime)?;
        #[cfg(debug_assertions)]
        println!("JWT: {}", jwt);
        response = match send_email(cfg, &email, &jwt) {
            Ok(_) => Some(format!("Je t'ai envoyé un mail à l'adresse \"{0}\". Il contient une commande `!grant` que tu devras copier/coller ici, en échange de quoi je t'attribuerai les rôles auxquels tu as droit. La commande est valable {1} minutes.", email, cfg.jwt_lifetime)),
            Err(e) => Some(format!("Une erreur est survenue: {}", e)),
        };
    }
    // Send response if any
    if let Some(val) = response {
        msg.channel_id.say(&ctx.http, val).await?;
    }
    Ok(())
}


// !grant <jwt>
#[command]
#[only_in(dm)]
#[usage("<JWT>")]
#[example("eyJ0eXAi...")]
async fn grant(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let mut response = None;
    let jwt = args.rest();
    let data = ctx.data.read().await;
    let cfg = data.get::<DataConfig>().unwrap();

    let grants = jwt_decode(*msg.author.id.as_u64(), &jwt, &cfg.jwt_key);
    if grants.is_err() {
        response = Some(grants.unwrap_err());
    } else {
        for (guild_id, role_ids) in grants.unwrap().iter() {
            let guild = PartialGuild::get(&ctx.http, *guild_id).await?;
            let mut member = guild.member(&ctx.http, msg.author.id).await?;
            for role_id in role_ids {
                let status = match member.add_role(&ctx.http, *role_id).await {
                    Ok(_) => "Ok".to_string(),
                    Err(e) => format!("Erreur: {}", e),
                };
                let role_name = match guild.roles.get(&RoleId::from(*role_id)) {
                    Some(val) => val.name.as_str(),
                    None => "?",
                };
                msg.channel_id.say(&ctx.http, format!("{guild}, ajout du rôle \"{role}\": {status}", role=role_name, guild=guild.name, status=status)).await?;
            }
        }
    }
    // Send response if any
    if let Some(val) = response {
        msg.channel_id.say(&ctx.http, val).await?;
    }
    Ok(())
}


#[tokio::main]
async fn main() {
    let cfg = load_config();
    #[cfg(debug_assertions)]
    println!("cfg: {:?}", cfg);

    let grants = load_grants(&cfg);
    #[cfg(debug_assertions)]
    println!("grants: {:?}", grants);

    let framework = StandardFramework::new()
        .configure(|c| c.prefix("!"))
        .group(&GENERAL_GROUP)
        .help(&MY_HELP);

    let mut client = Client::builder(&cfg.token)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Err creating client");

    {
        let mut data = client.data.write().await;
        data.insert::<DataConfig>(cfg);
        data.insert::<DataGrants>(grants);
    }

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
