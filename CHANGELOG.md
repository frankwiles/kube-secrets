# kube-secrets Changelog

## 0.7.1  

- Detect if appropriate to use shell colors 

## 0.7.0 

- Crate updates 
- socks5 support for kube api (thanks Baptiste! https://github.com/baprx) 
- Added AUTHORS.md file 

## 0.6.0 

- New build Github Actions
- Homebrew support 

## 0.5.0

- Move to clap 3.0.14 and to using derive rather than building our command line
  option parsing directly

## 0.4.2

- Fix panic bug when decoding invalid UTF-8

## 0.4.1

- Initial release
- Support viewing all secrets in a namespace decoded
- Support viewing all Opaque secrets, decoded, in a namespace by default
