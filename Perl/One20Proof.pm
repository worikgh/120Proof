package One20Proof;
use IPC::Open3;
#use POSIX "setsid";

## Kill any copies of the passed programme owned by this user
sub pkill( $ ){
    my $prog_name = shift or die;
    if(`pgrep $prog_name -u $ENV{USER} `){
	`pkill $prog_name  -u $ENV{USER} `;
	$? and die "$?: Failed to kill $prog_name\n";
    }
}

sub run_daemon( $ ) {
    my $cmd = shift or die "Must pass command";

    ## Prepare command
    my @cmd = split(/\s+/, $cmd);
    -x $cmd[0] or die "Must pass an executable.  '$cmd[0]' is nopt";
    
    defined(my $pid = fork())   or die "can't fork: $!";
    return($pid) if $pid;               # non-zero now means I am the parent
    # (setsid() != -1)            or die "Can't start a new session: $!";


    
    ## Create logs for stderr and stdout

    # Get the name f the command by separating it from the path
    my $command_file = $cmd[0];
    $command_file =~ s/^.+\/([^\/]+)$/$1/;

    # Turn on autoflush
    
    my $stderr_fn = $ENV{'Home120Proof'}."/output/$command_file.err";
    my $stdout_fn = $ENV{'Home120Proof'}."/output/$command_file.out";
    open(my $stderr, ">>", $stderr_fn) or
	die "$!: Cannot open $stderr_fn for append";
    open(my $stdout, ">>", $stdout_fn) or
	die "$!: Cannot open $stdout_fn for append";

    open3(undef, '>&' . fileno($stdout),  '>&' . fileno($stderr), $cmd);
    $|++;
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
