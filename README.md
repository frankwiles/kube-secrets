# kube-secrets

This is a command line utility for quickly looking at secrets in a Kubernetes
namespace that are typically looked at by humans.  It specifically hides
secrets which are TLS certificates, Helm charts, and Docker credentials.

## Install

```shell
$ cargo install kube-secrets
```

## Usage

List all of the useful secrets in namespace `fakespace`. (These are fake secrets in a fake namespace in case you're worried)
```shell
$ secrets fakespace
```

![Screenshot of full listing](/images/fakespace.png)

List all of the secrets in namespace `fakespace` whose name contains the string `token`

```shell
$ secrets fakespace token
```

![Screenshot of filtered listing](/images/fakespace-token.png)

And if you mistakenly look for secrets in a namespace that doesn't actually exist, it let's you know that.

```shell
$ secrets bob
```

![Screenshot of error message](/images/bob.png)
