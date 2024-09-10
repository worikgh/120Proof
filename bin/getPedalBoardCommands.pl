#!/usr/bin/perl -w
use strict;
use lib("$ENV{'Home120Proof'}/Perl");
use One20Proof;
my @commands = ();

## Read the "pedalboard" definitions created by `mod-ui`
## Create a set of files in the pedal directory (One20Proof::get_pedal_dir):
## `Initialise`: A file with commands for `mod-host` and `jackd` to
## set up the simulators needed for all the pedalboards
## One file for each pedalboard that contains the instructions for
## jackd to make that pedalboard active

## Delete any existing pedal definitions
my $pedal_dir = &One20Proof::get_pedal_dir();
opendir(my $dir, $pedal_dir) or die "$!: $pedal_dir";
foreach my $fn (readdir($dir)){
    $fn =~ /^\./ and next;
    $fn =~ /^[A-Z]$/ and next; # Links for pedal driver
    my $path_to_delete = $pedal_dir .'/'. $fn;
    unlink($path_to_delete) or die "$!: $path_to_delete";
}

## Get the pedal board definitions
my @fn = map{chomp;$_}
grep {$_ !~ /manifest.ttl$/}
grep{/\.ttl$/}
`find $One20Proof::MODEP_PEDALS -type f`;

## Each effect is uniquely identified bu `$index`  
my $index = 1;

my @add = ();
my @param = ();
my @jack_init = ();
my %jack_activation = ();

foreach my $fn (@fn){

    my %ex = &One20Proof::process_lv2_turtle($fn, $index);
    $index = $ex{index};

    my $board_name = $ex{pedal_board_name};
    # print "> $board_name\n";

    my $effects = $ex{effects};
    my @lv2_names = sort keys %$effects;
    my %number_name = %{$ex{number_name}};

    my  @j_internal_pipes = @{$ex{jack_internal_pipes}};
    my  @j_activation_pipes = @{$ex{jack_activation_pipes}};

    foreach my $name (@lv2_names){
	# print "\t$name\n";
	my $h = $effects->{$name};
	my @k = sort keys %$h;
	my $add = $h->{add} or die "No `add` for $name";
	# print "\t\t$add\n";
	# print "\t\t".join("", map{"\t\t$_\n"} @param)."\n";
	push(@add, $add);
	push(@param, @{$h->{param}});
    }
    $jack_activation{$board_name} = $ex{jack_activation_pipes};
    push(@jack_init, @{$ex{jack_internal_pipes}});
    
    # foreach my $k (sort keys %number_name){
    # 	my $v = $number_name{$k};
    # 	print "$k => $v\n";
    # }
}

## Output to pedal files.
## Output an initialisation file `Initialse` and a filke for each pedal board

my $pedal_init_fn = "$pedal_dir/Initialise";
open(my $initfh, ">$pedal_init_fn") or die "$!";

## mod-host commands prefixed with "mh"
print $initfh map{"mh $_\n"} @add;
print $initfh map{"mh $_\n"} @param;

## Jack pipes prefixed with "jack"
print $initfh map{"jack $_\n"} @jack_init;
close $initfh or die "$!";
warn "Written $pedal_init_fn\n";
## The activation data.  Pedals use this
foreach my $name (sort keys %jack_activation){
    open(my $actfh, ">$pedal_dir/$name") or die "$!";
    print $actfh map {"$_\n"}
    map{
	## Repair a special case
	/^(capture_\d+):(playback_\d+)/ and $_ = "system:$1 system:$2";
	$_
    }
    @{$jack_activation{$name}};
}
