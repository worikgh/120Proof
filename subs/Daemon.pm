#!/usr/bin/perl -w
use strict;

use POSIX "setsid";

sub run( $ ) {
    my $cmd = shift or die "Must pass command";
    -e $cmd or die "Must pass executable command";

    
    open(STDIN,  "< /dev/null") or die "can't read /dev/null: $!";
    open(STDOUT, "> /dev/null") or die "can't write to /dev/null: $!";
    defined(my $pid = fork())   or die "can't fork: $!";
    return($pid) if $pid;               # non-zero now means I am the parent

    (setsid() != -1)            or die "Can't start a new session: $!";
    open(STDERR, ">&STDOUT")    or die "can't dup stdout: $!";
    `$cmd`;
}

1;
