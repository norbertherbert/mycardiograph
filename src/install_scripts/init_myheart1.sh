#!/bin/sh

RUN_AS_USER="myheartuser"
DAEMON="/usr/local/bin/myheart"
DAEMON_NAME="myheart"
DAEMON_ARGS="--device-id Gw_01 --server-ip 192.168.1.200 --password asdf"
WORKING_DIR="/opt/myheart"

# Store PID file inside working directory (user-writable)
PIDFILE="$WORKING_DIR/$DAEMON_NAME.pid"

start() {
    if [ -f "$PIDFILE" ] && kill -0 "$(cat "$PIDFILE")" 2>/dev/null; then
        echo "$DAEMON_NAME is already running."
        return 0
    fi

    echo "Starting $DAEMON_NAME..."
    cd "$WORKING_DIR" || exit 1

    # Launch daemon as myheartuser and write its own PID
    su -s /bin/sh "$RUN_AS_USER" -c "nohup $DAEMON $DAEMON_ARGS >/dev/null 2>&1 & echo \$! > $PIDFILE"
    echo "$DAEMON_NAME started."
}

stop() {
    if [ ! -f "$PIDFILE" ] || ! kill -0 "$(cat "$PIDFILE")" 2>/dev/null; then
        echo "$DAEMON_NAME is not running."
        return 0
    fi

    echo "Stopping $DAEMON_NAME..."
    kill "$(cat "$PIDFILE")" 2>/dev/null
    rm -f "$PIDFILE"
    echo "$DAEMON_NAME stopped."
}

status() {
    if [ -f "$PIDFILE" ] && kill -0 "$(cat "$PIDFILE")" 2>/dev/null; then
        echo "$DAEMON_NAME is running with PID $(cat "$PIDFILE")."
        return 0
    else
        echo "$DAEMON_NAME is not running."
        return 1
    fi
}

case "$1" in
    start) start ;;
    stop) stop ;;
    restart) stop; start ;;
    status) status ;;
    *)
        echo "Usage: $0 {start|stop|restart|status}"
        exit 1
        ;;
esac

exit 0
