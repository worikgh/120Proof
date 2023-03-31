#!/usr/bin/perl -w
use strict;
use lib("$ENV{'Home120Proof'}/Perl");
use One20Proof;

## This is run from crontab @reboot.
## Wait a while for the system to get going
sleep 15;
fork() and exit;

## The type of guitar effects in use.  If using MODEP 120Proof (which
## is more than guitar effects) cannot run
my $effects = "MODEP";

## Initialise the control file to be running modep.
my $control_file = "$ENV{Home120Proof}/control";
open(my $fh, ">$control_file") or die $!;
print $fh "$effects\n";
close $fh;

## Programme to find temperature
my $temp_sense = "/home/patch/temperature/target/release/temperature";
-x $temp_sense or die;

## Initialise the pedal file 
my $pedal_file = "$ENV{Home120Proof}/pedal/PEDALS/.PEDAL";
## The current pedal.  
my $pedal = `cat $pedal_file`;


## Set up logging where `monitor` can find it
my $fn = "$ENV{Home120Proof}/output/control.$$.out";
my $time = scalar(localtime());
my $log = sub {
    my $message = shift or die;
    open(my $fh, ">>$fn") or die $!;
    print $fh "control.pl: $message";
    close $fh or die $!;
};
&$log("$0 $time\n");

## The programme to set the colour of the pads.  This is used to
## indicate which pedal is in action
my $set_colour = "$ENV{Home120Proof}/bin/lpx_colour";

## Loop forever....
while(1){
    my $z = `cat $control_file`;
    chomp($z);
    my $y = `cat $pedal_file`;
    chomp($y);

    if($y ne $pedal){
	$pedal = $y;
    }

    my $temp = `$temp_sense`;
    chomp $temp;
    if($temp > 75.0){
	&$log("Too hot!! $temp \n");
    }
    if ($z =~ /120Proof/){
	if($effects ne $z){
	    $effects = $z;
	    my $mistris = "$ENV{Home120Proof}/bin/button_1.pl";
	    &One20Proof::run_daemon($mistris);
	    &$log("\$effects $effects $mistris\n");
	}
    }elsif($z =~ /MODEP/){
	if($effects ne $z){
	    $effects = $z;
	    my $mistris = "$ENV{Home120Proof}/bin/button_2.pl";
	    &One20Proof::run_daemon($mistris);
	    &$log("\$effects $effects $mistris\n");
	}
    }
    
    if($effects eq '120Proof'){

	    
	my ($r1, $r2, $r3, $r4) = (int(rand() * 128), int(rand() * 128), int(rand() * 128), int(rand() * 128));
	my ($g1, $g2, $g3, $g4) = (int(rand() * 128), int(rand() * 128), int(rand() * 128), int(rand() * 128));
	my ($b1, $b2, $b3, $b4) = (int(rand() * 128), int(rand() * 128), int(rand() * 128), int(rand() * 128));
	if($pedal eq 'A'){
	}elsif($pedal eq 'A'){
	    ($r1, $g1, $b1) = (127, 0, 0);
	}elsif($pedal eq 'B'){
	    ($r2, $g2, $b2) = (127, 0, 0);
	}elsif($pedal eq 'C'){
	    ($r3, $g3, $b3) = (127, 0, 0);
	}elsif($pedal eq 'D'){
	}
	`$set_colour 91 $r1 $g1 $b1`;
	`$set_colour 92 $r2 $g2 $b2`;
	`$set_colour 93 $r3 $g3 $b3`;
	`$set_colour 94 $r4 $g4 $b4`;
	sleep .5;
    }
}
