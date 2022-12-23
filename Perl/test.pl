#!/usr/bin/perl -w
use strict;

use lib(".");
use One20Proof;

print "test  120Proof Perl module: One20Proof.pm\n";

my $to_test = shift or die "Pass a test";

if($to_test eq "kill_port"){
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
}elsif($to_test eq "blank_lpx"){
    One20Proof::blank_lpx;
}
