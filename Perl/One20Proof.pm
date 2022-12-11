package One20Proof;
use IPC::Open3;
#use POSIX "setsid";

BEGIN {
    require Exporter;
    our @ISA = qw(Exporter);
    our @EXPORT_OK = qw | $MODEP_PEDALS $PEDAL_DIR $MODHOST_PORT |;
}

## Constants
## Where modep puts its pedal definitions
our $MODEP_PEDALS = "/var/modep/pedalboards";

## Where the files for the foot pedal are
our $PEDAL_DIR = "$ENV{'Home120Proof'}/pedal/PEDALS";

## The port 120Proof's mod-host smulator runs on
our $MODHOST_PORT = 9116;

## Kill any copies of the passed programme owned by this user
sub pkill( $ ){
    my $prog_name = shift or die;
    if(`pgrep $prog_name -u $ENV{USER} `){
	`pkill $prog_name  -u $ENV{USER} `;
	$? and die "$?: Failed to kill $prog_name\n";
    }
}

## Parse output of aconnect -l to make a list of all MIDI connections  that
## 120Proof can use.  There is no accessible documentation for the format
## of this output.

## Output is Hash keyed by deviceID
sub all_midi_devices {
    my %result = ();
    my @aconnect_l = `aconnect -l`;

    my $card = undef;
    my $device = undef;
    my $device_name = undef;
    my $port = undef;
    my $port_name = undef;
    ## Lines like "Connected To: 32:1" as [$device, $port, 32, 1]

    while(my $ac = shift(@aconnect_l)){
	chomp $ac;
	# client 132: 'yoshimi-UltimatePartyKeys' [type=user,pid=17053]

	if($ac =~ /^client (\d+):\s\'(.+)\'\s\[type=\S+,(.+)\]$/){
	    $card = $3;
	    $device = $1;
	    $device_name = $2;
	    $port = undef;
	    $port_name = undef;
	    next;
	}
	if($ac =~ /^client/){
	    $card = undef;
	    $device = undef;
	    $device_name = undef;
	    $port = undef;
	    $port_name = undef;
	    next;
	}
	defined($device_name) or next;
	
	# 0 'Launchpad X MIDI 1'
	if($ac =~ /^\s+(\d+)\s\'(.+)\'$/){
	    defined($card) or die $ac;
	    defined($device) or die $ac;
	    $device_name or die $ac;
	    $port = $1;
	    $port_name = $2;
	    $result{"$device:$port"} = "$device_name/$port_name $card";
	}
    }

    return %result;
}

## Output is an array of all connections:
## [<from device>, <from port>, <to device>, <to port>,
## <from device name>, <from port name> from type, PID or Card id]
## E.g: [32,0,130,1,'Launchpad X','Launchpad X MIDI 1','card',4]

