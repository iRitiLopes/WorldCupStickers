# World Cup Stickers CLI

## Install 

## linux

build: 

```cargo build --bin stickers -r```

add to path: 

```echo "export PATH=./target/release:\${PATH}" >> ~/.zshrc```

and use

```stickers show bra``` 
show all the stickers from BRAZIL

```stickers show bra --missing```
show all the MISSING stickers from BRAZIL

```stickers show bra --repeated```
show all the REPEATED stickers from BRAZIL

```stickers collect BRA 1 2 3 4 5 6 7 8 9 10 10 10 1 1 1``` 
collect all these stickers from BRAZIL: 1 2 3 4 5 6 7 8 9 10 10 10 1 1 1