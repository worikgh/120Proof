#!/usr/bin/perl -w
use strict;

my $d = 3;
## Initialise pad
`bin/lpx_mode 1`;
`bin/lpx_mode 127`;

## Colour
my $r = 127;
my $g = 0;
my $b = 127;
my $step = int(127/8);
for(my $not_g = 0; $not_g <= 8; $not_g++){
    $g = $not_g * $step;
    for(my $row = 1; $row <= 8; $row++){
        for(my $col = 1; $col <= 8; $col++){
            my $pad = $row * 10 + $col;
            my $_r = $r - $row * $step;
            # my $_g = $g - $col * $step;
            my $_g = $g;
            my $_b = $b - $col * $step;
            # print sprintf("%d\t%d\t%d\t%d\n", $pad, $_r, $_g, $_b);
            `bin/lpx_colour $pad $_r $_g $b`;
        }
    }
}
exit;
my $pad = 54; ## Start here
while(1){

    print `bin/lpx_colour $pad $r $g $b 2>&1 > /dev/null`;
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
