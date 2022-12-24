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
my $midi_name =  'yoshimi-GhostWalkingKeys';
    
&One20Proof::run_daemon("$ENV{'Home120Proof'}/bin/InitialiseYos  GhostWalkingKeys '$KEYS_INSTR'");
&One20Proof::wait_for_jack($jack_name) or die "Jack: $jack_name not found";
&One20Proof::wait_for_midi($midi_name) or die "$midi_name not found";


$jack_name = 'GhostWalkingLPX';
$midi_name =  'yoshimi-GhostWalkingKeys';

&One20Proof::run_daemon("$ENV{'Home120Proof'}/bin/InitialiseYos GhostWalkingLPX '$LPX_INSTR'");
&One20Proof::wait_for_jack($jack_name) or die "Jack: $jack_name not found";
&One20Proof::wait_for_midi($midi_name) or die "$midi_name not found";

&One20Proof::run_daemon("$ENV{'Home120Proof'}/bin/lpx_manager $ENV{'Home120Proof'}/Instruments/GhostWalking/lpx_manager.cfg 69 1 4 6 9 11 ");

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
