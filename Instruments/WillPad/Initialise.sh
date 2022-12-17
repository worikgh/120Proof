#!/usr/bin/perl -w
use strict;
use lib("$ENV{'Home120Proof'}/Perl");
use One20Proof;


my $HOME=$ENV{'Home120Proof'};
-d $HOME or die "$!: $HOME";

my $TIME=scalar(localtime());
print "$TIME: Start WillPad\n";

# Must have jack
`jack_wait -w`;

## Programmes to use
my $PD='/usr/local/bin/pd';
-x $PD or die "$!: $PD";
my $LPX_SCALE="$HOME/bin/lpx_scale";
-x $LPX_SCALE or die "$!: $LPX_SCALE";

## Programme to kill
my $YOSHIMI="/usr/local/bin/yoshimi";
&One20Proof::pkill($YOSHIMI);

my $LPX_MANAGER="$HOME/bin/lpx_manager";
&One20Proof::pkill($LPX_MANAGER);

## Kill these if they exist
&One20Proof::pkill($PD);
&One20Proof::pkill($LPX_SCALE);

while(`pgrep -f $LPX_SCALE`){
    die "$LPX_SCALE has not quit";
}

&One20Proof::pkill($PD);
while (`pgrep -f $PD`){
    die "$PD has not quit";
}


&One20Proof::run_daemon("$PD  -jack -path $HOME/pd_patches/ -send \"; pd dsp 1\" -stdpath  -nogui  $HOME/pd_patches/instruments/HarpPoly.pd");
&One20Proof::wait)_for_jack("pure_data");


print "Running lpx_scale\n";
&One20Proof::run_daemon("$LPX_SCALE $HOME/Instruments/WillPad/lpx_scale.cfg 60 1 3 5 6 8 10 12 ");

print " WillPad: Keyboard sent to Electric Piano \n";
&One20Proof::run_daemon("$HOME/bin/InitialiseYos WillPadKeys '$HOME/Instruments/xiz/ElectricPiano.xiz'");

print "Will: Set up MIDI connections\n";
&One20Proof::run_daemon("$HOME/bin/InitialiseMidi $HOME/Instruments/WillPad/midi.cfg");

    
print " Will set up\n";
