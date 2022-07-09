#!/bin/sh
set -e
LOGFILE=/home/patch/120Proof/Instruments/SharkLips/run.log
TIME=`date`
echo ---------------------------------------- >> $LOGFILE
echo Start: $TIME >> $LOGFILE
echo Start Sharklips

while [ ! `jack_lsp` ] ;
do
    echo Wait for Jack
    sleep 1
done


echo jack_wait -c
jack_wait -c
echo jack_wait -w
jack_wait -w

TIME=`date`
echo Jack exists now: $TIME >> $LOGFILE

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

echo Sharklips: Set up >> $LOGFILE

echo Sharklips: LPX sent to an organ >> $LOGFILE
/home/patch/120Proof/InitialiseYos SharkLipsLPX '/home/patch/120Proof/Instruments/xiz/Hammond Organ.xiz' 2>&1 >> $LOGFILE &

echo Sharklips: Keyboard sent to Rhodes Piano >> $LOGFILE
/home/patch/120Proof/InitialiseYos SharkLipsKeys '/usr/share/yoshimi/banks/Rhodes/0004-DX Rhodes 4.xiz'  2>&1 >> $LOGFILE  &

while [ ! `jack_lsp |grep SharkLipsLPX` ] ;
do
    echo Waiting for jack SharkLipsLPX
    sleep 1
done

while [ ! `jack_lsp |grep SharkLipsKeys` ] ;
do
    echo Waiting for jack SharkLipsKeys
    sleep 1
done

## Mistris does this
# echo lpx_mode 1
# /home/patch/120Proof/lpx_mode 1
# echo lpx_mode 127
# /home/patch/120Proof/lpx_mode 127

echo Running lpx_manager >> $LOGFILE
/home/patch/120Proof/lpx_manager /home/patch/120Proof/Instruments/SharkLips/lpx_manager.cfg 57 1 4 7 8 11 < /dev/null  2>&1 >> $LOGFILE  &

# echo Sharklips: Sleep....
# sleep 5
echo Sharklips: Set up MIDI connections >> $LOGFILE
/home/patch/120Proof/InitialiseMidi /home/patch/120Proof/Instruments/SharkLips/midi.cfg 2>&1 >> $LOGFILE

echo SharkLips set up >> $LOGFILE
