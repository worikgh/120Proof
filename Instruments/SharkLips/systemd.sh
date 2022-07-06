#!/usr/bin/perl -w
use strict;

my $pid = fork();
if($pid){
    ## Parent
    exit;
}
## Child
$ENV{"JACK_PROMISCUOUS_SERVER"} = 'jack';
foreach my $env (sort keys %ENV){
    print "$env\t$ENV{$env}\n";
}

print `/home/patch/120Proof/Instruments/SharkLips/Initialise.sh`;
