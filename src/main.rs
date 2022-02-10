extern crate clap;
extern crate colorful;
extern crate k8s_openapi;
extern crate kube;

use clap::Parser;
use colorful::Colorful;
use k8s_openapi::api::core::v1::{Namespace, Secret};
use kube::{Api, Client};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Config {
    #[clap(short='a', long)]
    show_all: bool,

    namespace: String,
    query: Option<String>,
}

fn display_secret(c: &Config, s: &Secret) -> bool {
    if c.show_all || s.type_.as_ref().unwrap() == "Opaque" {
        let query = match &c.query {
            Some(q) => q,
            None => return true
        };

        // Filter the name against our query
        let secret_name = s.metadata.name.as_ref().unwrap();
        if secret_name.contains(query) {
            return true;
        }
        return false;
    } else {
        return false;
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = Config::parse();

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
