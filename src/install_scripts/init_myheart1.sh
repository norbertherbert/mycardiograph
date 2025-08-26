#!/bin/sh
### BEGIN INIT INFO
# Provides:       myheart
# Required-Start: $remote_fs $syslog
# Required-Stop:  $remote_fs $syslog
# Default-Start:  2 3 4 5
# Default-Stop:   0 1 6
# Short-Description: Start myheart daemon at boot time
# Description:      Enables myheart to run as a service.
### END INIT INFO

RUN_AS_USER="myheartuser"
DAEMON="/usr/local/bin/myheart"
DAEMON_NAME="myheart"
DAEMON_ARGS="--device-id Gw_01 --server-ip 192.168.1.200 --password asdf"
WORKING_DIR="/opt/myheart"

# Path to the PID file
PIDFILE="/var/run/$DAEMON_NAME.pid"

case "$1" in
  start)
    echo "Starting $DAEMON_NAME daemon..."
    # Using nohup to run in the background and & to detach
    su -s /bin/sh -c "nohup $DAEMON $DAEMON_ARGS &" $RUN_AS_USER
    echo "$!" > "$PIDFILE"
    echo "Started $DAEMON_NAME with PID $(cat $PIDFILE)"
    ;;
  stop)
    echo "Stopping $DAEMON_NAME daemon..."
    if [ -f "$PIDFILE" ]; then
      kill "$(cat "$PIDFILE")"
      rm "$PIDFILE"
      echo "$DAEMON_NAME daemon stopped."
    else
      echo "$DAEMON_NAME daemon not running."
    fi
    ;;
  restart)
    $0 stop
    sleep 2
    $0 start
    ;;
  status)
    if [ -f "$PIDFILE" ]; then
      PID=$(cat "$PIDFILE")
      if ps -p "$PID" > /dev/null; then
        echo "$DAEMON_NAME daemon is running with PID $PID"
        exit 0
      else
        echo "$DAEMON_NAME daemon is not running."
        exit 1
      fi
    else
      echo "$DAEMON_NAME daemon is not running."
      exit 1
    fi
    ;;
  *)
    echo "Usage: /etc/init.d/$DAEMON_NAME {start|stop|restart|status}"
    exit 1
    ;;
esac

exit 0
