#!/usr/bin/perl -w
use strict;

my $time = scalar(localtime());
print "Start GhostWalking $time\n";

use lib("$ENV{'Home120Proof'}/Perl");
use One20Proof;

# Must have jack
`jack_wait -w`;
if(!$0){
    die "Failed waiting jack: $0\n";
}


## Kill these if they exist
&One20Proof::pkill('lpx_manager');
&One20Proof::pkill('yoshimi');
&One20Proof::run_daemon("$ENV{'Home120Proof'}/bin/InitialiseYos  GhostWalkingKeys '$ENV{'Home120Proof'}/Instruments/xiz/Hammond Organ.xiz'");
&One20Proof::run_daemon("$ENV{'Home120Proof'}/bin/InitialiseYos GhostWalkingLPX '$ENV{'Home120Proof'}/Instruments/xiz/Wide Bass.xiz'");

while (! `jack_lsp |grep GhostWalkingLPX` ){
    print "Waiting for jack GhostWalkingLPX\n";
    sleep 1;
}

while (! `jack_lsp |grep GhostWalkingKeys` ){
    print "Waiting for jack GhostWalkingKeys\n";
    sleep 1;
}

&One20Proof::run_daemon("$ENV{'Home120Proof'}/bin/lpx_manager $ENV{'Home120Proof'}/Instruments/GhostWalking/lpx_manager.cfg 69 1 4 6 9 11 < /dev/null ");

# TODO: How can it be determined lpx_manager is running?
sleep 1;

&One20Proof::run_daemon("$ENV{'Home120Proof'}/bin/InitialiseMidi $ENV{'Home120Proof'}/Instruments/GhostWalking/midi.cfg ");
# TODO: Get a way of of confirming MDI set up correctly
