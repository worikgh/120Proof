#!/usr/bin/perl -w
use strict;
use lib("$ENV{Home120Proof}/Perl");
use One20Proof;

my $time = scalar(localtime());
print "Start Sharklips $time\n";

## Kill these if they exist.  They would conflict with what is run here
&One20Proof::pkill('lpx_manager');
&One20Proof::pkill('yoshimi');

print "jack...\n";
# Must have jack
`jack_wait -w`;
if(!$0){
    die "Failed waiting jack: $0\n";
}
print "...jack\n";

## lpx_control must be running
if(!`pgrep lpx_control`){
    die "lpx_control must be running";
}

print "Set up synths...\n";
my $jack_name = 'SharkLipsKeys';
my $midi_name =  "yoshimi-$jack_name";

warn("MARK ");
&One20Proof::run_daemon("$ENV{Home120Proof}/bin/InitialiseYos  $jack_name '$ENV{Home120Proof}/Instruments/xiz/Hammond Organ.xiz'");
warn("MARK \$jack_name $jack_name ");
&One20Proof::wait_for_jack($jack_name) or die "Jack: $jack_name not found";
warn("MARK ");
&One20Proof::wait_for_midi($midi_name) or die "$midi_name not found";


warn("MARK ");
&One20Proof::run_daemon("$ENV{Home120Proof}/bin/InitialiseYos $jack_name '$ENV{Home120Proof}/Instruments/xiz/0004-DX Rhodes 4.xiz'");
warn("MARK \$jack_name $jack_name ");
&One20Proof::wait_for_jack($jack_name) or die "Jack: $jack_name not found";
warn("MARK ");
&One20Proof::wait_for_midi($midi_name) or die "$midi_name not found";

$jack_name = 'SharkLipsLPX';
$midi_name =  'yoshimi-SharkLipsLPX';

warn("MARK ");
&One20Proof::run_daemon("$ENV{Home120Proof}/bin/InitialiseYos $jack_name '$ENV{Home120Proof}/Instruments/xiz/0004-DX Rhodes 4.xiz'");
warn("MARK ");
&One20Proof::wait_for_jack($jack_name) or die "Jack: $jack_name not found";
&One20Proof::wait_for_midi($midi_name) or die "$midi_name not found";

print "...set up synths\n";

warn("MARK ");
&One20Proof::run_daemon("$ENV{Home120Proof}/bin/lpx_manager $ENV{Home120Proof}/Instruments/SharkLips/lpx_manager.cfg  57 1 4 7 8 11  ");

print "Running lpx_manager\n";
# Wait until lpx_manager is running
&One20Proof::wait_for_midi("120-Proof-MIDI-In-LPX") or
    die "120-Proof-MIDI-In-LPX not found";
&One20Proof::wait_for_midi("120-Proof-MIDI-Out-LPX") or
    die "120-Proof-MIDI-Out-LPX not found";
&One20Proof::wait_for_midi("120-Proof-MIDI-Out-PD") or
    die "120-Proof-MIDI-Out-PD not found";

print "All MIDI found\n";
#&One20Proof::run_daemon("$ENV{Home120Proof}/bin/InitialiseMidi $ENV{Home120Proof}/Instruments/SharkLips/midi.cfg ");
print `$ENV{Home120Proof}/bin/InitialiseMidi $ENV{Home120Proof}/Instruments/SharkLips/midi.cfg`;

print "Set up MIDI\n";
