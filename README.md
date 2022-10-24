# cryptop

Command Line Interface for getting cryptocurrency prices and information

## Installation

```
cargo install cryptop
```

## Usage

### Get currency price:
```
cryptop --currency "<currency ticker>"
```
Example
```
cryptop --currency "BTC"
```

###Get currency price at a specified rate:
```
cryptop --currency "<currency ticker>" --rates "<rate>"
```
Example
```
cryptop --currency "BTC" --rates "CAD"
```
