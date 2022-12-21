#!/usr/bin/perl -w
use strict;
use lib("$ENV{'Home120Proof'}/Perl");
use One20Proof;


my $HOME=$ENV{'Home120Proof'};
-d $HOME or die "$!: $HOME";

my $TIME=scalar(localtime());
print "$TIME: Start WillPad\n";
warn scalar(localtime()) . " MARK ";

# Must have jack
`jack_wait -w`;

## The instruments for LPX and Keys
my $LPX_INSTR = "$HOME/pd_patches/instruments/HarpPoly.pd";
my $KEYS_INSTR = "$HOME/Instruments/xiz/ElectricPiano.xiz";

## Programmes to use
my $PD='/usr/local/bin/pd';
-x $PD or die "$!: $PD";
my $LPX_SCALE="$HOME/bin/lpx_scale";
-x $LPX_SCALE or die "$!: $LPX_SCALE";

## Programmes to kill
my $YOSHIMI="/usr/local/bin/yoshimi";
my $LPX_MANAGER="$HOME/bin/lpx_manager";
&One20Proof::pkill($YOSHIMI);
&One20Proof::pkill($LPX_MANAGER);

## Kill these if they exist.  They will be restarted
&One20Proof::pkill($PD);
&One20Proof::pkill($LPX_SCALE);

if(`pgrep -f $LPX_SCALE`){
    die "$LPX_SCALE has not quit";
}

if (`pgrep -f $PD`){
    die "$PD has not quit";
}


&One20Proof::run_daemon("$PD  -jack -path $HOME/pd_patches/ -send \"; pd dsp 1\" -stdpath  -nogui  $LPX_INSTR");
&One20Proof::run_daemon("$HOME/bin/InitialiseYos WillPadKeys $KEYS_INSTR");
&One20Proof::run_daemon("$LPX_SCALE $HOME/Instruments/WillPad/lpx_scale.cfg 60 1 3 5 6 8 10 12 ");
warn scalar(localtime()) . " MARK ";

##&One20Proof::run_daemon("$HOME/bin/InitialiseMidi $HOME/Instruments/WillPad/midi.cfg", 1);
print `$HOME/bin/InitialiseMidi $HOME/Instruments/WillPad/midi.cfg`;

    
print " Will set up\n";
