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
    #[clap(short = 'a', long)]
    show_all: bool,

    namespace: String,
    query: Option<String>,
}

fn display_secret(c: &Config, s: &Secret) -> bool {
    if c.show_all || s.type_.as_ref().unwrap() == "Opaque" {
        let query = match &c.query {
            Some(q) => q,
            None => return true,
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

        if let Some(data) = &s.data {
            for (key, value) in data.iter() {
                let bstring = std::str::from_utf8(&value.0);
                match bstring {
                    Ok(bstring) => println!("  {}: {}", key.clone().light_green(), bstring),
                    Err(_) => println!("  {}: <unable to decode UTF-8>", key.clone().light_green()),
                }
                found_secrets += 1;
            }
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

#[cfg(test)]
mod tests {
    use super::*;
    use k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta;

    /// Helper to create a test Secret
    fn test_secret(name: &str, type_: &str) -> Secret {
        Secret {
            metadata: ObjectMeta {
                name: Some(name.to_string()),
                ..Default::default()
            },
            type_: Some(type_.to_string()),
            ..Default::default()
        }
    }

    // ============================================================================
    // Clap Argument Parsing Tests
    // ============================================================================

    #[test]
    fn test_parse_namespace_only() {
        let args = ["secrets", "default"];
        let config = Config::try_parse_from(&args).unwrap();
        assert_eq!(config.namespace, "default");
        assert!(!config.show_all);
        assert!(config.query.is_none());
    }

    #[test]
    fn test_parse_with_show_all_short() {
        let args = ["secrets", "-a", "default"];
        let config = Config::try_parse_from(&args).unwrap();
        assert_eq!(config.namespace, "default");
        assert!(config.show_all);
        assert!(config.query.is_none());
    }

    #[test]
    fn test_parse_with_show_all_long() {
        let args = ["secrets", "--show-all", "default"];
        let config = Config::try_parse_from(&args).unwrap();
        assert_eq!(config.namespace, "default");
        assert!(config.show_all);
        assert!(config.query.is_none());
    }

    #[test]
    fn test_parse_with_query() {
        let args = ["secrets", "default", "token"];
        let config = Config::try_parse_from(&args).unwrap();
        assert_eq!(config.namespace, "default");
        assert_eq!(config.query, Some("token".to_string()));
        assert!(!config.show_all);
    }

    #[test]
    fn test_parse_with_all_options() {
        let args = ["secrets", "-a", "kube-system", "cert"];
        let config = Config::try_parse_from(&args).unwrap();
        assert_eq!(config.namespace, "kube-system");
        assert_eq!(config.query, Some("cert".to_string()));
        assert!(config.show_all);
    }

    #[test]
    fn test_parse_empty_args_fails() {
        let args = ["secrets"];
        let result = Config::try_parse_from(&args);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_invalid_short_option_fails() {
        let args = ["secrets", "-x", "default"];
        let result = Config::try_parse_from(&args);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_invalid_long_option_fails() {
        let args = ["secrets", "--invalid", "default"];
        let result = Config::try_parse_from(&args);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_help_flag() {
        let args = ["secrets", "--help"];
        let result = Config::try_parse_from(&args);
        // Clap exits for --help, so this should fail in a test context
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_version_flag() {
        let args = ["secrets", "--version"];
        let result = Config::try_parse_from(&args);
        // Clap exits for --version, so this should fail in a test context
        assert!(result.is_err());
    }

    // ============================================================================
    // display_secret Logic Tests
    // ============================================================================

    #[test]
    fn test_display_secret_opaque_no_query() {
        let config = Config {
            show_all: false,
            namespace: "default".to_string(),
            query: None,
        };
        let secret = test_secret("my-secret", "Opaque");
        assert!(display_secret(&config, &secret));
    }

    #[test]
    fn test_display_secret_opaque_with_matching_query() {
        let config = Config {
            show_all: false,
            namespace: "default".to_string(),
            query: Some("token".to_string()),
        };
        let secret = test_secret("api-token", "Opaque");
        assert!(display_secret(&config, &secret));
    }

    #[test]
    fn test_display_secret_opaque_with_non_matching_query() {
        let config = Config {
            show_all: false,
            namespace: "default".to_string(),
            query: Some("cert".to_string()),
        };
        let secret = test_secret("api-token", "Opaque");
        assert!(!display_secret(&config, &secret));
    }

    #[test]
    fn test_display_secret_non_opaque_filtered_by_default() {
        let config = Config {
            show_all: false,
            namespace: "default".to_string(),
            query: None,
        };
        let secret = test_secret("tls-cert", "kubernetes.io/tls");
        assert!(!display_secret(&config, &secret));
    }

    #[test]
    fn test_display_secret_non_opaque_shown_with_show_all() {
        let config = Config {
            show_all: true,
            namespace: "default".to_string(),
            query: None,
        };
        let secret = test_secret("tls-cert", "kubernetes.io/tls");
        assert!(display_secret(&config, &secret));
    }

    #[test]
    fn test_display_secret_show_all_filters_on_query() {
        let config = Config {
            show_all: true,
            namespace: "default".to_string(),
            query: Some("tls".to_string()),
        };
        let secret = test_secret("my-tls-cert", "kubernetes.io/tls");
        assert!(display_secret(&config, &secret));
    }

    #[test]
    fn test_display_secret_show_all_filters_out_non_matching() {
        let config = Config {
            show_all: true,
            namespace: "default".to_string(),
            query: Some("db".to_string()),
        };
        let secret = test_secret("tls-cert", "kubernetes.io/tls");
        assert!(!display_secret(&config, &secret));
    }

    #[test]
    fn test_display_secret_case_sensitive_query() {
        let config = Config {
            show_all: true,
            namespace: "default".to_string(),
            query: Some("TOKEN".to_string()),
        };
        let secret = test_secret("api-token", "Opaque");
        assert!(!display_secret(&config, &secret));
    }

    #[test]
    fn test_display_secret_query_matches_substring() {
        let config = Config {
            show_all: false,
            namespace: "default".to_string(),
            query: Some("api".to_string()),
        };
        let secret = test_secret("my-api-credentials", "Opaque");
        assert!(display_secret(&config, &secret));
    }

    // ============================================================================
    // Common Kubernetes Secret Types
    // ============================================================================

    #[test]
    fn test_display_secret_docker_config_filtered() {
        let config = Config {
            show_all: false,
            namespace: "default".to_string(),
            query: None,
        };
        let secret = test_secret("docker-creds", "kubernetes.io/dockerconfigjson");
        assert!(!display_secret(&config, &secret));
    }

    #[test]
    fn test_display_secret_docker_config_shown_with_show_all() {
        let config = Config {
            show_all: true,
            namespace: "default".to_string(),
            query: None,
        };
        let secret = test_secret("docker-creds", "kubernetes.io/dockerconfigjson");
        assert!(display_secret(&config, &secret));
    }

    #[test]
    fn test_display_secret_service_account_token_filtered() {
        let config = Config {
            show_all: false,
            namespace: "default".to_string(),
            query: None,
        };
        let secret = test_secret("sa-token", "kubernetes.io/service-account-token");
        assert!(!display_secret(&config, &secret));
    }

    #[test]
    fn test_display_secret_bootstrap_token_filtered() {
        let config = Config {
            show_all: false,
            namespace: "default".to_string(),
            query: None,
        };
        let secret = test_secret("bootstrap-token", "bootstrap.kubernetes.io/token");
        assert!(!display_secret(&config, &secret));
    }
}
