# aes

This library implements AES128ECB mode. This is for learning and for following along with the Cryptopals challenges.

The library passes the encryption and decryption test vectors for AES-128-ECB! This library takes in slices of &[u8; 16] at a time and encrypts or decrypts them. Padding must be handled separately for now, though it is in the plan to handle that soon.

## Usage

Add the following to your Cargo.toml under `[dependencies]`:
```
aes = { git = "https://github.com/genonullfree/aes.git" }
```

Add the following to your .rs file:
```
use ::aes::*;
```
