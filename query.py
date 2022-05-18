#!/usr/bin/python3

import socket
import sys

client = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
client.connect((sys.argv[1], 25565))

client.send(b'\x15\x00\xf6\x05\x0e' + str.encode(sys.argv[1]) + b'\x63\xdd\x01')
client.send(b'\x01\x00')
donnees = client.recv(1024)
print(donnees)
