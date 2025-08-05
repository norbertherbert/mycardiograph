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

### installation on the monitoring server

```sh
# Download the pre-compiled server binary file for your x86-linux machine 
curl -L https://github.com/norbertherbert/mycardiograph/releases/download/v0.1.2/cardiograph-0.1.2-x86_64-unknown-linux-gnu -o cardiograph

# OR download the pre-compiled server binary file for your x86-windows machine 
curl -L https://github.com/norbertherbert/mycardiograph/releases/download/v0.1.2/cardiograph-0.1.2-x86_64-pc-windows-msvc.exe  -o cardiograph.exe

# Make the downloaded file executable
chmod +x cardiograph

# Run the server
# Modify the 'password' according to your environment
./cardiograph --password asdf
```

### installation on the embedded linux clients

```sh
# Login to your gateway as a root user and makee sure yyou are in the root's home folder.
root@Gw01:~# cd ~
root@Gw01:~# pwd
/root

# Download the pre-compiled client binary file for your armv7-linux machine 
curl -L https://github.com/norbertherbert/mycardiograph/releases/download/v0.1.1S/myheart-0.1.1-armv7-unknown-linux-gnueabihf -o myheart

# Make the downloaded file executable
chmod +x myheart

# Run the client
# Modify the 'device-id', 'server-ip' and 'password' params according to your environment
./myheart --device-id Gw01 --server-ip 192.168.1.200 --password asdf
```

### autostart

If you want the `myheart` monitoring app to start automatically after system boot, please follow the instructions below:

- download the installation scripts from the [installation_scripts](https://github.com/norbertherbert/mycardiograph/tree/main/src/install_scripts)
folder of the github repo and place them in the root's home folder next to the previosly downloaded
`myheart` binary file. You should see the following files in your home folder:

    ```txt
    root@Gw01:~# ls -l
    total 1744
    -rw-r--r-- 1 root root    1085 Aug  5 10:11 install_myheart.sh
    -rw-r--r-- 1 root root 1773068 Aug  5 10:11 myheart
    -rw-r--r-- 1 root root    1374 Aug  5 10:12 init_myheart.sh
    -rw-r--r-- 1 root root    1721 Aug  5 10:13 uninstall_myheart.sh
    ````

- Update the following line inside the `init_myheart.sh` script to define the parameters `myheart` starts with.

    ```sh
    DAEMON_ARGS="--device-id Gw01 --server-ip 192.168.1.200 --password asdf"
    ```

- Execute the `install_myheart.sh` script.

    ```sh
    sh ./install_myheart.sh
    ```

- After this, you can use the `/etc/init.d/myheart` command with the `start|stop|restart|status` options.
This offers you a convenient way to manage the `myheart` application.

## Build

This is optional. Prebuilt binary fiiles are shared at the [releases](https://github.com/norbertherbert/mycardiograph/releases)
folder of the github repo.

Build the MyCardiograph monitoring server for your `x86-linux` platform *(on an `x86-linux` platform)*

```bash
cargo build --bin cardiograph --release --target x86_64-unknown-linux-gnu
# cp target/x86_64-unknown-linux-gnu/release/cardiograph releases/cardiograph-0.1.2-x86_64-unknown-linux-gnu
```

Build the MyCardiograph monitoring server for your `x86-windows` platform *(on an `x86-windows` platform)*

```bash
cargo build --bin cardiograph --release --target x86_64-pc-windows-msvc
# cp target/x86_64-pc-windows-msvc/release/cardiograph.exe releases/cardiograph-0.1.2-x86_64-pc-windows-msvc.exe
```

Build the MyHeart monitoring client for `armv7-linux` platform *(on an `x86-linux` platform)*

```bash
cargo install cross
cross build --bin myheart --release --target=armv7-unknown-linux-gnueabihf
# cp target/armv7-unknown-linux-gnueabihf/release/myheart releases/myheart-0.1.1-armv7-unknown-linux-gnueabihf
```
