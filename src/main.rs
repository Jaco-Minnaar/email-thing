use std::net::TcpStream;

use clap::Parser;
use cli::{Cli, Command};
use imap::Session;
use keyring::Entry;
use native_tls::{TlsConnector, TlsStream};
use text_io::read;

use crate::config::Config;

mod cli;
mod config;

const SERVICE_NAME: &str = "email_widget";

fn main() {
    let args = Cli::parse();

    execute_args(&args);
}

fn execute_args(cli: &Cli) {
    match cli.command {
        Command::Setup => setup(),
        Command::Count => count(),
    }
}

fn setup() {
    println!("Please enter your email details below");
    println!();

    print!("Domain: ");
    let domain: String = read!("{}\n");

    print!("Server address: ");
    let server_addr: String = read!("{}\n");

    print!("IMAP port: ");
    let port: u16 = read!("{}\n");

    print!("Email address: ");
    let email_addr: String = read!("{}\n");

    let password = rpassword::prompt_password("Email password: ").unwrap();

    let config = Config::new(domain, server_addr, email_addr, port);

    confy::store(SERVICE_NAME, None, config).unwrap();

    save_password(&password);
}

fn count() {
    let config: Config = confy::load(SERVICE_NAME, None).unwrap();
    let password = get_password();

    let tls = TlsConnector::builder().build().unwrap();

    let client =
        imap::connect((config.server.as_ref(), config.port), &config.server, &tls).unwrap();

    let mut imap_session = client.login(&config.email_addr, password).unwrap();

    match get_unseen_count(&mut imap_session) {
        Ok(count) => println!("{count}"),
        Err(err) => eprintln!("{}", err.to_string()),
    };

    imap_session.logout().unwrap();
}

fn get_unseen_count(
    imap_session: &mut Session<TlsStream<TcpStream>>,
) -> imap::error::Result<usize> {
    imap_session.examine("INBOX")?;

    let query_result = imap_session.search("(UNSEEN)")?;

    Ok(query_result.len())
}

fn get_password() -> String {
    let username = whoami::username();
    let entry = Entry::new(SERVICE_NAME, &username);

    entry.get_password().unwrap()
}

fn save_password(password: &str) {
    let username = whoami::username();
    let entry = Entry::new(SERVICE_NAME, &username);

    entry.set_password(&password).unwrap();
}
