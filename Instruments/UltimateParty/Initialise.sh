#!/usr/bin/perl -w
use strict;
use lib("$ENV{'Home120Proof'}/Perl");
use One20Proof;

my $time = scalar(localtime());
warn "MARK $time";

my $instrument_name = "UltimateParty";

my $xiz_dir = "$ENV{Home120Proof}/Instruments/xiz";
-d $xiz_dir or die "Not a directory: $xiz_dir";

## Kill these if they exist.  They would conflict with what is run here
## Programmes to kill
my $YOSHIMI=&One20Proof::get_yoshimi;
my $LPX_MANAGER=&One20Proof::get_lpx_manager;
my $PD=&One20Proof::get_pd;

&One20Proof::pkill($PD);
&One20Proof::pkill($YOSHIMI);
&One20Proof::pkill($LPX_MANAGER);

## lpx_control must be running
if(!`pgrep lpx_control`){
    die "lpx_control must be running";
}

my $jack_name_keys = $instrument_name.'Keys';
my $xiz_file = 'Hammond Organ';

&One20Proof::initialise_yoshimi($jack_name_keys, "$xiz_dir/$xiz_file.xiz");

my $jack_name_lpx = $instrument_name.'LPX';
$xiz_file = 'StrangeDays';
&One20Proof::initialise_yoshimi($jack_name_lpx,  "$xiz_dir/$xiz_file.xiz");


&One20Proof::run_daemon("$LPX_MANAGER $ENV{Home120Proof}/Instruments/$instrument_name/lpx_manager.cfg   57 5 17 1 1 4 6 8 11 ");

# Wait until lpx_manager is running
&One20Proof::wait_for_midi("120-Proof-MIDI-In-LPX") or
    die "120-Proof-MIDI-In-LPX not found";
&One20Proof::wait_for_midi("120-Proof-MIDI-Out-LPX") or
    die "120-Proof-MIDI-Out-LPX not found";
&One20Proof::wait_for_midi("120-Proof-MIDI-Out-PD") or
    die "120-Proof-MIDI-Out-PD not found";

# sleep 5

&One20Proof::run_daemon("$ENV{'Home120Proof'}/bin/InitialiseMidi $ENV{'Home120Proof'}/Instruments/$instrument_name/midi.cfg ");
