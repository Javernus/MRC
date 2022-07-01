import meshtastic.serial_interface
from pubsub import pub
import time

# try to get interface
# interface = meshtastic.serial_interface.SerialInterface(devPath='/dev/DINGEN')
interface = meshtastic.serial_interface.SerialInterface()


def on_receive(packet):  # called when a packet arrives
    # to print whole packet:
    # print(packet)

    decoded = packet["decoded"]
    message = str(decoded["payload"].decode('utf-8').rstrip('\n'))
    portnum = str(decoded["portnum"])

    print(f"Received message {message} on port {portnum}")


# async function to handle incomming meshtastic messages
pub.subscribe(on_receive, "meshtastic.receive.data")

while True:
    time.sleep(1)
