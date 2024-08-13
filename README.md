readme backup i'm making a pull:
# Multiplayer-fps

This project is a game based on the old game [maze wars](https://www.youtube.com/watch?v=5V5X5SbSjns). Most of all the elements of the game has been recreated.

## Objectives

### User Interface

The game present a specific User Interface, in which there is:

- A mini map where the player can see his own position and the whole game world.
- The graphics of the game (walls and other players) are similar to the original game (see maze_wars for more details)
- Finally the game display the frame rate on the screen.

### Architecture

- We use the client-server architecture where clients connect to a central server to play the game.
- Our implementation allow one client and the server to run on the same machine, with other clients connecting from different machines.
- We are using the UDP protocol to enable the communication between the clients and the server.
- The game has 3 levels with increasing difficulty that you can choose when running the server.
- The server must accept as many connections as possible (the minimum should be 10).
- When the client is initialized, the game asks for:

  1. The IP address of the server, allowing the client application to connect to any server.

  2. A username for identification.

  3. And a level of difficulty(1 to 3).

**Example:**

    $ cargo run
    Enter IP Address: 198.1.1.34:34254
    Enter Name: name
    Choose a level: 1
    Starting...

### Performance

The game is always above 50 fps (frames per second).

### Playing the game

- Running the server:

```
multiplayer-fps$ cd server
multiplayer-fps/server$ cargo run
Please enter the game level: 2
Starting server...
Server listening on 192.168.60.70:8081
```

- Running the client:

```
Enter Server IP Address (e.g., 11.11.90.13:1234): 192.168.60.70:8081
Enter Your Name: name

```

- After launching the game use the the directional keys to move the player and the space key to shoot at enemies.
- Use the minimap showing at the bottom of the screen to navigate the map.

### Implementation

- [Bevy game engine](https://bevyengine.org/)
- [Tokio](https://tokio.rs/)
- [bevy_rapier3d](https://rapier.rs/docs/user_guides/bevy_plugin/getting_started_bevy/)
- And many other crates that you can find in the cargo.toml

### Authors

- [sdakodo](https://learn.zone01dakar.sn/git/sdakodo)
- [bindoye](https://learn.zone01dakar.sn/git/bindoye)
- [djibsow](https://learn.zone01dakar.sn/git/djibsow)
- [fatouthiam2](https://learn.zone01dakar.sn/git/fatouthiam2)
- [alogou](https://learn.zone01dakar.sn/git/alogou)
- [ahbarry](https://learn.zone01dakar.sn/git/ahbarry)
- [pndione](https://learn.zone01dakar.sn/git/pndione)