#!/usr/bin/perl -w
use strict;

use lib("$ENV{'Home120Proof'}/Perl");
use One20Proof;


my $to_test = shift or die "Pass a test";
my $cmd = "test_$to_test";
print "test  120Proof Perl module: One20Proof.pm\n$to_test\n";
{
    no strict 'refs';
    &$cmd;
};
    
sub test_all {
    ## Use reflection
    my @subs = grep{$_ !~ /^test_all$/} map{s/^sub (.+\S)\s*\{\s*$/$1/; $_} `grep "^sub " test.pl`;
    foreach my $test (@subs){
	
	 {
	    no strict 'refs';
	    &$test;
	};
	print "Passed $test\n";
    }
    print "Passed all tests\n";
}

sub test_get_pedal_names {
    print join("\n", &One20Proof::get_pedal_names())."\n";
}
sub test_remove_all_mod_host_simulators{
    print  &One20Proof::remove_all_mod_host_simulators."\n";
}

sub test_list_mod_host_simulators{
    print join (", ", &One20Proof::list_mod_host_simulators)."\n";
}
		

sub test_all_jack_connections {
    my @jack_connections = &One20Proof::all_jack_connections;
    print join("\n", @jack_connections)."\n";    
}
sub test_process_lv2_turtle {

    my $fn = '/var/modep/pedalboards/Test.pedalboard/Test.ttl';
    -r $fn or die "$fn";
    my %result = One20Proof::process_lv2_turtle($fn, 1);
    &report_hash(\%result, "");
}


sub test_initialise_pedals {
    One20Proof::initialise_pedals(qw| delay DISTORTION Wookie |);
    1;
}

sub test_read_turtle {

    # Write a test turtle document
    my $test_dir = "/tmp/$$"."_test_data";
    -d $test_dir or mkdir $test_dir or die $!;
    my $fn1 = "$test_dir/read_turtle_test1.ttl";
    open(my $fh1, ">$fn1") or die "$!: $fn1";
    my $str = <<'END';
# this is a complete turtle document
@prefix foo: <http://example.org/ns#> .
@prefix : <http://other.example.org/ns#> .
foo:bar foo: : .
:bar : foo:bar .
END
    print $fh1 $str;
    close $fh1 or die $!;
    my $fn2 = '/var/modep/pedalboards/Gx_Studio_Preamp.pedalboard/Gx_Studio_Preamp.ttl';
    &One20Proof::read_turtle($fn2);
    &One20Proof::read_turtle($fn1);
    &One20Proof::read_turtle("/var/modep/pedalboards/AK20.pedalboard/AK20.ttl");
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
sub test_get_modep_simulation_commands {
    my %result = &One20Proof::get_modep_simulation_commands([]);
    print join("\n", @{$result{add}})."\n";
    print join("\n", @{$result{param}})."\n";
    print join("\n", @{$result{jack_initial}})."\n";
    foreach my $key(sort keys %{$result{jack_activation}}){
	print "$key:\n\t" . join("\n\t",  @{$result{jack_activation}->{$key}})."\n";
    }
    print join("\n", map{"$_\t".$result{number_name}->{$_}} sort {$a <=> $b} keys %{$result{number_name}})."\n";
}

sub test_list_pedals {
    my @array = One20Proof::list_pedals;
    @array or die "No pedals";
    my $pedals = One20Proof::list_pedals;
    $pedals or die "No pedals";
    print ref $pedals;
    print $pedals;
}

sub test_blank_lpx{
    One20Proof::blank_lpx;
}

sub test_get_bin{
    -d     One20Proof::get_bin  or die $!;
}

sub test_get_pedal_dir{
    -d     One20Proof::get_pedal_dir  or die $!;
}

sub test_get_lpx_blank_screen {
    my $result = &One20Proof::get_lpx_blank_screen();
    -x $result or die "$!: '$result'";
}

sub test_get_lpx_colour {
    my $result = &One20Proof::get_lpx_colour();
    -x $result or die "$!: '$result'";
}

sub test_get_lpx_controll {
    my $result = &One20Proof::get_lpx_controll();
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



