#!/usr/bin/env python

import socket

sock = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)

server_address = ("127.0.0.1", 8080)

message = bytes([33, 255, 236])

try:
    # Send data
    sent = sock.sendto(message, server_address)
    print(f"Sent {sent} bytes to {server_address}")

    data = sock.recv(4096)
    print(f"Answer: {list(data)}")


except Exception as e:
    print(f"An error occurred: {e}")
finally:
    # Close the socket
    sock.close()
