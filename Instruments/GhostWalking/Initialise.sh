#!/bin/sh
set -e
LOGFILE=/home/patch/120Proof/Instruments/GhostWalking/run.log
TIME=`date`
echo ---------------------------------------- >> $LOGFILE
echo Start: $TIME >> $LOGFILE
echo Start GhostWalking

# Must have jack
jack_wait -w


## Kill these if they exist
pgrep lpx_manager && pkill lpx_manager
pgrep yoshimi && pkill yoshimi

while [ `pgrep lpx_manager` ] ;
do
    echo Wait for lpx_manager to quit
done

while [ `pgrep yoshimi` ] ;
do
    echo Wait for yoshimi to quit
done

echo GhostWalking: Set up >> $LOGFILE

echo GhostWalking: LPX sent to an organ >> $LOGFILE
/home/patch/120Proof/InitialiseYos GhostWalkingLPX '/home/patch/120Proof/Instruments/xiz/Hammond Organ.xiz' 2>&1 >> $LOGFILE &

echo GhostWalking: Keyboard sent to Rhodes Piano >> $LOGFILE
/home/patch/120Proof/InitialiseYos GhostWalkingKeys '/usr/share/yoshimi/banks/Rhodes/0004-DX Rhodes 4.xiz'  2>&1 >> $LOGFILE  &

while [ ! `jack_lsp |grep GhostWalkingLPX` ] ;
do
    echo Waiting for jack GhostWalkingLPX
    sleep 1
done

while [ ! `jack_lsp |grep GhostWalkingKeys` ] ;
do
    echo Waiting for jack GhostWalkingKeys
    sleep 1
done

## Mistris does this
# echo lpx_mode 1
# /home/patch/120Proof/lpx_mode 1
# echo lpx_mode 127
# /home/patch/120Proof/lpx_mode 127

echo Running lpx_manager >> $LOGFILE
/home/patch/120Proof/lpx_manager /home/patch/120Proof/Instruments/GhostWalking/lpx_manager.cfg 57 1 4 6 9 11 < /dev/null  2>&1 >> $LOGFILE  &

# echo GhostWalking: Sleep....
# sleep 5
echo GhostWalking: Set up MIDI connections >> $LOGFILE
/home/patch/120Proof/InitialiseMidi /home/patch/120Proof/Instruments/GhostWalking/midi.cfg 2>&1 >> $LOGFILE

echo GhostWalking set up >> $LOGFILE