sub list_all_midi_connections {

    my @aconnect_l = `aconnect -l`;


    my $card = undef;
    my $device = undef;
    my $device_name = undef;
    my $pid = undef;
    my $port = undef;
    my $port_name = undef;
    ## Lines like "Connected To: 32:1" as [$device, $port, 32, 1]
    my @connections = ();
    while(my $ac = shift(@aconnect_l)){
	chomp $ac;
	# client 132: 'yoshimi-UltimatePartyKeys' [type=user,pid=17053]
	if($ac =~ /^client (\d+):\s\'(.+)\'\s\[type=\S+,pid=(\d+)\]$/){
	    $card = undef;
	    $device = $1;
	    $device_name = $2;
	    $pid = $3;
	    $port = undef;
	    $port_name = undef;
	    next;
	}
	
	if($ac =~ /^client (\d+):\s\'(.+)\'\s\[type=\S+,card=(\d+)\]$/){
	    $card = $3;
	    $device = $1;
	    $device_name = $2;
	    $pid = undef;
	    $port = undef;
	    $port_name = undef;
	    next;
	}
	if($ac =~ /^client/){
	    $card = undef;
	    $device = undef;
	    $device_name = undef;
	    $pid = undef;
	    $port = undef;
	    $port_name = undef;
	    next;
	}
	defined($device_name) or next;
	
	# 0 'Launchpad X MIDI 1'
	if($ac =~ /^\s+(\d+)\s\'(.+)\'$/){
	    defined($card) or defined($pid) or die $ac;
	    defined($device) or die $ac;
	    $device_name or die $ac;
	    $port = $1;
	    $port_name = $2;
	    next;
	}
	# Connecting To: 128:0[real:0], 130:1
	if($ac =~ /^\s+Connecting To: (.+)/){
	    my @targets = split(/,/, $1);

	    ## Filter outy the perverse target "128:0[real:0]"
	    @targets = map{s/\s//g; $_} grep{_ !~ /128:0]+$/} @targets;
	    foreach my $t (@targets){
		my $real = undef;
		$t  =~ s/\[(.+)\:.+\]// and $real = $1; 
		my @t = split(/\:/, $t);
		scalar(@t) == 2 or die $ac;

		## FIXME What is device 128? (0-127 is MIDI range)
		$t[0] == 128 and next;
		
		push(@connections, [$device, $port, $t[0], $t[1],
				    $device_name, $port_name,
				    defined($card) ? "card" : "programme",
				    defined($card) ? $card : $pid]);
	    }
	}
    }
    return @connections;
}

sub run_daemon( $ ) {
    my $cmd = shift;

    ## Prepare command
    my @cmd = split(/\s+/, $cmd);
    -x $cmd[0] or die "Must pass an executable.  '$cmd[0]' is not";

    
    defined(my $pid = fork())   or die "can't fork: $!";
    return($pid) if $pid;               # non-zero now means I am the parent
    
    ## Create logs for stderr and stdout

    # Get the name of the command by separating it from the path
    my $command_file = $cmd[0];
    $command_file =~ s/^.+\/([^\/]+)$/$1/;

    # Turn on autoflush
    
    my $stderr_fn = $ENV{'Home120Proof'}."/output/$command_file.err";
    my $stdout_fn = $ENV{'Home120Proof'}."/output/$command_file.out";
    open(my $stderr, ">>", $stderr_fn) or
	die "$!: Cannot open $stderr_fn for append";
    open(my $stdout, ">>", $stdout_fn) or
	die "$!: Cannot open $stdout_fn for append";

    select($stdout);
    $|++;
    select($stderr);
    $|++;

    open3(undef, '>&' . fileno($stdout),  '>&' . fileno($stderr), $cmd);


    exec(@cmd);
    exit;
}


## Wait for a MIDI client to come up.  After five seconds give up
sub wait_for_midi {
    my $midi_name = shift or die "Pass a midi name to wait for";
    chomp $midi_name;

    my $time_out = 5; # Five seconds
    my $delay = 0.05;
    my $loops = $time_out / $delay; ## How many loops until time out
    my $counter = 0;
    while(1){
	$counter++;
	if($counter > $loops){
	    return 0;
	}else{
	    foreach my $client (grep{/^client \d+:\s+\'([^\']+)\'/} `aconnect -l`){
		$client =~ /^client \d+:\s+\'([^\']+)\'/ or die ;
		if($midi_name eq $1 ){
		    return 1;
		}
	    }
	}
	select(undef, undef, undef, 0.05);
    }
}    

sub wait_for_jack {
    my $jack_name = shift or die "Pass a midi name to wait for";
    chomp $jack_name;

    my $time_out = 5; # Five seconds
    my $delay = 0.05;
    my $loops = $time_out / $delay; ## How many loops until time out
    my $counter = 0;
    while(1){
	$counter++;
	if($counter > $loops){
	    return 0;
	}else{
	    if(`jack_lsp |grep $jack_name`){
		return 1;
	    }
	}
	select(undef, undef, undef, 0.05);
    }
}    

1;
