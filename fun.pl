#!/usr/bin/perl -w
use strict;

my $d = 3;
## Initialise pad
`./lpx_mode 1`;
`./lpx_mode 127`;

## Colour
my $r = int(rand 128); ## 0-127
my $g = int(rand 128); ## 0-127
my $b = int(rand 128); ## 0-127
my $pad = 54; ## Start here
while(1){

    print `./lpx_colour $pad $r $g $b 2>&1 > /dev/null`;
    print sprintf("$pad\t%03i %03i %03i\n", $r, $g, $b);

    ## Next pad must be valid.  Could get complex or choose one at
    ## random and if not valid then change it
    my $next_pad = sub {
	my $pad = shift;
	## Choose next pad
	my $r = int($pad / 10);
	my $c = $pad % 10;
	my($nr, $nc) = ($r, $c);
	## There are eight possible directions: N = 0, NE = 1, E,...,W, NW = 7
	my $direction = int(rand 8);
	if($direction == 0){
	    $nr = $r + 1;
	}elsif($direction == 1){
	    $nr = $r + 1;
	    $nc = $c + 1;
	}elsif($direction == 2){
	    $nc = $c + 1;
	}elsif($direction == 3){
	    $nr = $r - 1;
	    $nc = $c + 1;
	}elsif($direction == 4){
	    $nr = $r - 1;
	}elsif($direction == 5){
	    $nr = $r - 1;
	    $nc = $c - 1;
	}elsif($direction == 6){
	    $nc = $c - 1;
	}elsif($direction == 7){
	    $nc = $c - 1;
	    $nr = $r + 1;
	}
	return $nr * 10 + $nc;
    };
    my $valid_pad = sub {
	my $pad = shift;
	my $c = $pad % 10;
	$c < 1 or $c > 8 and return 0;
	my $r = $pad / 10;
	$r < 10 or $r > 8 and return 0;
	return 1;
    };

    while(1){
	my $new_pad = &$next_pad($pad);
	if(&$valid_pad($new_pad)){
	    $pad = $new_pad;
	    last;
	}
    }

    ## Now get a neighbouring colour.  Three colours, there are 27
    ## possible ways to move.
    ## Handle this by changing each colour individually
    ## Wrap from 127 -> 0
    ## Each colour has three possible states: up, down, stay still
    my $dr = int(rand 3) - 1; ## -1 => red down, 0 => same 1 => red up
    my $dg = int(rand 3) - 1;
    my $db = int(rand 3) - 1;
    my $wrap = sub {
	my $colour = shift;
	$colour < 0 and return 127;
	$colour > 127 and return 0;
	return $colour;
    };
    $r = &$wrap($r + $dr);
    $g = &$wrap($r + $dg);
    $b = &$wrap($r + $db);
##    select(undef, undef, undef, 0.1);
}
