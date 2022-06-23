import meshtastic
import sys
import meshtastic.serial_interface
from pubsub import pub

# By default will try to find a meshtastic device, otherwise provide a device path like /dev/ttyUSB0
interface = meshtastic.serial_interface.SerialInterface()


def onReceive(packet, interface):  # called when a packet arrives
    print(f"Received: {packet}")
    # de-serialize data and


def onConnection(interface, topic=pub.AUTO_TOPIC):  # called when we (re)connect to the radio
    # defaults to broadcast, specify a destination ID if you wish
    interface.sendText("Connected to mesh network")


pub.subscribe(onReceive, "meshtastic.receive")
pub.subscribe(onConnection, "meshtastic.connection.established")

interface.sendText("Testing script")

# event loop, listen for input on stdin and send data
while True:
    data = sys.stdin.readline()
    if data:
        interface.sendData(
            data,
            destinationId='^all',
            portNum=256,
            wantAck=False,
            wantResponse=False,
            hopLimit=None,
            onResponse=None,
            channelIndex=0
        )
