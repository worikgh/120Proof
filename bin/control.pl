#!/usr/bin/perl -w
use strict;
my $control_file = "$ENV{Home120Proof}/control";
open(my $fh, ">$control_file") or die $!;
print $fh "MODEP\n";
close $fh;
my $flag = "MODEP\n";


## This is run from crontab @reboot.
fork() and exit;

## Wait a while for the system to get going
sleep 15;
my $fn = "$ENV{Home120Proof}/output/control.$$.out";
my $time = scalar(localtime());
my $log = sub {
    my $message = shift or die;
    open(my $fh, ">>$fn") or die $!;
    print $fh $message;
    close $fh or die $!;
};
&$log("$0 $time\n");
while(1){
    my $z = `cat $control_file`;
    if ($z =~ /120Proof/){
	if($flag ne $z){
	    $flag = $z;
	    my $mistris = "$ENV{Home120Proof}/bin/button_1.pl";
	    print `sudo --preserve-env $mistris`;
	    &$log("\$flag $flag $mistris\n");
	}
    }elsif($z =~ /MODEP/){
	if($flag ne $z){
	    $flag = $z;
	    my $mistris = "$ENV{Home120Proof}/bin/button_2.pl";
	    print `sudo --preserve-env $mistris`;
	    &$log("\$flag $flag $mistris\n");
	}
    }
    sleep 1;
}
