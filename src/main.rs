extern crate clap;
extern crate colorful;
extern crate k8s_openapi;
extern crate kube;

use clap::{App, Arg, ArgMatches};
use colorful::Colorful;
use k8s_openapi::api::core::v1::{Namespace, Secret};
use kube::{Api, Client};

#[derive(Debug)]
struct Config {
    show_all: bool,
    namespace: String,
    query: String,
}

fn determine_config(matches: ArgMatches) -> Config {
    let show_all = matches.is_present("all");
    let namespace = matches.value_of("namespace").unwrap().to_string();
    let mut query = String::from("");

    if matches.is_present("query") {
        query = matches.value_of("query").unwrap().to_string();
    }

    let config = Config {
        show_all: show_all,
        namespace: namespace,
        query: query,
    };

    return config;
}

fn display_secret(c: &Config, s: &Secret) -> bool {
    if c.show_all || s.type_.as_ref().unwrap() == "Opaque" {
        // We should show the secret if we aren't querying
        if c.query.is_empty() {
            return true;
        }

        // Filter the name against our query
        let secret_name = s.metadata.name.as_ref().unwrap();
        if secret_name.contains(&c.query) {
            return true;
        }
        return false;
    } else {
        return false;
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let matches = App::new("secrets")
        .version("0.1.0")
        .author("Frank Wiles <frank@revsys.com>")
        .about("List commonly needed secrets in a decoded way")
        .arg(
            Arg::new("all")
                .about("Show all secret types")
                .short('a')
                .long("all"),
        )
        .arg(
            Arg::new("namespace")
                .about("Namespace to list secrets from")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::new("query")
                .about("Match secrets names by substring")
                .takes_value(true),
        )
        .get_matches();

    // Turn command line matches into our Config struct
    let config = determine_config(matches);

    let client = Client::try_default().await?;
    let secrets: Api<Secret> = Api::namespaced(client, &config.namespace);
    let mut found_secrets = 0;

    for s in secrets.list(&Default::default()).await? {
        let display = display_secret(&config, &s);
        if !display {
            continue;
        }
        println!("{}:", s.metadata.name.unwrap().light_blue());

        for (key, value) in s.data {
            let bstring = std::str::from_utf8(&value.0);
            match bstring {
                Ok(bstring) => println!("  {}: {}", key.light_green(), bstring),
                Err(_) => println!("  {}: <unable to decode UTF-8>", key.light_green()),
            }
            found_secrets += 1;
        }
        println!()
    }

    // If we didn't find any secrets in this namespace, check to see if the
    // namespace actually exists or not to give user a decent message
    if found_secrets == 0 {
        let client = Client::try_default().await?;
        let namespaces: Api<Namespace> = Api::all(client);
        let mut found = false;
        for n in namespaces.list(&Default::default()).await? {
            let name = n.metadata.name.unwrap();
            if name == config.namespace {
                found = true;
                break;
            }
        }

        if found == true {
            println!("No secrets found in namespace '{}'", config.namespace)
        } else {
            println!(
                "Namespace '{}' does not exist. Maybe you're looking at the wrong cluster?",
                config.namespace
            );
        }
    }

    Ok(())
}
