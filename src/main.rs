mod config;
mod store;
mod types;
mod weather;

use anyhow::Context as _;
use serenity::all::{GuildId, Interaction};
use serenity::async_trait;
use serenity::builder::{
    CreateCommand, CreateCommandOption, CreateInteractionResponse, CreateInteractionResponseMessage,
};
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use shuttle_runtime::SecretStore;
use tracing::info;

use crate::store::Store;
use crate::types::{Coordinates, Location};

async fn handle_predict_by_location(
    city_option: Option<serenity::all::CommandDataOption>,
    client: &reqwest::Client,
) -> String {
    let argument = city_option;
    let Some(argument) = argument else {
        return "Please enter a city name".to_owned();
    };

    let value = argument.value.as_str();
    let Some(value) = value else {
        return "Please enter a value".to_owned();
    };

    let lowercase_value = value.to_lowercase();
    let store = Store::get_store().await;
    let coordinates = store
        .get_coordinates_by_location(&Location::from(lowercase_value.as_str()))
        .await;

    let Some(coordinates) = coordinates else {
        return format!(
            "We currently do not have {value} in our database, you can try using the command with coordinates instead."
        );
    };

    let result = weather::get_aurora_prediction(coordinates, client).await;

    match result {
        Ok(probability) => {
            format!(
                "There is a {}% chance to see an aurora in {value}",
                probability.value
            )
        }
        Err(err) => {
            format!("Err: {err}")
        }
    }
}

struct Bot {
    client: reqwest::Client,
    discord_guild_id: GuildId,
}

#[async_trait]
impl EventHandler for Bot {
    async fn ready(&self, ctx: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);

        let commands = vec![
            CreateCommand::new("hello").description("Say hello"),
            CreateCommand::new("predict-by-coordinates")
                .description("Display the aurora prediction information")
                .add_option(
                    CreateCommandOption::new(
                        serenity::all::CommandOptionType::Integer,
                        "latitude",
                        "Latitude rounded to closest integer to lookup",
                    )
                    .required(true),
                )
                .add_option(
                    CreateCommandOption::new(
                        serenity::all::CommandOptionType::Integer,
                        "longitude",
                        "Longitude rounded to closest integer to lookup",
                    )
                    .required(true),
                ),
            CreateCommand::new("predict-by-location")
                .description("Display the aurora prediction information")
                .add_option(
                    CreateCommandOption::new(
                        serenity::all::CommandOptionType::String,
                        "city",
                        "City to lookup Aurora information for",
                    )
                    .required(true),
                ),
        ];

        let commands = &self
            .discord_guild_id
            .set_commands(&ctx.http, commands)
            .await
            .unwrap();

        info!("Registered commands: {:#?}", commands);
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(command) = interaction {
            let response_content = match command.data.name.as_str() {
                "hello" => "hello".to_owned(),
                "predict-by-coordinates" => {
                    let argument1 = command
                        .data
                        .options
                        .iter()
                        .find(|opt| opt.name == "latitude")
                        .cloned();

                    let argument2 = command
                        .data
                        .options
                        .iter()
                        .find(|opt| opt.name == "longitude")
                        .cloned();

                    let value1 = argument1.unwrap().value;
                    let value2 = argument2.unwrap().value;
                    let latitude = value1.as_i64().unwrap();
                    let longitude = value2.as_i64().unwrap();

                    let coordinates = Coordinates {
                        lat: latitude as i8,
                        long: longitude as i16,
                    };

                    let result = weather::get_aurora_prediction(coordinates, &self.client).await;

                    match result {
                        Ok(probability) => {
                            format!(
                                "There is a {}% chance to see an aurora at ({},{})",
                                probability.value, coordinates.lat, coordinates.long
                            )
                        }
                        Err(err) => {
                            format!("Err: {err}")
                        }
                    }
                }
                "predict-by-location" => {
                    let city_option = command
                        .data
                        .options
                        .iter()
                        .find(|opt| opt.name == "city")
                        .cloned();

                    handle_predict_by_location(city_option, &self.client).await
                }
                command => unreachable!("Unknown command: {}", command),
            };

            let data = CreateInteractionResponseMessage::new().content(response_content);
            let builder = CreateInteractionResponse::Message(data);

            if let Err(why) = command.create_response(&ctx.http, builder).await {
                println!("Cannot respond to slash command: {why}");
            }
        }
    }
}

#[shuttle_runtime::main]
async fn serenity(
    #[shuttle_runtime::Secrets] secret_store: SecretStore,
) -> shuttle_serenity::ShuttleSerenity {
    // Get the discord token set in `Secrets.toml`
    let discord_token = secret_store
        .get("DISCORD_TOKEN")
        .context("'DISCORD_TOKEN' was not found")?;

    let discord_guild_id = secret_store
        .get("DISCORD_GUILD_ID")
        .context("'DISCORD_GUILD_ID' was not found")?;

    let client = get_client(&discord_token, discord_guild_id.parse().unwrap()).await;
    Ok(client.into())
}

pub async fn get_client(discord_token: &str, discord_guild_id: u64) -> Client {
    // Set gateway intents, which decides what events the bot will be notified about.
    // Here we don't need any intents so empty
    let intents = GatewayIntents::empty();

    Client::builder(discord_token, intents)
        .event_handler(Bot {
            client: reqwest::Client::new(),
            discord_guild_id: GuildId::new(discord_guild_id),
        })
        .await
        .expect("Err creating client")
}
