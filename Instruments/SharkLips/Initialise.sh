#!/usr/bin/perl -w
use strict;
use lib("$ENV{'Home120Proof'}/Perl");
use One20Proof;

my $time = scalar(localtime());
print "Start Sharklips $time\n";

## Kill these if they exist.  They would conflict with what is run here
&One20Proof::pkill('lpx_manager');
&One20Proof::pkill('yoshimi');


# Must have jack
`jack_wait -w`;
if(!$0){
    die "Failed waiting jack: $0\n";
}

## lpx_control must be running
if(!`pgrep lpx_control`){
    die "lpx_control must be running";
}

my $jack_name = 'SharkLipsKeys';
my $midi_name =  "yoshimi-$jack_name";


&One20Proof::run_daemon("$ENV{'Home120Proof'}/bin/InitialiseYos  $jack_name '$ENV{'Home120Proof'}/Instruments/xiz/Hammond Organ.xiz'");
&One20Proof::wait_for_jack($jack_name) or die "Jack: $jack_name not found";
&One20Proof::wait_for_midi($midi_name) or die "$midi_name not found";


&One20Proof::run_daemon("$ENV{'Home120Proof'}/bin/InitialiseYos $jack_name '$ENV{'Home120Proof'}/Instruments/xiz/0004-DX Rhodes 4.xiz'");
&One20Proof::wait_for_jack($jack_name) or die "Jack: $jack_name not found";
&One20Proof::wait_for_midi($midi_name) or die "$midi_name not found";

 $jack_name = 'SharkLipsLPX';
$midi_name =  'yoshimi-SharkLipsLPX';

&One20Proof::run_daemon("$ENV{'Home120Proof'}/bin/InitialiseYos $jack_name '$ENV{'Home120Proof'}/Instruments/xiz/0004-DX Rhodes 4.xiz'");
&One20Proof::wait_for_jack($jack_name) or die "Jack: $jack_name not found";
&One20Proof::wait_for_midi($midi_name) or die "$midi_name not found";


&One20Proof::run_daemon("$ENV{'Home120Proof'}/bin/lpx_manager $ENV{'Home120Proof'}/Instruments/SharkLips/lpx_manager.cfg  57 1 4 7 8 11 < /dev/null ");

# Wait until lpx_manager is running
&One20Proof::wait_for_midi("120-Proof-MIDI-In-LPX") or
    die "120-Proof-MIDI-In-LPX not found";
&One20Proof::wait_for_midi("120-Proof-MIDI-Out-LPX") or
    die "120-Proof-MIDI-Out-LPX not found";
&One20Proof::wait_for_midi("120-Proof-MIDI-Out-PD") or
    die "120-Proof-MIDI-Out-PD not found";

#&One20Proof::run_daemon("$ENV{'Home120Proof'}/bin/InitialiseMidi $ENV{'Home120Proof'}/Instruments/SharkLips/midi.cfg ");
print `$ENV{'Home120Proof'}/bin/InitialiseMidi $ENV{'Home120Proof'}/Instruments/SharkLips/midi.cfg`;


# TIME=`date`
# echo Jack exists now: $TIME >> $LOGFILE

# ## Kill these if they exist
# pgrep lpx_manager && pkill lpx_manager
# pgrep yoshimi && pkill yoshimi

# while [ `pgrep lpx_manager` ] ;
# do
#     echo Wait for lpx_manager to quit
# done

# while [ `pgrep yoshimi` ] ;
# do
#     echo Wait for yoshimi to quit
# done

# echo Sharklips: Set up >> $LOGFILE

# echo Sharklips: LPX sent to an organ >> $LOGFILE
# /home/patch/120Proof/bin/InitialiseYos SharkLipsLPX '/home/patch/120Proof/Instruments/xiz/Hammond Organ.xiz' 2>&1 >> $LOGFILE &

# echo Sharklips: Keyboard sent to Rhodes Piano >> $LOGFILE
# /home/patch/120Proof/bin/InitialiseYos SharkLipsKeys '/usr/share/yoshimi/banks/Rhodes/0004-DX Rhodes 4.xiz'  2>&1 >> $LOGFILE  &

# while [ ! `jack_lsp |grep SharkLipsLPX` ] ;
# do
#     echo Waiting for jack SharkLipsLPX
#     sleep 1
# done

# while [ ! `jack_lsp |grep SharkLipsKeys` ] ;
# do
#     echo Waiting for jack SharkLipsKeys
#     sleep 1
# done

# ## Mistris does this
# # echo lpx_mode 1
# # /home/patch/120Proof/lpx_mode 1
# # echo lpx_mode 127
# # /home/patch/120Proof/lpx_mode 127

# echo Running lpx_manager >> $LOGFILE
# /home/patch/120Proof/lpx_manager /home/patch/120Proof/Instruments/SharkLips/lpx_manager.cfg 57 1 4 7 8 11 < /dev/null  2>&1 >> $LOGFILE  &

# # echo Sharklips: Sleep....
# # sleep 5
# echo Sharklips: Set up MIDI connections >> $LOGFILE
# /home/patch/120Proof/InitialiseMidi /home/patch/120Proof/Instruments/SharkLips/midi.cfg 2>&1 >> $LOGFILE

# echo SharkLips set up >> $LOGFILE
