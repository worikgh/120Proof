#!/bin/sh
set -e
echo Start
pgrep lpx_manager && pkill lpx_manager
pgrep yoshimi && pkill yoshimi
echo killed
while [ `pgrep lpx_manager` ] ;
do
    echo Wait for lpx_manager to quit
done

while [ `pgrep yoshimi` ] ;
do
    echo Wait for yoshimi to quit
done

echo Sharklips: Set up 

echo Sharklips: LPX sent to an organ
/home/patch/120Proof/InitialiseYos SharkLipsLPX '/home/patch/120Proof/Instruments/xiz/Hammond Organ.xiz' &

echo Sharklips: Keyboard sent to Rhodes Piano
/home/patch/120Proof/InitialiseYos SharkLipsKeys '/usr/share/yoshimi/banks/Rhodes/0004-DX Rhodes 4.xiz' &

/home/patch/120Proof/lpx_mode 1 ; /home/patch/120Proof/lpx_mode 127
echo lpx_manager
/home/patch/120Proof/lpx_manager /home/patch/120Proof/Instruments/SharkLips/lpx_manager.cfg 57 1 4 7 8 11 < /dev/null &

# echo Sharklips: Sleep....
# sleep 5
echo Sharklips: Set up MIDI connections
/home/patch/120Proof/InitialiseMidi /home/patch/120Proof/Instruments/SharkLips/midi.cfg

echo SharkLips set up
