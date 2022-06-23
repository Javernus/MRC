import time

import meshtastic
import meshtastic.serial_interface
from pubsub import pub

interface = meshtastic.serial_interface.SerialInterface()

ourNode = interface.getNode('^local')
print(f'Our node preferences:{ourNode.radioConfig.preferences}')


def onReceive(packet, interface):  # called when a packet arrives
    print(f"Received: {packet}")


def onConnection(interface, topic=pub.AUTO_TOPIC):  # called when we (re)connect to the radio
    # defaults to broadcast, specify a destination ID if you wish
    interface.sendText("hello mesh")


pub.subscribe(onReceive, "meshtastic.receive.text")
pub.subscribe(onConnection, "meshtastic.connection.established")


while True:
    time.sleep(1000)

interface.close()