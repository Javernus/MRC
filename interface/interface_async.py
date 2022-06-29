import os
import asyncio

import meshtastic.serial_interface
from pubsub import pub

interface = False

# message queues
incoming = []
outgoing = []


def get_interface():
    global interface

    if not interface:
        try:
            # By default will try to find a meshtastic device, otherwise provide a devPath like devPath='/dev/ttyUSB0'
            interface = meshtastic.serial_interface.SerialInterface()
        except FileNotFoundError:
            incoming.append("Error: interface not available")

    return interface


socket_path = "/tmp/ipc.sock"

# remove socket if it already exists
if os.path.exists(socket_path):
    os.remove(socket_path)


# try making a unix socket file
try:
    os.unlink(socket_path)
except OSError:
    pass


def on_receive(packet):  # called when a packet arrives
    decoded = packet["decoded"]
    message = str(decoded["payload"].decode('utf-8').rstrip('\n'))
    portnum = str(decoded["portnum"])

    print(f"Received message {message} on port {portnum}")

    # add message to incomming queue
    incoming.append(message)


# async function to handle incomming meshtastic messages
pub.subscribe(on_receive, "meshtastic.receive.data")


async def send_message():
    available_interface = get_interface()
    if not available_interface:
        return

    if outgoing:
        message = outgoing.pop(0)
        # print(f"sending {message} over network..")
        interface.sendData(
            bytes(message.encode('utf-8')),
            destinationId='^all',
            portNum=256,
            wantAck=False,
            wantResponse=False,
            hopLimit=7,
            onResponse=None,
            channelIndex=0
        )


async def to_socket(writer):
    if incoming:
        message = incoming.pop(0)
        # print(f"sending message {message} to socket..")
        encoded_message = message.encode('utf-8')
        writer.write(encoded_message)
        try:
            await writer.drain()
        except ConnectionResetError:
            #TODO
            pass

    asyncio.create_task(to_socket(writer))


async def from_socket(reader):
    # print("trying to read from socket..")
    data = await reader.readline()
    message = data.decode('utf-8')

    if message:
        print(f"received {message.strip()}, adding to outgoing queue..")
        outgoing.append(message)
        await send_message()


async def handle_client(reader, writer):
    print("Client connected")
    # check if interface is available
    get_interface()

    while True:
        write = asyncio.create_task(to_socket(writer))
        read = asyncio.create_task(from_socket(reader))
        await write
        await read


async def run_server():
    server = await asyncio.start_unix_server(handle_client, socket_path)

    async with server:
        await server.serve_forever()

asyncio.run(run_server())
