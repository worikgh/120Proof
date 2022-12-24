#!/usr/bin/perl -w
use strict;

use lib(".");
use One20Proof;

print "test  120Proof Perl module: One20Proof.pm\n";

my $to_test = shift or die "Pass a test";
my $cmd = "test_$to_test";
eval {
    no strict 'refs';
    &$cmd;
};
    
sub test_all {
    ## Use reflection
    my @subs = grep{$_ !~ /^test_all$/} map{s/^sub (.+\S)\s*\{\s*$/$1/; $_} `grep "^sub " test.pl`;
    foreach my $test (@subs){
	
	eval {
	    no strict 'refs';
	    &$test;
	};
	print "Passed $test\n";
    }
    print "Passed all tests\n";
}

sub test_kill_port {
    ## Start some a programme to kill
    my $PORT = 9000;          # pick something not in use
    my $pid = fork();
    if(!$pid){
        use IO::Socket;
        use Net::hostent;      # for OOish version of gethostbyaddr


        my $server = IO::Socket::INET->new( Proto     => "tcp",
                                         LocalPort => $PORT,
                                         Listen    => SOMAXCONN,
                                         Reuse     => 1);

        die "$!: Can not setup server" unless $server;

        while (my $client = $server->accept()) {
	    close $client;
        }
    }
    sleep .2;
    One20Proof::kill_port($PORT);
}

sub test_blank_lpx{
    One20Proof::blank_lpx;
}


sub test_get_lpx_blank_screen {
    my $result = &One20Proof::get_lpx_blank_screen();
    -x $result or die "$!: '$result'";
}

sub test_get_lpx_colour {
    my $result = &One20Proof::get_lpx_colour();
    -x $result or die "$!: '$result'";
}

sub test_get_lpx_control {
    my $result = &One20Proof::get_lpx_control();
    -x $result or die "$!: '$result'";
}

sub test_get_lpx_manager {
    my $result = &One20Proof::get_lpx_manager();
    -x $result or die "$!: '$result'";
}

sub test_get_lpx_mode {
    my $result = &One20Proof::get_lpx_mode();
    -x $result or die "$!: '$result'";
}

sub test_get_lpx_scale {
    my $result = &One20Proof::get_lpx_scale();
    -x $result or die "$!: '$result'";
}


sub test_get_mod_host {
    my $result = &One20Proof::get_mod_host();
    -x $result or die "$!: '$result'";
}

sub test_get_pd {
    my $result = &One20Proof::get_pd();
    -x $result or die "$!: '$result'";
}

sub test_get_yoshimi {
    my $result = &One20Proof::get_yoshimi();
    -x $result or die "$!: '$result'";
}



	
