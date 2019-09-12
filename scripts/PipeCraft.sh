#!/bin/bash

# Command for launching the Minecraft server
MCCMD="java -jar server.jar -nogui"

# Command for launching the Bot
BOTCMD="./blocky"

# Keep the input pipe open (no EOF)
cat > mci &

# Start the bot
eval $BOTCMD < mco > mci &

# Start the Minecraft server with the pipes
eval $MCCMD < mci | tee mco

# Reap the bot
killall $BOTCMD