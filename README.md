# ikea_or_fake
A guessing game where you can try to find out if the given name is an IKEA item or a generated name

## How to use

 - Install Rust if you haven't already
 ```bash
 curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
 rustup default stable
 ```
 
 - Clone repo
 ```bash
 git clone https://github.com/Ten0/ikea_or_fake.git
 ```
 
 - Run
 ```bash
 cd ikea_or_fake
 cargo run --release
 ```
 
## Example run
```
Is RINGSJÖ an ikea item ?
y
YOU ARE CORRECT!
Is LINDSDAL an ikea item ?
y
YOU ARE WRONG!
Is LIMMA an ikea item ?
n
YOU ARE WRONG!
Is RATIONELLA an ikea item ?
n
YOU ARE WRONG!
Is STEK an ikea item ?
y
YOU ARE WRONG!
Is TRILLING an ikea item ?
y
YOU ARE WRONG!
Is TRILLINGE an ikea item ?
y
YOU ARE CORRECT!
Is TAFJORD an ikea item ?
y
YOU ARE WRONG!
Is LONEVÅG an ikea item ?
y
YOU ARE WRONG!
Is RUTA an ikea item ?
n
YOU ARE CORRECT!
Is GIDEÅ an ikea item ?
y
YOU ARE WRONG!
Is PETRONES an ikea item ?

You got 3/11 (2TP, 6FP, 1TN, 2FN)
```
