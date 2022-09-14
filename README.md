# World Cup Stickers CLI

## Install 

## linux

build: 

```cargo build --bin stickers -r```

add to path: 

```echo "export PATH=./target/release:\${PATH}" >> ~/.zshrc```


## Examples

### Show
```stickers show bra``` 
show all the stickers from BRAZIL

```bash
stickers show bra --missing
```
show all the MISSING stickers from BRAZIL
```
Team(BRA)
        - id: 11, name: Philippe Coutinho 
                collected: false, 
                repeated: false, 
                quantity: 0
        - id: 18, name: Raphinha 
                collected: false, 
                repeated: false, 
                quantity: 0
        - id: 14, name: Lucas Paquetá 
                collected: false, 
                repeated: false, 
                quantity: 0
        - id: 15, name: Eder Antony 
                collected: false, 
                repeated: false, 
                quantity: 0
        - id: 13, name: Fred 
                collected: false, 
                repeated: false, 
                quantity: 0
        - id: 19, name: Richarlison 
                collected: false, 
                repeated: false, 
                quantity: 0
        - id: 12, name: Fabinho 
                collected: false, 
                repeated: false, 
                quantity: 0
        - id: 20, name: Vinícius Jr 
                collected: false, 
                repeated: false, 
                quantity: 0
        - id: 16, name: Gabriel Jesus 
                collected: false, 
                repeated: false, 
                quantity: 0
        - id: 17, name: Neymar Jr 
                collected: false, 
                repeated: false, 
                quantity: 0
```


```shell
stickers show bra --repeated
```
show all the REPEATED stickers from BRAZIL
```Team(BRA)
        - id: 1, name: Team 
                collected: true, 
                repeated: true, 
                quantity: 4
        - id: 10, name: Casemiro 
                collected: true, 
                repeated: true, 
                quantity: 3
```


### Collect

```bash
stickers collect BRA 1 2 3 4 5 6 7 8 9 10 10 10 1 1 1
``` 
collect all these stickers from BRAZIL: 1 2 3 4 5 6 7 8 9 10 10 10 1 1 1


### Trade
```
stickers trade BRA 1 2 3 4 5 6 7 8 9 10 10 10 1 1 1
``` 

trade all these stickers from BRAZIL: 1 2 3 4 5 6 7 8 9 10 10 10 1 1 1
- only remove if you have repeated from specific sticker

If you try to trade a not repeated sticker:
```
> stickers trade BRA 12
Não foi possível trocar esta figurinha pois você não tem repetida: 12 Fabinho
```              

## TBD
- export as csv or another type of export media
- finish include each player name for each nation
