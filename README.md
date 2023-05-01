# wsyeet
Reads data from a file and sends it to a websocket port line by line

## Usage Example
```
wsyeet --host 0.0.0.0 --port 9999 --file ./mock_data.txt --verbose --freq 200
```

## Options
### -h,--help
- Opens the help message and exit

### -v,--verbose
- Show log messages

### -H,--host HOST
- The WebSocket host address. Defaults to 127.0.0.1

### -p,--port PORT
- Sets the WebSocket port to send to. Defaults to 8888

### -f,--file FILE
- The file path to be read from.

### -F,--freq FREQ
- The frequency to send the data in ms. Defauls to 1000ms.

