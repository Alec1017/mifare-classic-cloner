
![banner](assets/logo.png)

## What is mfcc?

Mfcc is a way to facilitate easy cloning of mifare classic cards

Mfcc features include

- Writing a source file to a blank card
- Overwriting a previously written card with another source file


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
