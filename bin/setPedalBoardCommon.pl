#!/usr/bin/perl -w
use strict;
use IO::Socket::INET;
use lib("$ENV{'Home120Proof'}/Perl");
use One20Proof qw | $PEDAL_DIR $MODHOST_PORT |;

## Set up the simulators and jack pipes between them.  Read data
## written by `getPedalBoardCommands.pl`

my $pedal_dir = &One20Proof::get_pedal_dir();
my $initialise_fn = "$pedal_dir/Initialise";
#-r $initialise_fn or die "$!: '$initialise_fn'";
open(my $fh, $initialise_fn) or die "$!: $initialise_fn";
my @config = map{chomp ; $_} <$fh>;
my @add = grep{s/^mh //} map{chomp ; $_} grep {/^mh add/} @config; 
my @param = grep{s/^mh //} map{chomp ; $_} grep {/^mh param_set /} @config; 
my @jack_initial = grep{s/^jack //} map{chomp ; $_} grep {/^jack /} @config; 
close $fh or die $!;

## Set up the effects, and the arameters
my @mh_commands = ();
push(@mh_commands, @add);
push(@mh_commands, @param);
&mod_host(\@mh_commands);
foreach my $jcmd ( @jack_initial ) {
    &One20Proof::handle_jack( "connect $jcmd" );
}

# foreach my $jcmd ( @jack_initial ) {
#     &One20Proof::handle_jack( "connect $jcmd" );
# }


## `handle_mh_cmd` and `mod_host` set up the LV2 simulators.
## `mod_host` is passed an array of commands to send to `mod-host`
sub handle_mh_cmd( $$ ) {
    my ($sock, $cmd) = @_;
    warn "handle_mh_cmd(SOCK, $cmd)\n";
    print $sock "$cmd\n";

    my $result = '';
    my $r = &One20Proof::fhbits($sock);
    my $res = '';
    my ($nfound, $timeleft) =
	select(my $rout = $r, my $wout = undef, my $eout = undef,
	       0.5);
    if($nfound){
	my $os = 0;
	while(my $c = read($sock, $res, 1)){
	    if($c != 1 or
	       ord($res) == 0){
		last;
	    }
	    $result .=  $res;
	}
    }
    # warn "handle_mh_cmd: \$result $result\n";
    if($result =~ /resp ([\-0-9]+)/){
	# If status is a negative number an error has
	# occurred. The table below shows the number of each
	# error.
	
	# status 	error
	# -1 	ERR_INSTANCE_INVALID
	# -2 	ERR_INSTANCE_ALREADY_EXISTS
	# -3 	ERR_INSTANCE_NON_EXISTS
	# -4 	ERR_INSTANCE_UNLICENSED
	# -101 	ERR_LV2_INVALID_URI
	# -102 	ERR_LV2_INSTANTIATION
	# -103 	ERR_LV2_INVALID_PARAM_SYMBOL
	# -104 	ERR_LV2_INVALID_PRESET_URI
	# -105 	ERR_LV2_CANT_LOAD_STATE
	# -201 	ERR_JACK_CLIENT_CREATION
	# -202 	ERR_JACK_CLIENT_ACTIVATION
	# -203 	ERR_JACK_CLIENT_DEACTIVATION
	# -204 	ERR_JACK_PORT_REGISTER
	# -205 	ERR_JACK_PORT_CONNECTION
	# -206 	ERR_JACK_PORT_DISCONNECTION
	# -301 	ERR_ASSIGNMENT_ALREADY_EXISTS
	# -302 	ERR_ASSIGNMENT_INVALID_OP
	# -303 	ERR_ASSIGNMENT_LIST_FULL
	# -304 	ERR_ASSIGNMENT_FAILED
	# -401 	ERR_CONTROL_CHAIN_UNAVAILABLE
	# -402 	ERR_LINK_UNAVAILABLE
	# -901 	ERR_MEMORY_ALLOCATION
	# -902 	ERR_INVALID_OPERATION

	#     A status zero or positive means that the command was
	#     executed successfully. In case of the add command,
	#     the status returned is the instance number. The
	#     value field currently only exists for the param_get
	#     command.
	if($1 < 0 and $1 != -2){
	    print  STDERR ">> FAIL $cmd >>  $result\n";
	}else{
	    # print  STDERR ">> SUCCESS $cmd >>  $result\n";
	    return 1;
	}
    }else{
	print STDERR ">> Unexpected result: $result ";
    }
    return 0;
}    
sub mod_host( $ ){
    my $cmds = shift or die;
    my @cmds = @$cmds;

    my $remote = "localhost";

    my $mod_host_port_p = $One20Proof::MODHOST_PORT;
    my $sock = new IO::Socket::INET( PeerAddr => 'localhost',
				     PeerPort => $mod_host_port_p, 
				     Proto => 'tcp') or
	die "$!: Failed to connect to mod-host localhost:$mod_host_port_p ".
	"lsof -i :$mod_host_port_p: ".`lsof -i :$mod_host_port_p` . ' '; 

    ## Debugging why some effects randomly fail to be added
    my $failed = 0;
    
    foreach my $cmd (@cmds){
	# warn "Process: \$cmd($cmd) \n";
	# print STDERR  "mod-host: $cmd\n";
	if(!$failed){
	    &handle_mh_cmd($sock, $cmd);
	}
	## If command was an `add` check the effects got added
	if($cmd =~ /^add.+\s(\d+)/){
	    # print STDERR "$cmd\n";
	    # warn "Before jack_lsp\n";
	    my $jack = grep{/effect_$1/} `jack_lsp`;
	    # warn "after jack_lsp\n";
	    if(!$jack){
		print STDERR "$cmd: effect_$1 failed\n";
		$failed = 1;
	    }else{
		$failed = 0;
		# print STDERR "Got effect_$1\n";
	    }
	}
    }
}
