# Echo

Echo is a simple DNS-ish server used by Ariadnet.

## Packet

Echo has two parts: a 3-byte header and an n-byte body.

### Header

The header is made of 3 bytes:

1. Version = 1
2. Method = Query = 0 | Answer = 1
3. Error = None = 0 | Parse = 1 | Not Found = 2 | Wrong Version = 3 | Internal Server Error = 4

### Body

The body contains an utf8 encoded string. The body can be empty. The body contains either a query or an answer. The body can contain additional error information.
