# MyCardiograph

Light-weight availability monitoring solution for embedded Linux systems.

## Build

### Build the MyCardiograph monitoring server for your platform

```bash
cargo build --bin mycardiograph --release
```

### Build the MyHeart monitoring client for ARMv7

```bash
cargo install cross
cross build --bin myheart --target=armv7-unknown-linux-gnueabihf --release
```

## Use

### Copy the MyHeart client to your embedded linux system

```bash
scp ./target/armv7-unknown-linux-gnueabihf/release/myheart root@192.168.1.163:/root
# Modify the username, hostname and target folder according to your environment
```

### Run the MyHeart client on your embedded linux system

```bash
ssh root@192.168.1.163
./myheart --device-id OutdoorAP --server-ip 192.168.1.200 --password asdf
# Modify the 'server-ip' and 'password' according to your environment
```

### Run the server on your computer

```bash
./target/release/mycardiograph --password asdf
# Modify the 'password' according to your environment
```
