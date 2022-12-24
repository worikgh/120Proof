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
my $LPX_INSTR = "$ENV{'Home120Proof'}/Instruments/xiz/Wide Bass.xiz";


my $jack_name = 'GhostWalkingKeys';
&One20Proof::initialise_yoshimi($jack_name, $KEYS_INSTR);

$jack_name = 'GhostWalkingLPX';
&One20Proof::initialise_yoshimi($jack_name, $KEYS_INSTR);

my $lpx_manager = &One20Proof::get_lpx_manager;
-x $lpx_manager or die  "$!: Not: '$lpx_manager'";
my $lpx_manager_cfg = "$ENV{'Home120Proof'}/Instruments/GhostWalking/lpx_manager.cfg";
-r $lpx_manager_cfg or die  "$!: Not: '$lpx_manager_cfg'";
&One20Proof::run_daemon("$lpx_manager $lpx_manager_cfg 69 1 4 6 9 11 ");

# Wait until lpx_manager is running
&One20Proof::wait_for_midi("120-Proof-MIDI-In-LPX") or
    die "120-Proof-MIDI-In-LPX not found";
&One20Proof::wait_for_midi("120-Proof-MIDI-Out-LPX") or
    die "120-Proof-MIDI-Out-LPX not found";
&One20Proof::wait_for_midi("120-Proof-MIDI-Out-PD") or
    die "120-Proof-MIDI-Out-PD not found";

#&One20Proof::run_daemon("$ENV{'Home120Proof'}/bin/InitialiseMidi $ENV{'Home120Proof'}/Instruments/GhostWalking/midi.cfg ");
print STDERR `$ENV{'Home120Proof'}/bin/InitialiseMidi $ENV{'Home120Proof'}/Instruments/GhostWalking/midi.cfg`;
exit;
# TODO: Get a way of of confirming MIDI set up correctly
