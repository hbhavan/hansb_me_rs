use super::*;

pub struct RPCBot;

impl ProjectData for RPCBot {
    fn project_id(&self) -> String {
        String::from("rpc_bot")
    }

    fn title(&self) -> String {
        String::from("RPC Bot")
    }

    fn skills(&self) -> Vec<Skillset> {
        use Skillset::*;
        vec![Dotnet, SQL]
    }

    fn status(&self) -> Status {
        Status::ongoing(2023, 11)
    }

    fn project_type(&self) -> ProjectType {
        ProjectType::Experiment
    }

    fn desc(&self) -> String {
        String::from(
            "
RPC Bot is a discord bot application initially created using the Discord Javascript
API, and then migrated to Dotnet.
### Features:
#### Bullshite
The first command I developed was just a simple coinflip. The name is taken from an inside-joke within our group. Based on the result of the coinflip, it would either post
`AHHHHHHHHHHHHHHHHHHHHHH`
or
`Do YOU have a bullshite?`.
#### Caught lacking
Certain members of the chatroom like to post messages/images/reactions and instantly delete them.
The bot will call them out for doing so. I saw what you deleted.
#### Pokedraw
At any time, the `!pokedraw` command will pull a random Pokemon from a list of all of them (stored in a text file because I didn't want to spam the API on startup).
The initial motivation was for user's to draw the pokemon they pulled from memory, but it also functions as a random Pokemon selector.
#### Gacha game
The bot has a built-in gacha game. Every hour, a loot pool is refreshed with varying drops. The drops would have a random rarity assigned to them and the rarity determined how many points the drop would give. During each loot pool interval, each member of the chat could invoke the `!gamba` command to roll for 3 drops taken from the current loot pool. After an initial delay (thanks in part to Discord rate-limiting the loading message's updates), the bot would display the 3 drops chosen along with 3 random emojis. Users in the chat then have the opportunity to claim one of the drops by posting the corresponding emoji. This allows anyone active in the chat to have a chance at claiming the loot. The drops expire once 15 seconds have passed, after which the winners are displayed along with the points they earned from the claimed loot. Points are then added to their score, stored in a SQL database. Each user has a daily refresh of their roll that allows them to roll again and claim again during a loot pool interval.
        ",
        )
    }
}

