#!/usr/bin/perl -w
use strict;
use lib("$ENV{'Home120Proof'}/Perl");
use One20Proof;

my $time = scalar(localtime());
print "Start GhostWalking $time\n";

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

my $jack_name = 'GhostWalkingKeys';
my $midi_name =  'yoshimi-GhostWalkingKeys';

&One20Proof::run_daemon("$ENV{'Home120Proof'}/bin/InitialiseYos  GhostWalkingKeys '$ENV{'Home120Proof'}/Instruments/xiz/Hammond Organ.xiz'");
&One20Proof::wait_for_jack($jack_name) or die "Jack: $jack_name not found";
&One20Proof::wait_for_midi($midi_name) or die "$midi_name not found";


$jack_name = 'GhostWalkingLPX';
$midi_name =  'yoshimi-GhostWalkingKeys';

&One20Proof::run_daemon("$ENV{'Home120Proof'}/bin/InitialiseYos GhostWalkingLPX '$ENV{'Home120Proof'}/Instruments/xiz/Wide Bass.xiz'");
&One20Proof::wait_for_jack($jack_name) or die "Jack: $jack_name not found";
&One20Proof::wait_for_midi($midi_name) or die "$midi_name not found";

&One20Proof::run_daemon("$ENV{'Home120Proof'}/bin/lpx_manager $ENV{'Home120Proof'}/Instruments/GhostWalking/lpx_manager.cfg 69 1 4 6 9 11 < /dev/null ");

# Wait until lpx_manager is running
&One20Proof::wait_for_midi("120-Proof-MIDI-In-LPX") or
    die "120-Proof-MIDI-In-LPX not found";
&One20Proof::wait_for_midi("120-Proof-MIDI-Out-LPX") or
    die "120-Proof-MIDI-Out-LPX not found";
&One20Proof::wait_for_midi("120-Proof-MIDI-Out-PD") or
    die "120-Proof-MIDI-Out-PD not found";

#&One20Proof::run_daemon("$ENV{'Home120Proof'}/bin/InitialiseMidi $ENV{'Home120Proof'}/Instruments/GhostWalking/midi.cfg ");
print `$ENV{'Home120Proof'}/bin/InitialiseMidi $ENV{'Home120Proof'}/Instruments/GhostWalking/midi.cfg`;
# TODO: Get a way of of confirming MDI set up correctly
