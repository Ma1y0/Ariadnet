#!/usr/bin/env python

import socket

sock = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)

server_address = ("127.0.0.1", 8080)

message = bytes([1, 0, 0, 72, 105, 33])

try:
    # Send data
    sent = sock.sendto(message, server_address)
    print(f"Sent {sent} bytes to {server_address}")
except Exception as e:
    print(f"An error occurred: {e}")
finally:
    # Close the socket
    sock.close()
