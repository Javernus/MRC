import meshtastic
import sys
import meshtastic.serial_interface
from pubsub import pub

# By default will try to find a meshtastic device, otherwise provide a device path like /dev/ttyUSB0
interface = meshtastic.serial_interface.SerialInterface()


def onReceive(packet, interface):  # called when a packet arrives
    print(packet)


pub.subscribe(onReceive, "meshtastic.receive")

# interface.sendText("Testing script")

# event loop, listen for input on stdin and send data
while True:
    data = sys.stdin.readline()
    if data:
        # interface.sendText(data)
        interface.sendData(
            bytes(data.encode('utf-8')),
            destinationId='^all',
            portNum=256,
            wantAck=False,
            wantResponse=False,
            hopLimit=None,
            onResponse=None,
            channelIndex=0
        )
