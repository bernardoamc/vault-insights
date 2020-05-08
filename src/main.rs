use std::fs;
use anyhow::Result;
use structopt::StructOpt;
use serde::Deserialize;
use chrono::Datelike;

#[macro_use] extern crate prettytable;
use prettytable::{Table, Row, Cell};

mod vault;
use vault::Vault;

#[derive(Debug, Deserialize)]
struct Config {
    key: String,
    token: String,
    vault_url: String,
}

#[derive(Debug, StructOpt)]
#[structopt(
    name = "vault-insights",
    about = "What has been happening with your projects these days? Time to figure it out!"
)]
struct Opt {
    /// Project IDs from the vault
    #[structopt(short, long, raw(use_delimiter = "true"))]
    projects: Vec<u32>,

    /// Fetch projects updated since the amount of days specified
    #[structopt(short, long, default_value="14")]
    since_days_ago: i64,
}

fn main() -> Result<()> {
    let config: Config =  toml::from_str(&fetch_config_contents())?;
    validate_config(&config);
 
    let opts = Opt::from_args();
    let vault = Vault::new(&config.key[..], &config.token[..], &config.vault_url[..], &opts.projects);
    let projects = vault.fetch_projects()?;

    let mut outdated_table = Table::new();
    let mut updated_table = Table::new();

    outdated_table.add_row(row!["Name", "Updated At", "Comment URL"]);
    updated_table.add_row(row!["Name", "Updated At", "Comment URL"]);

    for project in projects {
        let formatted_date = project.updated_at.as_ref().map_or(
            "--".to_owned(),
            |u| format!("{}-{:02}-{:02}", u.year(), u.month(), u.day())
        );

        let comment_url = project.comment_url.as_ref().map_or("",|c| &c);

        if project.is_updated(opts.since_days_ago) {
            updated_table.add_row(Row::new(vec![
                Cell::new(&project.name),
                Cell::new(&formatted_date),
                Cell::new(&comment_url)]));
        } else {
            outdated_table.add_row(Row::new(vec![
                Cell::new(&project.name),
                Cell::new(&formatted_date),
                Cell::new(&comment_url)]));
        }
    }

    println!("OUTDATED:");
    outdated_table.printstd();
    println!("\n");
    println!("UPDATED:");
    updated_table.printstd();

    Ok(())
}

fn fetch_config_contents() -> String {
    let mut config_path = dirs::home_dir().unwrap();
    config_path.push(".config/vault-insights");

    fs::read_to_string(config_path)
        .expect("Please create a configuration file located in ~/.config/vault-insights")
}

fn validate_config(config: &Config) {
    if config.key.is_empty() || config.key.is_empty() || config.key.is_empty() {
        let error_message = r#"
        Invalid credentials.
        Make sure key, token and vault_url are set inside ~/.config/vault-insights in the format:

        key = abc123
        token = def456
        vault_url = https://myapiurl.com
        "#;

        eprintln!("{}", error_message);
        std::process::exit(1);
    }
}
