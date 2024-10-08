# Apollo

Apollo is a web server for Ariadnet.

# Aethon

Aethon ("Blazing") is the name of protocol used by Ariadnet. It uses TCP under the hood.

## Request

```
-----------------------------------
| VERSION |  | METHOD |  | PATH |
-----------------------------------
| HEADER_KEY | : | HEADER_VALUE |
-----------------------------------
| \n |
-----------------------------------
| \n |
-----------------------------------
| BODY |
-----------------------------------
```

### VERSION

- Indicates the protocol's version
- [1]

### METHOD

- Indicates the METHOD
- [GET, POST, DELETE]

### PATH

- Indicated the request's path
- An UTF-8 string

### HEADER

- Additional information in KEY: VALUE format
- An UTF-8 string
- If VALUE's first character is an whitespace it's trimmed

### BODY

- The content of the packet
- An UTF-8 string
- It's divided from rest of the packet by double newline (\n\n)

## Response

```
-----------------------------------
| VERSION |  | STATUS |
-----------------------------------
| HEADER_KEY | : | HEADER_VALUE |
-----------------------------------
| \n |
-----------------------------------
| BODY |
-----------------------------------
```

### VERSION

- Indicates the protocol's version
- [1]

### STATUS

- Indicates STATUS of the response
- **Successful responses**
  - 200 OK
  - 201 Created
- **Client error responses**
  - 400 Bad Request
  - 404 Not Found
  - 405 Method Not Found
  - 418 I'm a teapot (The server refuses the attempt to brew coffee with a teapot.)
- **Server error responses**
  - 500 Internal Server Error

### HEADER

- Additional information in KEY: VALUE format
- An UTF-8 string
- If VALUE's first character is an whitespace it's trimmed

### BODY

- The content of the packet
- An UTF-8 string
