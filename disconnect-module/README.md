# Disconnect
This release brings a self-hostable Connect feature called "Disconnect". 

You can host the server yourself, or you can use my public at: https://qobine-disconnect.sofusconnect.addington.dk
Please don't abuse it 😄 

## How to use:
When running the rfid project provide the following flags:
```
      --disconnect-device-name <DISCONNECT_DEVICE_NAME>
          Disconnect device name
      --disconnect-password <DISCONNECT_PASSWORD>
          Disconnect password
      --disconnect-server-url <DISCONNECT_SERVER_URL>
          Disconnect server url
```

eg: `qobuz-player-rfid --disconnect-device-name livingroom --disconnect-password very-secret --disconnect-server-url https://qobine-disconnect.sofusconnect.addington.dk`

Running clients with the same password will be able to coordinate activation through the disconnect server.

## Why not Qobuz connect?
Good question. 

The official Qobuz Connect protocol is undocumented, and generally not fun to develop against. 
The needed code for "Disconnect" is also required for Qobuz Connect to control, activate and deactivate a player (client). 

Qobuz Connect would also allow us to use the official smartphone app for controlling. 
I personally don't use the official app, so this is not compelling for me.
