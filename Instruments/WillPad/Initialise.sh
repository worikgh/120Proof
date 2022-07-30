#!/bin/perl -w
use strict;
use POSIX "setsid";

sub run( $ ) {
    my $cmd = shift or die "Must pass command";
    
    open(STDIN,  "< /dev/null") or die "can't read /dev/null: $!";
    open(STDOUT, "> /dev/null") or die "can't write to /dev/null: $!";
    defined(my $pid = fork())   or die "can't fork: $!";
    return($pid) if $pid;               # non-zero now means I am the parent

    (setsid() != -1)            or die "Can't start a new session: $!";
    open(STDERR, ">&STDOUT")    or die "can't dup stdout: $!";
    `$cmd`;
}




my $HOME='/home/patch/120Proof';
-d $HOME or die "$!: $HOME";

my $LOGFILE="$HOME/Instruments/WillPad/run.log";
open(my $log, ">>$LOGFILE") or die "$!: $LOGFILE";
my $TIME=scalar(localtime());
print $log "echo ----------------------------------------\nStart: $TIME\nStart WillPad\n";

# Must have jack
`jack_wait -w`;

## Programmes to use
my $PD='/usr/local/bin/pd';
-x $PD or die "$!: $PD";
my $LPX_SCALE="$HOME/lpx_scale";
-x $LPX_SCALE or die "$!: $LPX_SCALE";

## Programme to kill
my $LPX_MANAGER="$HOME/lpx_manager";
-x $LPX_MANAGER or die "$!: $LPX_MANAGER";
`pgrep -f $LPX_MANAGER && pkill  -f $LPX_MANAGER `;

## Kill these if they exist
`pgrep -f $PD && pkill -f $PD`;
`pgrep -f $LPX_SCALE && pkill -f $LPX_SCALE`;

while(`pgrep -f $LPX_SCALE`){
    print " Wait for $LPX_SCALE to quit\n";
}

while (`pgrep -f $PD`){
    print " Wait for $PD to quit\n";
}

print $log "WillPad: LPX sent to PD\n";

&run("$PD  -jack -path $HOME/pd_patches/ -send \"; pd dsp 1\" -stdpath  -nogui  $HOME/pd_patches/instruments/HarpPoly.pd");


while(! `jack_lsp |grep pure_data`){
    print "Waiting for jack pure_data\n";
    sleep 1;
}


print $log "Running lpx_scale\n";
&run("$LPX_SCALE $HOME/Instruments/WillPad/lpx_scale.cfg 60 1 3 5 6 8 10 12 ");

print $log "Will: Set up MIDI connections\n";
`/home/patch/120Proof/InitialiseMidi /home/patch/120Proof/Instruments/WillPad/midi.cfg 2>&1 >> $LOGFILE`;

print $log " Will set up\n";
