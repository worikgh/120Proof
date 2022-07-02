#!/bin/sh
set -e
LOGFILE=/home/patch/120Proof/startup.log
TIME=`date`

echo Start: $TIME >> $LOGFILE
WHOAMI=`whoami`
echo Start: $WHOAMI >> $LOGFILE

pgrep lpx_manager && pkill lpx_manager
pgrep yoshimi && pkill yoshimi
#echo killed
while [ `pgrep lpx_manager` ] ;
do
    echo Wait for lpx_manager to quit >> $LOGFILE
done

while [ `pgrep yoshimi` ] ;
do
    echo Wait for yoshimi to quit >> $LOGFILE
done

echo Sharklips: Set up >> $LOGFILE

echo Sharklips: LPX sent to an organ >> $LOGFILE
/home/patch/120Proof/InitialiseYos SharkLipsLPX '/home/patch/120Proof/Instruments/xiz/Hammond Organ.xiz' 2>&1 >> $LOGFILE &

echo Sharklips: Keyboard sent to Rhodes Piano >> $LOGFILE
/home/patch/120Proof/InitialiseYos SharkLipsKeys '/usr/share/yoshimi/banks/Rhodes/0004-DX Rhodes 4.xiz'  2>&1 >> $LOGFILE  &
ACONNECT_L=`aconnect -l`
echo $ACONNECT_L >> $LOGFILE
sleep 1
ACONNECT_L=`aconnect -l`
echo -- >> $LOGFILE
echo $ACONNECT_L >> $LOGFILE

/home/patch/120Proof/lpx_mode 1 ; /home/patch/120Proof/lpx_mode 127
echo lpx_manager >> $LOGFILE
/home/patch/120Proof/lpx_manager /home/patch/120Proof/Instruments/SharkLips/lpx_manager.cfg 57 1 4 7 8 11 < /dev/null  2>&1 >> $LOGFILE  &

# echo Sharklips: Sleep....
# sleep 5
echo Sharklips: Set up MIDI connections >> $LOGFILE
/home/patch/120Proof/InitialiseMidi /home/patch/120Proof/Instruments/SharkLips/midi.cfg 2>&1 >> $LOGFILE

echo SharkLips set up >> $LOGFILE
