#!/usr/bin/perl -w
use strict;

my $d = 3;

## For each pad
my $r = 0;
my $g = 0;
my $b = 0;
while(1){
    my $p = int(rand(81) + 0.5) + 11; 
    $r += $d;
    if($r > 127){
	$r = 0;
	$g += $d;
	if ($g > 127) {
	    $g = 0;
	    $b += $d;
	    if($b > 127){
		$b = 0;
	    }
	}
    }
    print `./lpx_colour $p $r $g $b 2>&1 > /dev/null`;
    print sprintf("$p\t%03i %03i %03i\n", $r, $g, $b);
    select(undef, undef, undef, 0.1);
}
