A simple Discord bot I created for a Discord server I own. It's *not* a
general-purpose bot. It's very much a work in progress, and also a way for
me to learn Rust. Feel free to open issues or submit pull requests, or contact
me directly with any comment or suggestion you may have.

The main feature is the ability to automatically assign roles based on the user's email address:

1. The bot automatically DMs any new user that joins the guild

1. The new user DMs the bot `!iam <email>`

1. The bot generates a JWT and sends it to specified email address. The JWT contains the user's discord user id and the roles associated with the email address, as defined in the bot's configuration files.

1. The user DMs the bot `!iam <JWT>`, thereby proving he owns the email address

1. If the user id of the author matches the user id in the JWT, the bot assigns the roles contained in the JWT
