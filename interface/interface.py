import socket
import os
import meshtastic.serial_interface
from pubsub import pub
import time

# By default will try to find a meshtastic device, otherwise provide a devPath like devPath='/dev/ttyUSB0'
interface = meshtastic.serial_interface.SerialInterface(devPath='/dev/ttyUSB0')

socket_path = "/tmp/ipc.sock"

# remove socket if it already exists
if os.path.exists(socket_path):
    os.remove(socket_path)


# try making a unix socket file
try:
    os.unlink(socket_path)
except OSError:
    pass

# messages queue
messages = []


def on_receive(packet):  # called when a packet arrives
    decoded = packet["decoded"]
    message = str(decoded["payload"].decode('utf-8').rstrip('\n'))
    portnum = str(decoded["portnum"])

    print(f"Received message {message} on port {portnum}")

    # add message to message array
    messages.append(message)


# async function to handle incomming meshtastic messages
pub.subscribe(on_receive, "meshtastic.receive.data")

# start socket server
s = socket.socket(socket.AF_UNIX, socket.SOCK_STREAM)
s.bind(socket_path)
s.listen()


# try to send the first item in the messages queue
def send_reply(connection):
    if not messages:
        print("no messages to send to client")

    else:
        message = messages.pop(0)
        print(f"Sending message {message} to client..")
        connection.send(bytes(message.encode('utf-8')))


def send_message(message: str):
    print(f"sending message {message} over network..")
    interface.sendData(
        bytes(message.encode('utf-8')),
        destinationId='^all',
        portNum=69,
        wantAck=False,
        wantResponse=False,
        hopLimit=None,
        onResponse=None,
        channelIndex=0
    )


while True:
    # wait for a connection
    connection, address = s.accept()
    print("client connected..")

    while True:
        print("starting loop")
        # check for new messages from socket
        # message = str(connection.recv(256))
        # if message != str(b''):
        #     send_message(message)

        time.sleep(1)

        send_reply(connection)

    # connection should never close
    # connection.close()


