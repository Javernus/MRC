import meshtastic.serial_interface
import sys

# try to get interface
# interface = meshtastic.serial_interface.SerialInterface(devPath='/dev/DINGEN')
interface = meshtastic.serial_interface.SerialInterface(devPath='/dev/ttyUSB1')

data = str(sys.argv[1])
print(f"sending {data}")

interface.sendData(
    bytes(data.encode('utf-8')),
    destinationId='^all',
    portNum=69,
    wantAck=False,
    wantResponse=False,
    hopLimit=None,
    onResponse=None,
    channelIndex=0
)

