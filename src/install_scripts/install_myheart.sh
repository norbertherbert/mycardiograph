#!/bin/sh

# Exit immediately if a command exits with a non-zero status.
set -e

printf -- "--- 1. Creating system user 'myheartuser' ---\n"
useradd -r -s /sbin/nologin myheartuser
printf "User 'myheartuser' created.\n"

printf "\n--- 2. Setting up application binary ---\n"
cp ./myheart /usr/local/bin/myheart
chown root:myheartuser /usr/local/bin/myheart
chmod 750 /usr/local/bin/myheart
printf "Binary '/usr/local/bin/myheart' configured.\n"

printf "\n--- 3. Setting up working directory ---\n"
mkdir -p /opt/myheart
chown -R myheartuser:myheartuser /opt/myheart
printf "Working directory '/opt/myheart' configured.\n"

printf "\n--- 4. Setting up init script ---\n"
cp ./init_myheart.sh /etc/init.d/myheart
chmod 755 /etc/init.d/myheart
printf "Init script '/etc/init.d/myheart' configured.\n"

printf "\n--- 5. Enabling service to start on boot ---\n"
# Prefix with LC_ALL=C to avoid locale warnings on minimal systems
LC_ALL=C update-rc.d myheart defaults
printf "Service 'myheart' enabled.\n"

printf "\nSetup complete! It is recommended to reboot to test the autostart.\n"
