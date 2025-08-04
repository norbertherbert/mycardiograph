# Cardiograph

This is a lightweight availability monitoring solution for embedded Linux systems.

When you compile this code you will get 2 binary files:

- `cardiograph` - the monitoring server that is pre-compiled for x86-linux
- `myheart` - the monitoring client that is pre-compiled for armv7-linux

The client sends periodic heartbeat messages to the server. All messages are encrypted and protected against reply attacks.

The server receives heartbeat messages from several clients and logs events/alerts in daily rotating log files. The log files are stored in the `log` folder that is located in the same folder from where the server was launched.

Before executing the binary files for monitoring, please check all supported command line params with the `--help` command line option.

## Installation and setup

The pre-compiled binary files are availeble at the [releases](https://github.com/norbertherbert/mycardiograph/releases) section of this repo.

### installation on the monitoring server:

```bash
# Download the pre-compiled server binary file for your x86-linux machine 
curl -L https://github.com/norbertherbert/mycardiograph/releases/download/v0.1.1/cardiograph-0.1.1-x86_64-unknown-linux-gnu -o cardiograph

# OR download the pre-compiled server binary file for your x86-windows machine 
curl -L https://github.com/norbertherbert/mycardiograph/releases/download/v0.1.1/cardiograph-0.1.1-x86_64-pc-windows-msvc.exe  -o cardiograph.exe

# Make the downloaded file executable
chmod +x cardiograph

# Run the server
# Modify the 'password' according to your environment
./cardiograph --password asdf
```

### installation on the embedded linux clients:

```bash
# Download the pre-compiled client binary file for your armv7-linux machine 
curl -L https://github.com/norbertherbert/mycardiograph/releases/download/v0.1.1/myheart-0.1.1-armv7-unknown-linux-gnueabihf -o myheart

# Make the downloaded file executable
chmod +x myheart

# Run the client
# Modify the 'device-id', 'server-ip' and 'password' params according to your environment
./myheart --device-id OutdoorAP --server-ip 192.168.1.200 --password asdf
```

### autostart

If you want that your embedded Linux starts the application automatically create a `/etc/init.d/S99myheart` file with the following commmand:

```bash
vim /etc/init.d/S99myheart
```

... and the file shall include the following lines: 
```bash
#!/bin/sh

# path to your binary file
/root/myheart &
```

Then execute the following commands:
```bash
chmod +x /etc/init.d/S99myheart
ln -s /etc/init.d/S99myheart /etc/rcS.d/S99myheart
```

After that you caan reboot your system and check if the binary iss running...

## Build

Build the MyCardiograph monitoring server for your x86-linux platform

```bash
cargo build --bin cardiograph --release --target x86_64-unknown-linux-gnu
# cp target/x86_64-unknown-linux-gnu/release/cardiograph releases/cardiograph-0.1.1-x86_64-unknown-linux-gnu
```

Build the MyCardiograph monitoring server for your x86-windows platform

```bash
cargo build --bin cardiograph --release --target x86_64-pc-windows-msvc
# cp target/x86_64-pc-windows-msvc/release/cardiograph.exe releases/cardiograph-0.1.1-x86_64-pc-windows-msvc.exe
```

Build the MyHeart monitoring client for armv7-linux platform

```bash
cargo install cross
cross build --bin myheart --release --target=armv7-unknown-linux-gnueabihf
# cp target/armv7-unknown-linux-gnueabihf/release/myheart releases/myheart-0.1.1-armv7-unknown-linux-gnueabihf
```
