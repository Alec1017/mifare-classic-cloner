
![banner](assets/logo.png)

## What is mfcc?

Mfcc is a way to facilitate easy cloning of mifare classic cards.

It is a wrapper around libnfc and mfoc that streamlines the decrypting and writing process.

Mfcc features include

- Writing a source file to a blank card
- Overwriting a previously written card with another source file

## Requirements

Requires [libnfc-1.7.1](https://github.com/nfc-tools/libnfc/releases/tag/libnfc-1.7.1) and a NFC contactless reader. I used a ACS NFC ACR122U RFID Contactless Smart IC Card Reader.


## Usage

### Installation 

```bash
cargo install mfcc
```


### Write to a blank mifare classic card

```bash
mfcc write-blank <card-uid> <source-file>
```


#### Overwrite a mifare classic card

```bash
mfcc overwite --key <-file> <source-file>
```