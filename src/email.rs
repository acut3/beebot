use lettre::{
    Message,
    Transport,
    SmtpTransport,
    message::{
        header,
        SinglePart,
    },
    transport::smtp::authentication::Credentials,
};

use crate::config::Config;

pub fn send_email(cfg: &Config, to: &str, jwt: &str) -> Result<(), String> {
    let dest = match to.parse() {
        Ok(val) => val,
        Err(_) => return Err(format!("'{}' n'est pas une adresse mail valide", to)),
    };
    let from = match cfg.smtp.user.parse() {
        Ok(val) => val,
        Err(_) => return Err(format!("L'adresse mail {} n'est pas valide", to)),
    };
    let email = Message::builder()
        .to(dest)
        .from(from) // From is required by lettre, but ignored by gmail
        .subject("Accès à Discord")
        .singlepart(
            SinglePart::builder()
                .header(header::ContentType("text/plain; charset=utf8".parse().unwrap()))
                .body(format!("Envoie (copie/colle) la command suivante au bot dans un message privé:\n\n!grant {}\n\nAttention ! Bien qu'il ne contienne aucunes données personnelles ou confidentielles, ce token ne doit pas être partagé. Ne l'envoie pas sur un salon public !", jwt)))
        .unwrap();

    let mailer = match SmtpTransport::relay(&cfg.smtp.host) {
        Ok(val) => val,
        Err(_) => return Err(format!("{} n'est pas un serveur SMTP valide", cfg.smtp.host)),
    }
        .credentials(Credentials::new(cfg.smtp.user.clone(), cfg.smtp.password.clone()))
        .build();

    // Send the email
    match mailer.send(&email) {
        Ok(_) => Ok(()),
        Err(e) => Err(e.to_string()),
    }
}
