import socket
import os
import meshtastic.serial_interface
from pubsub import pub

# By default will try to find a meshtastic device, otherwise provide a device path like /dev/ttyUSB0
interface = meshtastic.serial_interface.SerialInterface()

socket_path = "ipc.sock"

try:
    os.unlink(socket_path)
except OSError:
    pass

s = socket.socket(socket.AF_UNIX, socket.SOCK_STREAM)
s.bind(socket_path)

s.listen()


def onReceive(packet, interface):  # called when a packet arrives
    print("Received message " + str(packet["decoded"]["payload"].decode('utf-8').rstrip('\n')) + " on port " + str(
        packet["decoded"]["portnum"]))
    conn.send(packet["decoded"]["payload"])


pub.subscribe(onReceive, "meshtastic.receive.data")

while True:
    conn, addr = s.accept()
    try:
        while True:
            data = conn.recv(256)
            if data:
                print("received ", data)
                interface.sendData(
                    # bytes(data.encode('utf-8')),
                    data,
                    destinationId='^all',
                    portNum=69,
                    wantAck=False,
                    wantResponse=False,
                    hopLimit=None,
                    onResponse=None,
                    channelIndex=0
                )
    finally:
        conn.close()
