#!/usr/bin/perl -w
use strict;
use lib("$ENV{'Home120Proof'}/Perl");
use One20Proof;

my $time = scalar(localtime());
print "Start UltimateParty $time\n";

my $instrument_name = "UltimateParty";

my $initialise_yos = "$ENV{Home120Proof}/bin/InitialiseYos";
-e $initialise_yos or die "Not executable: $initialise_yos ";
my $xiz_dir = "$ENV{Home120Proof}/Instruments/xiz";
-d $xiz_dir or die "Not a directory: $xiz_dir";

## Kill these if they exist.  They would conflict with what is run here
&One20Proof::pkill("$ENV{'Home120Proof'}/lpx_manager");
&One20Proof::pkill('/usr/local/bin/yoshimi');


## lpx_control must be running
if(!`pgrep lpx_control`){
    die "lpx_control must be running";
}

my $jack_name_keys = $instrument_name.'Keys';
my $midi_name_keys =  "yoshimi-$jack_name_keys";
my $xiz_file = 'Hammond\ Organ';

&One20Proof::run_daemon("$initialise_yos $jack_name_keys $xiz_dir/$xiz_file.xiz");
&One20Proof::wait_for_jack($jack_name_keys) or die "Jack: $jack_name_keys not found";
&One20Proof::wait_for_midi($midi_name_keys) or die "MIDI: $midi_name_keys not found";


my $jack_name_lpx = $instrument_name.'LPX';
my $midi_name_lpx =  'yoshimi-'.$instrument_name.'LPX';
$xiz_file = '0004-DX\ Rhodes\ 4';
&One20Proof::run_daemon("$initialise_yos $jack_name_lpx $xiz_dir/$xiz_file.xiz");
&One20Proof::wait_for_jack($jack_name_lpx) or die "Jack: $jack_name_lpx not found";
&One20Proof::wait_for_midi($midi_name_lpx) or die "$midi_name_lpx not found";

&One20Proof::run_daemon("$ENV{Home120Proof}/bin/lpx_manager $ENV{Home120Proof}/Instruments/$instrument_name/lpx_manager.cfg   57 1 4 6 8 11 ");

# Wait until lpx_manager is running
&One20Proof::wait_for_midi("120-Proof-MIDI-In-LPX") or
    die "120-Proof-MIDI-In-LPX not found";
&One20Proof::wait_for_midi("120-Proof-MIDI-Out-LPX") or
    die "120-Proof-MIDI-Out-LPX not found";
&One20Proof::wait_for_midi("120-Proof-MIDI-Out-PD") or
    die "120-Proof-MIDI-Out-PD not found";

# sleep 5
## Set up MIDI

## MIDI notes to 120Proof from LPX


my $midi_lpx_to_manager = 'Launchpad X:Launchpad X MIDI 2	120-Proof-MIDI-In-LPX:120-Proof-MIDI-In-LPX-in';

## Send LPX MIDI to Yoshimi 

my $midi_manager_to_yoshimi = '120-Proof-MIDI-Out-PD:120-Proof-MIDI-Out-PD-out	'.$midi_name_lpx.':input';

## Send control MIDI from 120Proof to LPX
my $midi_manager_lpx = '120-Proof-MIDI-Out-LPX:120-Proof-MIDI-Out-LPX-out	Launchpad X:Launchpad X MIDI 1';

my $midi_keys_manager = 'Impact LX88+:Impact LX88+ MIDI 1'."\t".$midi_name_keys.':input';

# lpx_control: Send MIDI notes to manager for translation
my $midi_lpx_to_control = 'Launchpad X:Launchpad X MIDI 2	120-Proof-CTL:120-Proof-CTL-in';
my $midi_control_to_lpx = '120-Proof-CTL:120-Proof-CTL-out	Launchpad X:Launchpad X MIDI 1';

&One20Proof::run_daemon("$ENV{'Home120Proof'}/bin/InitialiseMidi $ENV{'Home120Proof'}/Instruments/$instrument_name/midi.cfg ");
