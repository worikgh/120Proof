#!/usr/bin/perl -w
use strict;
use lib("$ENV{Home120Proof}/Perl");
use One20Proof;

my $TIME =  scalar(localtime()) ;
warn "MARK $TIME ";

# Must have jack
`jack_wait -w`;
if($?){
    die "Failed waiting jack: $?\n";
}

## lpx_control must be running
if(!`pgrep lpx_control`){
    die "lpx_control must be running";
}


## Instruments
my $KEYS_INSTR = "$ENV{Home120Proof}/Instruments/xiz/Hammond Organ.xiz";
my $LPX_INSTR = "$ENV{Home120Proof}/Instruments/xiz/Simple Clonewheel.xiz";

## Kill these if they exist.  They would conflict with what is run here
## TODO: Put all the executable files in a configuration file
&One20Proof::pkill("$ENV{Home120Proof}/bin/lpx_manager");
&One20Proof::pkill('/usr/local/bin/yoshimi');


my $jack_name = 'SharkLipsKeys';
&One20Proof::initialise_yoshimi($jack_name, $KEYS_INSTR);

$jack_name = 'SharkLipsLPX';
&One20Proof::initialise_yoshimi($jack_name, $LPX_INSTR);
my $lpx_manager = &One20Proof::get_lpx_manager;
&One20Proof::run_daemon("$lpx_manager $ENV{Home120Proof}/Instruments/SharkLips/lpx_manager.cfg  57 5 17 113 1 4 7 8 11  ");


# Wait until lpx_manager is running
&One20Proof::wait_for_midi("120-Proof-MIDI-In-LPX") or
    die "120-Proof-MIDI-In-LPX not found";
&One20Proof::wait_for_midi("120-Proof-MIDI-Out-LPX") or
    die "120-Proof-MIDI-Out-LPX not found";
&One20Proof::wait_for_midi("120-Proof-MIDI-Out-PD") or
    die "120-Proof-MIDI-Out-PD not found";

#&One20Proof::run_daemon("$ENV{Home120Proof}/bin/InitialiseMidi $ENV{Home120Proof}/Instruments/SharkLips/midi.cfg ");
print STDERR `$ENV{Home120Proof}/bin/InitialiseMidi $ENV{Home120Proof}/Instruments/SharkLips/midi.cfg`;

warn "Set up MIDI";
