#!/bin/sh
### BEGIN INIT INFO
# Provides:          myheart
# Required-Start:    $remote_fs $syslog
# Required-Stop:     $remote_fs $syslog
# Default-Start:     2 3 4 5
# Default-Stop:      0 1 6
# Short-Description: Start myheart daemon at boot time
# Description:       Enables myheart to run as a service.
### END INIT INFO

RUN_AS_USER="myheartuser"
DAEMON="/usr/local/bin/myheart"
DAEMON_NAME="myheart"
DAEMON_ARGS="--device-id Gw_01 --server-ip 192.168.1.200 --password asdf"
WORKING_DIR="/opt/myheart"

# Path to the PID file
PIDFILE="/var/run/$DAEMON_NAME.pid"

# Source function library.
. /lib/lsb/init-functions

case "$1" in
  start)
    log_daemon_msg "Starting system $DAEMON_NAME daemon"
    # Added the --chdir option to set the working directory
    start-stop-daemon --start --background --pidfile $PIDFILE --make-pidfile --chuid $RUN_AS_USER --chdir $WORKING_DIR --startas $DAEMON -- $DAEMON_ARGS
    log_end_msg $?
    ;;
  stop)
    log_daemon_msg "Stopping system $DAEMON_NAME daemon"
    start-stop-daemon --stop --pidfile $PIDFILE --retry 10
    log_end_msg $?
    ;;
  restart)
    $0 stop
    $0 start
    ;;
  status)
    status_of_proc -p $PIDFILE "$DAEMON_NAME" && exit 0 || exit $?
    ;;
  *)
    echo "Usage: /etc/init.d/$DAEMON_NAME {start|stop|restart|status}"
    exit 1
    ;;
esac

exit 0
