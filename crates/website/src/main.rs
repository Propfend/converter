use std::{io::Result, net::SocketAddr};
use actix_web::{App, HttpServer};

use clap::{arg, Command};

mod main_router;

const DEFAULT_SERVER_ADDRESS: &str = "127.0.0.1:8080";

fn cli_configuration() -> Command {
    Command::new("converter")
        .about("Converter is a GUI for multimodal large language models usage")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .subcommand(
            Command::new("serve")
                .about("Starts the application server")
                .arg(
                    arg!(--addr <ADDR>)
                        .value_parser(clap::value_parser!(SocketAddr))
                        .default_value(DEFAULT_SERVER_ADDRESS),
            )
    )
}

async fn start_converter_server(
    server_address: &SocketAddr
) -> Result<()> {
    HttpServer::new(|| {
        App::new()
        .configure(main_router::register)
    })
    .bind(server_address)?
    .run()
    .await
}

#[actix_web::main]
async fn main() -> Result<()> {
    let command = cli_configuration().get_matches();

    match command.subcommand() {
        Some(("serve", sub_match)) => {
            let server_address = sub_match
                .get_one::<SocketAddr>("addr")
                .expect("Failed to parse address");

            start_converter_server(server_address).await?;
        }

        _ => unreachable!(),
    }

    Ok(())
}