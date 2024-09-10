#!/usr/bin/perl -w
use strict;

use lib("$ENV{'Home120Proof'}/Perl");
use One20Proof qw | $MODHOST_PORT |;

# Get executables
my $meter = `which x42-meter`;
defined $meter or die $!;
chomp $meter;
-x $meter or die $!;

my $tuner = `which gxtuner`;
defined $tuner or die $!;
chomp $tuner;
-x $tuner or die $!;

if(!`pgrep -f $meter`){
    open(my $fh, '|-', "$meter 8 >/dev/null 2>&1 &") or die $!;
}
if(!`pgrep -f $tuner`){
    open(my $fh, '|-', "$tuner >/dev/null 2>&1 &") or die $!;
}

my @jackc = ();

foreach (&One20Proof::all_jack_connections()){
    /\"([^\"]+)\" \"([^\"]+)\"/ or die $_;
    push @jackc, [$1, $2];
    my $one = $1;
    my $two = $2;
    if($2 =~ /system:playback_\d+/){
	print "$one $two\n";
    }
}

## Find pipe to output
my @outputc =  grep{$_ !~ /^mod-monitor/}map{$_->[0]}grep{$_->[1] =~ /system:playback/} @jackc;
#grep{$_->[1] =~ /system:playback_\d+/ } @jackc;
my $out_to_m = undef;
foreach (@outputc){
    print "> $_ \n";
    $out_to_m = $_;
    last;
}

my $m_cmd1 = "jack_connect '$out_to_m' 'Nordic Meter (Stereo):inR'";
my $m_cmd2 = "jack_connect system:capture_1 'Nordic Meter (Stereo):inL'";
my $t_cmd =  "jack_connect system:capture_1 gxtuner:in_0";
print `$m_cmd1`;
print `$m_cmd2`;
print `$t_cmd`;
