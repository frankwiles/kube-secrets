# kube-secrets Changelog

## 0.5.0

- Move to clap 3.0.14 and to using derive rather than building our command line
  option parsing directly

## 0.4.2

- Fix panic bug when decoding invalid UTF-8

## 0.4.1

- Initial release
- Support viewing all secrets in a namespace decoded
- Support viewing all Opaque secrets, decoded, in a namespace by default