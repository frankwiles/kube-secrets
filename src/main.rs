use clap::{App, Arg, ArgMatches};
use colorful::Colorful;
use k8s_openapi::api::core::v1::Secret;
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

    for s in secrets.list(&Default::default()).await? {
        let display = display_secret(&config, &s);
        if !display {
            continue;
        }
        println!("{}:", s.metadata.name.unwrap().light_blue());

        for (key, value) in s.data {
            let bstring = std::str::from_utf8(&value.0).unwrap();
            println!("  {}: {}", key.light_green(), bstring);
        }
        println!();
    }

    Ok(())
}
