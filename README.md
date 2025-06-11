# MyCardiograph

This is a lightweight availability monitoring solution for embedded Linux systems.

When you compile this code you will get 2 binary files:

- `mycardiograph` - the monitoring server that is pre-compiled for x86-linux
- `myheart` - the monitoring client that is pre-compiled for armv7-linux

The client sends periodic heartbeat messages to the server. All messages are encrypted and protected against reply attacs.

The server receives heartbeat messages from several clients and logs events/alerts in dayly rotating log files. Log fiiles are stored in the `log` folder that is located in the same folder from where the server was launched.

## Installation and setup

The pre-compiled binary files are availeble at [releases](https://github.com/norbertherbert/mycardiograph/releases) section of this repo.

On the monitoring server:

```bash
# Download the pre-compiled server binary file for your x86-linux machine 
curl -L https://github.com/norbertherbert/mycardiograph/releases/download/v0.1.0/mycardiograph-0.1.0-x86_64-unknown-linux-gnu -o mycardiograph

# Make the downloaded file executable
chmod +x mycardiograph

# Run the server
# Modify the 'password' according to your environment
./mycardiograph --password asdf
```

On the embedded linux clients:

```bash
# Download the pre-compiled client binary file for your armv7-linux machine 
curl -L https://github.com/norbertherbert/mycardiograph/releases/download/v0.1.0/myheart-0.1.0-armv7-unknown-linux-gnueabihf -o myheart

# Make the downloaded file executable
chmod +x myheart

# Run the client
# Modify the 'device-id', 'server-ip' and 'password' params according to your environment
./myheart --device-id OutdoorAP --server-ip 192.168.1.200 --password asdf
```

You can check all supported command line params by executing the binary files with the `--help` option.

## Build

Build the MyCardiograph monitoring server for your platform

```bash
cargo build --bin mycardiograph --release --target x86_64-unknown-linux-gnu
# cp target/x86_64-unknown-linux-gnu/release/mycardiograph releases/mycardiograph-0.1.0-x86_64-unknown-linux-gnu
```

Build the MyHeart monitoring client for ARMv7

```bash
cargo install cross
cross build --bin myheart --release --target=armv7-unknown-linux-gnueabihf
# cp target/armv7-unknown-linux-gnueabihf/release/myheart releases/myheart-0.1.0-armv7-unknown-linux-gnueabihf
```
