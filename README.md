# kube-secrets

This is a command line utility for quickly looking at secrets in a Kubernetes
namespace that are typically looked at by humans.  It specifically hides
secrets which are TLS certificates, Helm charts, and Docker credentials.

## Install

```shell
$ cargo install kube-secrets
```

## Usage

List all of the useful secrets in namespace `foo`
```shell
$ secrets foo
```

List all of the secrets in namespace `foo` whose name contains the string `bob`

```shell
$ secrets foo bob
```

