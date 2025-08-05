#!/bin/sh

# Exit immediately if a command exits with a non-zero status.
set -e

printf "\n--- 1. Stopping and disabling 'myheart' service ---\n"

# Stop the service if it's running. Ignore errors if it's already stopped.
if service myheart status >/dev/null 2>&1; then
    echo "Stopping service..."
    service myheart stop
else
    echo "Service is not running."
fi

# Disable the service from starting on boot.
echo "Disabling autostart..."
LC_ALL=C update-rc.d -f myheart remove

printf "\n--- 2. Deleting application files ---\n"

# Delete the init script
if [ -f /etc/init.d/myheart ]; then
    echo "Deleting init script: /etc/init.d/myheart"
    rm -f /etc/init.d/myheart
else
    echo "Init script not found."
fi

# Delete the binary
if [ -f /usr/local/bin/myheart ]; then
    echo "Deleting binary: /usr/local/bin/myheart"
    rm -f /usr/local/bin/myheart
else
    echo "Binary not found."
fi

# Delete the working directory
if [ -d /opt/myheart ]; then
    echo "Deleting working directory: /opt/myheart"
    rm -rf /opt/myheart
else
    echo "Working directory not found."
fi

printf "\n--- 3. Removing user 'myheartuser' ---\n"

# Check if the user exists
if id "myheartuser" >/dev/null 2>&1; then
    # First, make sure no processes are left running as this user
    echo "Checking for and stopping any remaining processes for 'myheartuser'..."
    pkill -u myheartuser || true # Ignore error if no processes are found

    # Delete the user
    echo "Deleting user 'myheartuser'..."
    userdel myheartuser
    echo "User 'myheartuser' has been deleted."
else
    echo "User 'myheartuser' does not exist."
fi

printf "\nCleanup complete. The 'myheart' service and all its components have been removed.\n"
