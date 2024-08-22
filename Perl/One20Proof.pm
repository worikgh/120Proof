package One20Proof;
use IO::Socket::INET;
use IPC::Open3;
use File::Find;
use POSIX; # "setsid";
use strict;
BEGIN {
    require Exporter;
    our @ISA = qw(Exporter);
    our @EXPORT_OK = qw | $MODEP_PEDALS $PEDAL_DIR $MODHOST_PORT |;
}

## Constants
## Where modep puts its pedal definitions
our $MODEP_PEDALS = "/home/worik/.pedalboards";

## Where the files for the foot pedal are
our $PEDAL_DIR = "$ENV{'Home120Proof'}/Pedal/PEDALS";

## The port 120Proof's mod-host smulator runs on
our $MODHOST_PORT = 5555;

## Turn off all the LEDs on the LPX
sub blank_lpx {
    my $lpx_blank_screen = &One20Proof::get_lpx_blank_screen();
    -x $lpx_blank_screen or die "$!: $lpx_blank_screen";
    `$lpx_blank_screen`;
}

## From select(2) section of perlfunc
sub fhbits {
    my @fhlist = @_;
    my $bits = "";
    for my $fh (@fhlist) {
        vec($bits, fileno($fh), 1) = 1;
    }
    return $bits;
}

## Remove all mod-host simulators
sub remove_all_mod_host_simulators {
    my $port = shift;
    defined($port) or $port = $MODHOST_PORT;
    my @simulators = &One20Proof::list_mod_host_simulators;
    my $result = "";
    # my ( $name,   $passwd,   $uid,       $gid,     $quota,
    # 	 $comment,  $gcos,     $dir,       $shell,   $expire ) = getpwuid($$);
    # warn "Here: $$ $name ";
    my $sock = new IO::Socket::INET( PeerAddr => 'localhost',
                                     PeerPort => $port,
                                     Proto => 'tcp') or
        die "$!";

    foreach my $sim (@simulators){
        my $cmd = "remove $sim";
        print $sock "$cmd\n";
        my $cmd_result = '';
        my $r = &One20Proof::fhbits($sock);
        my $res = '';
        my ($nfound, $timeleft) =
            select(my $rout = $r, my $wout = undef, my $eout = undef,
                   0.5);
        # warn "handle_mh_cmd: \$nfound $nfound\n";
        if($nfound){
            my $os = 0;
            while(my $c = read($sock, $res, 1)){
                if($c != 1 or
                   ord($res) == 0){
                    last;
                }
                $cmd_result .=  $res;
            }
        }
        $result .= "$cmd: $cmd_result\n";
    }
    return $result;
}

## For debugging.  Not very useful
sub stack_trace {
    my $frame = 0;
    my @frames = ();
    while(1){
        my @frame = caller($frame++);
        if(!@frame or @frame == 0 or $frame > 100){
            last;
        }
        push(@frames, "$frame[3] \@ $frame[1]:$frame[2]");
    }
    wantarray and return @frames;
    return join("\n", @frames)."\n";
}
sub report_hash {
    my $hashref = shift or die;
    ref $hashref eq 'HASH' or die ;
    my $prefix = shift;
    defined($prefix) or $prefix = "";
    foreach my $key (sort keys %$hashref){
        my $next_prefix = "$prefix\t$key\t";
        $next_prefix =~ s/^\s*//;
        # print("$next_prefix\n");
        my $value = $hashref->{$key};
        if(ref($value) eq ''){
            print("$next_prefix$value\n");
        }elsif(ref($value) eq 'HASH'){
            &report_hash($value, $next_prefix);
        }elsif(ref($value) eq 'ARRAY'){
            &report_array($value, $next_prefix);
        }
    }
}
sub report_array( $$ ) {
    my $arrayref = shift or die;
    ref $arrayref eq 'ARRAY' or die ;
    my $prefix = shift;
    defined($prefix) or $prefix = "";
    foreach my $elem (@$arrayref){
        if(ref($elem) eq ''){
            print("$prefix$elem\n");
        }elsif(ref($elem) eq 'HASH'){
            &report_hash($elem, $prefix);
        }elsif(ref($elem) eq 'ARRAY'){
            &report_array($elem, $prefix);
        }
    }
}

## Kill whatsoever process, owned by us, holds a port
sub kill_port( $ ) {
    my $port = shift or die;
    my @lsof = `lsof -i :$port -F`;
    my @pids = ();
    foreach my $l (@lsof){
        chomp $l;
        $l =~ /^p(\d+)/ or next;
        my $pid = $1;
        push(@pids, $pid);
    }
    
    foreach my $pid (@pids){
        my $cmd = "kill $pid";
        my $output = `$cmd`;
        if($?){
            ## `kill` failed
            die "$output: $!: Could not kill $pid";
        }
    }
}

## Kill any copies of the passed programme owned by this user
sub pkill( $ ){
    my $prog_name = shift or die;
    chomp $prog_name;
    ## The prog_name must be the complete path to the executable
    -x $prog_name or die "The argument to `pkill` ($prog_name) must be the complete path to the executable: " . scalar(One20Proof::stack_trace) . " ";

    my $signal = shift;
    defined($signal) or $signal = SIGTERM;
    my @pgrep = `pgrep -f $prog_name -u $ENV{USER} `;
    if(`pgrep -f $prog_name -u $ENV{USER} `){
        `pkill --signal $signal -f $prog_name  -u $ENV{USER} `;
        if($? && $? != 256){
            ## $? is eight bits.  256 is nine.
            warn join("\n", @pgrep);
            ## Could not kill the programme.  Do some diagnostics
            my $die_msg = "$?: Failed to kill $prog_name: ".scalar(stack_trace());


            #  0 dev      device number of filesystem
            #  1 ino      inode number
            #  2 mode     file mode  (type and permissions)
            #  3 nlink    number of (hard) links to the file
            #  4 uid      numeric user ID of file's owner
            #  5 gid      numeric group ID of file's owner
            #  6 rdev     the device identifier (special files only)
            #  7 size     total size of file, in bytes
            #  8 atime    last access time in seconds since the epoch
            #  9 mtime    last modify time in seconds since the epoch
            # 10 ctime    inode change time in seconds since the epoch (*)
            # 11 blksize  preferred I/O size in bytes for interacting with the
            #             file (may vary from file to file)
            # 12 blocks   actual number of system-specific blocks allocated
            #             on disk (often, but not always, 512 bytes each)

            
            warn "Info: Stating \$prog_name: $prog_name";
            my @stat = stat($prog_name);
            my $owner = getpwuid($stat[4]);
            warn "Info: Stating \$prog_name: $prog_name.  Owner: $owner";
            if($owner){
                $die_msg .= " Owner: $owner ";
            }else{
                $die_msg .= " No stat data for $prog_name";
            }
            die $die_msg;
        }
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
            @targets = map{s/\s//g; $_} grep{$_ !~ /128:0]+$/} @targets;
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

## Run a programme, either as a daemon (this function retutns straight
## away) or wait on its output
sub run_daemon( $;$ ) {
    my $cmd = shift;
    my $wait = shift or 0;
    ## Prepare command
    $cmd =~ /^(\S+)/ or die "Invalid command: '$cmd'";
    my $_x = $1;
    -x $_x or die "Must pass an executable.  '$_x' is not";
    
    defined(my $pid = fork())   or die "can't fork: $!";
    $wait and waitpid($pid, 0);
    if (!$pid){ 
        
        ## Create logs for stderr and stdout

        # Get the name of the command by separating it from the path
        my $command_file = $_x;
        $command_file =~ s/^.+\/([^\/]+)$/$1/;
        
        my $stderr_fn = $ENV{'Home120Proof'}."/output/$command_file.$$.err";
        $stderr_fn =~ /\/\.err$/ and
            die "No file name for err: \$cmd: '$cmd' ".
            join("\n", stack_trace());
        my $stdout_fn = $ENV{'Home120Proof'}."/output/$command_file.$$.out";
        $stdout_fn =~ /\/\.out$/ and
            die "No file name for out: \$cmd: '$cmd' ".
            join("\n", stack_trace());
        open(my $stderr, ">>", $stderr_fn) or
            die "$!: Cannot open $stderr_fn for append";
        open(my $stdout, ">>", $stdout_fn) or
            die "$!: Cannot open $stdout_fn for append";

        # Turn on autoflush
        select($stdout);
        $|++;
        select($stderr);
        $|++;

        open3(undef, '>&' . fileno($stdout),  '>&' . fileno($stderr), $cmd);
        exit;
    }
    return $pid;
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

## Wait for a Jack client to come up.  After five seconds give up
sub wait_for_jack {
    my $jack_name = shift or die "Pass a jack client name to wait for";
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
            if(grep{/$jack_name/} `jack_lsp`){
                return 1;
            }
        }
        select(undef, undef, undef, 0.05);
    }
}

## Check for a connetion between two ports.  
sub test_jack_connection( $$ ) { 
    my ($lhs, $rhs) = @_;
    my @jack_lsp = `jack_lsp -c`;


    my $c_lhs;
    my $c_rhs;

    my $result = 0;
    my $state = "";
    foreach my $line (@jack_lsp){
        chomp $line;
        if($line =~ /^$lhs$/){
            $state = $lhs;
            next;
        }elsif($line =~ /^\S/){
            $state = "";
            next;
        }elsif($line =~ /^\s+$rhs$/){
            if($state){
                return 1;
                exit;
            }
        }
    }
    return 0;
}

sub all_jack_connections {
    my @jack_lsp = `jack_lsp -tc`;
    my @result = ();

    ## Where connection is from
    my $lhs = undef;
    ## What it is connected to
    my @rhs = ();

    foreach my $l(@jack_lsp){
        chomp $l;

        ## Pipes are arranged so for one LHS there are one or more RHS.
        ##
        ## The lines have three types:
        ## 1. Name the LHS. Line starts with no white space
        ## 2. Name a RHS.  Line starts with three spaces
        ## 3. Kind of pipe.  Can be audio or MIDI

        ## LHS: TODO: Is pipe name always non-white space only?
        if($l =~ /^(\S.*)$/){
            $lhs = $1;
            next;
        }

        ## A RHS
        if($l =~ /^   (\S.*)$/){
            push(@rhs, $1);
            next;
        }

        ## Type of pipe.  Finished 
        if($l =~ /^\t(.+)/){
            my $type_dfn = $1;
            if($type_dfn =~ /audio/){
                @rhs and push(@result, map{"\"$lhs\" \"$_\""} @rhs);
            }elsif($type_dfn =~ /midi/){
                ## Disconnect the midi too
                @rhs and push(@result, map{"\"$lhs\" \"$_\""} @rhs);
            }
            $lhs = undef;
            @rhs = ();
            next;	    
        }	    
        die $l;
    }

    return @result;
}

## Handle a connecet or disconnect jack command: When there are no
## spaces in MIDI device names
sub handle_jack( $ ){
	 my $str = shift or die;
	 my ($cmd, $lhs, $rhs) = split(/\s/, $str);;
	 return &handle_jack_3($cmd, $lhs, $rhs);
}

## Handle a connecet or disconnect jack command: When there spaces in
## MIDI device names
sub handle_jack_3( $$$ ){
    ## Passed a Jack command execute it.  There are two: "connect"
    ## and "disconnect"
	 my ($cmd, $lhs, $rhs) = @_;

    # warn "$cmd ";
    if($cmd eq 'connect'){ ## (\S+)\s+(\S+)\s*$/){
        ## Commanded to make a connection.  Check first if it exists
        ## and there is nothing to do
        if( ! &One20Proof::test_jack_connection($lhs, $rhs)){
            # print STDERR "connect $1\t$2\n";
            print `jack_connect '$lhs' '$rhs'`;
        }
    }elsif($cmd =~ /^disconnect (\S+)\s+(\S+)\s*$/){
        if(  &One20Proof::test_jack_connection($1, $2)){
            print `jack_disconnect '$lhs' '$rhs'`;
        }
    }
}

## Remove all jack effects
## Musical Instruments.

sub initialise_yoshimi( $$ ) {
    my $name = shift or die "Pass name to use";
    my $instrument = shift or die "Pass instrument";
    -r $instrument or die "$!: '$instrument'";

    my $bin = &get_yoshimi;
    -x $bin or die "Cannot find yoshimi. Not:  '$bin'";

    ## MIDI client will be named "yoshimi-$name". Port will be 0
    my $cmd = "$bin  -I -J --alsa-midi=120Proof -c -K -L '$instrument' -N $name -R 48000";

    &run_daemon($cmd);
    &wait_for_jack($name) or die "Jack: $name not found";
    my $midi_name =  "yoshimi-$name";
    &wait_for_midi($midi_name) or die "$midi_name not found";
    
}

## Initialise foot pedal files.  Four: A, B, C, and D.  Pass the names
## of the pedal boards in the input array.  It must be four long
sub initialise_pedals( @ ) {
    my @names = @_;
    my @pedals = &list_pedals;
    my $pedals_dir = &get_pedal_dir();
    
    ## Make sure the pedals passed are all valid
    ## `@names` are the pedals we want. `@pedals` are the pedals available.
    foreach my $name (@names){
        ## Restet this if `$name` in @pedals
        my $die = 1;
        foreach my $pedal (@pedals){
            $pedal eq $name and $die = 0;
        }
        $die and die "'$name' is not a valid pedal";
    }

    ## Four pedals: A, B, C, and D
    scalar(@names) == 4 or die "Must have exactly four pedals ";

    my $pedalA = "$pedals_dir/A";
    my $pedalB = "$pedals_dir/B";
    my $pedalC = "$pedals_dir/C";
    my $pedalD = "$pedals_dir/D";

    !-e $pedalA or -l $pedalA or die "$pedalA is not a link";
    !-e $pedalB or -l $pedalB or die "$pedalB is not a link";
    !-e $pedalC or -l $pedalC or die "$pedalC is not a link";
    !-e $pedalD or -l $pedalD or die "$pedalD is not a link";

    -e $pedalA and (unlink $pedalA or die "$!");
    -e $pedalB and unlink $pedalB;
    -e $pedalC and unlink $pedalC;
    -e $pedalD and unlink $pedalD;

    my @pedal_names = qw | A B C D |;
    foreach my $name (@names){

        ## This will not fail.  Checked @names
        my $p = shift(@pedal_names) or die;

        symlink("$pedals_dir/$name", "$pedals_dir/$p") or die
            "$pedals_dir/$name $pedals_dir/$p";
    }
}

## Read a ttl, Turtle, document
## Return an array of triples (RDF)
sub read_turtle( $ ){
    my $fn = shift or die "Pass a Turtle file to process";
    open(my $fh, $fn) or die "$!: $fn";
    my @lines = map{chomp; $_} grep {$_ !~ /^\s*#/} <$fh>;

    ## Create prefixs
    my %prefix_lines = map{
        /^\@prefix (\S*):\s+<(\S*)>\s*\.$/;
        defined($1) or die "Prefix undefined: $_";
        defined($2) or die "Prefix subject undefined: $_";
        $1 => $2
    } grep{/^\@prefix /} @lines;
    @lines = grep {$_ !~ /^\@prefix /} @lines;
    my @result = ();

    ## Turtle is broken up by '.'
    my $input = join("", @lines);

    my @input = split(' \.', $input);
    ## Process the ";" 
    my @semi_colon_processed = ();
    foreach my $statement  (@input) {
        if($statement =~ /\s;\s/){
            ## There is a ' ; ' on this line
            ## The ; symbol may be used to repeat the subject of of triples that vary only in predicate and object RDF terms.semi_colon_processed
	    
            $statement =~ s/^\s*(\S+)\s+(\S+)\s+([^;]*[^;\s])\s+;// or die $statement;
            my ($subject, $predicate, $object) = ($1, $2, $3);
            # if($object =~ /\s/){
            #     ## Must be a quoted string
            #     $object =~ /"(.+)"$/ or die $object;
            # }
            # push(@semi_colon_processed, "$subject $predicate $object");
            ## This next line will break if and predicate or object
            ## has an embedded ';'
            while(1){
		my @objects = split(/\s,\s/, $object);
		for my $o (@objects){
		    push(@semi_colon_processed, "$subject $predicate $o");
		}
                $statement =~ s/^\s*(\S+)\s+([^;]*[^;\s])\s*;?// or last;
                ($predicate, $object) = ($1, $2);
            }
            
        }else{
            push(@semi_colon_processed, $statement);
        }
    }

    ## Process ' , '
    my @comma_processed = ();
    foreach my $semi (@semi_colon_processed){
        if($semi =~ / , /){
            $semi =~ s/^(\S+)\s+(\S+)\s+(\S+)\s+,//;
            my($subject, $predicate, $object) = ($1, $2, $3);
            push(@comma_processed, "$subject $predicate $object");
            my @objects = split(' , ', $semi);
            foreach $object (@objects){
                push(@comma_processed, "$subject $predicate $object");
            }
        }else{
            push(@comma_processed, $semi);
        }
    }
    return @comma_processed;
}

## Passed a file name and a start index, returns a HASH ref that
## describes all the actions required to instantiate a pedal board.
## The `index` is used to identify each effect.  This function is
## called for all the pedal boards at the same time, and each one must
## be independent.  So by initialising the index in the arguments,
## each effect, in a pedal board, across all pedal boards, can have a
## unique index
sub process_lv2_turtle( $$ ) {
    my $fn = shift or die;
    my $index = shift or die; ## Zero is invalid index
    $fn =~ /([^\/]+).ttl$/ or die $fn;
    my $pedal_board_name = $1;

    ## Break up an effect name and port.  This we do a lot
    my $name_port = sub {
        my $name_port = shift or die;
        if($name_port =~ /^(\S+)\/(\S+)/){
            return [$1, $2];
        }else{
            print "$pedal_board_name bad: $name_port\n";
            return undef;
        }
    };

    ## Strip angle brackets from around a value.  We do this a lot as
    ## it turns out
    my $strip_ang = sub {
        my $v = shift or die;
        $v =~ s/^<//;
        $v =~ s/>$//;
        $v
    };

    unless( -r $fn ){
        return ();
    }

    ## Decode the Turtle file
    my @lines = read_turtle($fn) or die "Cannot process $fn";

    ## We need to get the instructions needed to initialise this
    ## effect and turn it on.

    ## Need: 

    ## add <lv2_uri> <instance_number> Record what instance number
    ## goes with what effect so it can be communicated to the user.  

    ## param_set <instance_number> <param_symbol> <param_value>
    ## Set up the effect in the way it was saved from mod-ui

    ## Triples and their meanings
    ## predicate == "lv2:prototype" => subject is an effect, objecty is the URL.
    ## ......... <DS1> lv2:prototype <http://moddevices.com/plugins/mod-devel/DS1> 
    ## _________ Use for the "add" command
    ## predicate == ingen:arc => object names a Jack connection.
    ## .........   "<> ingen:arc _:b1"
    ## _________  Use in "jack_connect" commands
    ## predicate == lv2:port => subject is a device and object is a port of that device
    ## ......... <DS1> lv2:port  <DS1/Out1>
    ## ......... <DS1> lv2:port  <DS1/Tone>

    ## predicate == ingen:tail => subject is a Jack connection, object is where it starts
    ## predicate == ingen:head => subject is a Jack connection, object is where it ends
    ## .........  "_b2 ingen:tail <bitta/output>" 
    ## .........  "_b1 ingen:head <playback_1>
    ## predicate == a  => subject is of type object
    ## .........  <DS1/In> a lv2:AudioPort
    ## .........  <DS1/In> a lv2:InputPort
    ## _________  Use in "jack_connect" commands
    ## .........  <bitta/drywet> a lv2:ControlPort
    ## _________  Use in "param" commands
    ## predicate == "ingen:value" and subject == a control port of a device => object is a value to set a port
    ## .........  <bitta/drywet> ingen:value 1.000000
    ## _________  Use for the "param" command

    ## .........  
    ## .........  

    ## Each effect is setup in this hash.
    ## Indexed by the name	
    my %effects = ();

    ## The internal pipes between the effects that make up the pedal
    ## board and the output.  These are established at startup for all
    ## effects
    my @persistant_jack_pipes = ();

    ## The input audio pipes, and output.  Connecting these enables the effect
    ## chain that makes up the pedal board.  (TODO: What about MIDI
    ## LV2 effects?)
    my @activation_jack_pipes = ();
    
    ## each entry om @line is a triple as text.  Convert into an
    ## array of arrays, each with three elements: subject, predicate,
    ## object
    my @triples = map {
        chomp;
        /^(\S+)\s+(\S+)\s+(.+)/ or die $_;
        [$1, $2, $3]
    } @lines;

    ## Get the commands to add
    my @prototypes = grep {$_->[1] eq "lv2:prototype" } @triples;

    # To map numbers names
    my %name_number = ();
    my %number_name = ();
    
    foreach my $prototype (@prototypes){
        my ($name, $predicate, $uri) = @$prototype;

        ## The name and uri are in angle brackets
        $name = &$strip_ang($name);
        $uri = &$strip_ang($uri);

        $predicate eq "lv2:prototype" or die "Error in prototypes: $predicate";

        ## Initialise the effect hash 
        $effects{$name} = {};
        $effects{$name}->{param} = [];
        $effects{$name}->{add} = "add $uri $index";
        $name_number{$name} = $index;
        $number_name{$index} = $name;

	my @param_set = map{/^<$name\/([^>]+)> ingen:value (.+)\s*$/; "param_set $index $1 $2" } map{join(' ', @$_)} grep{$_->[0] =~ /^<$name\//} grep { $_->[1] =~ /^ingen:value$/ } @triples;
	push(@{$effects{$name}->{param}}, @param_set);
        $index += 1;
    }

    ## Get all the control ports.  As a hash so it can be used to
    ## identify `ingen:value` commands directed at the control ports
    ## of effects in the pedal board
    my $filter_port = sub {
        ## Filter for the ports wanted and get the name/port from
        ## inside the angle brackets
        my $raw = shift or die;
        $raw =~ /^([a-z0-9_]+\/[a-z0-9_\:]+)$/i or 
            # Not a name/port
            return undef; 
        return $1;
    };

    my %control_ports = map{
        &$strip_ang($_) => 1
    } grep {
        defined
    }map{
        &$filter_port(&$strip_ang($_->[0]))
    }grep {
        $_->[1] eq 'a' && $_->[2] eq 'lv2:ControlPort'
    } @triples;

    ## Get all the values for control ports
    my %control_port_values = map {
        &$strip_ang($_->[0]) => $_->[2]
    } grep {
        defined($control_ports{&$strip_ang($_->[0])})
    }grep{
        $_->[1] eq 'ingen:value'
    }grep{
        ## These are some sort of global setting
        ## TODO: Document
        $_->[0] !~ /^:/
    }@triples;

    # ## Set up the `param set` commands in effects
    foreach my $port (keys %control_port_values){
        my $value = $control_port_values{$port};
        $port =~ /([a-z_0-9]+)\/([\:a-z0-9_]+)/i or 
            die "Badly formed port: $port";
        my $name = $1;
        my $port = $2;
        my $number = $name_number{$name};
        defined($number) or die "Unknown name: $name";
        my $command = "param_set $number $port $value";
        push(@{$effects{$name}->{param}}, $command);
    }

    ## Build jack connections
    my @jack_pipes = grep{
        $_->[1] =~ /^ingen:tail$/ or
        $_->[1] =~ /^ingen:head$/ 
    }@triples;

    # There are two sorts of pipe: Internal pipes between effects, and
    # to output, are created at startup.  Activation pipes, pipes from
    # input (capture_N) to first effect in chain 
    my @jack_internal_pipes = ();
    my @jack_activation_pipes = ();
    
    foreach my $pipe (@jack_pipes){
        # `$pipe` is the name of the pipe.  The subject of the triple

        # Get the subject, predicate, and object for both ends of the pipe
        my @records = map {
            [$_->[0], $_->[1], &$strip_ang($_->[2])]
        } grep {
            # Filter by name
            $_->[0] eq $pipe->[0]
                # ## Do not implement MIDI yet.  MIDI pipes eq
                # ## 'midi_merger_out' for now, the only one I have
                # ## seen.  TODO: Make some more pedal boards with MIDI
                # ## controls and watch this die here
                # and $->[2] ne 'midi_merger_out'
        }@triples;

	## Filter out some mysterious MIDI records
        join("", map{$_->[2]} @records) =~ /midi_merger_out/ and next;
        join("", map{$_->[2]} @records) =~ /midi_capture_2/ and next;

        # One "ingen:tail" and one "ingen:head"
        scalar(@records) == 2 or die "Pipe is bad";

        my @tail = grep {$_->[1] eq "ingen:tail"} @records;
        scalar @tail == 1 or  die "Pipe is bad";

        my @head = grep {$_->[1] eq "ingen:head"} @records;
        scalar @head == 1 or  die "Pipe is bad";

        # Activation connections are connected to system:capture_N
        if($tail[0]->[2] =~ /^capture_\d+$/ and 
	       $head[0]->[2] =~ /^playback_\d+$/){
            ## A connection directly from capture to playback
            push(@jack_activation_pipes, "$tail[0]->[2]:$head[0]->[2]");
            next;
        }elsif($tail[0]->[2] =~ /^capture_\d+$/ ){
            ## A connection from the system input
            my $name_port = &$name_port($head[0]->[2]) or die;
            my $number = $name_number{$name_port->[0]};
            my $p = "system:$tail[0]->[2] effect_$number:$name_port->[1]";
            push(@jack_activation_pipes, $p);
            next;
        }elsif($head[0]->[2] =~ /^playback_\d+$/){
            # Output pipe.  An internal pipe
            my $name_port = &$name_port($tail[0]->[2]) or die;
            my $number = $name_number{$name_port->[0]};
            my $p = "effect_$number:$name_port->[1] system:$head[0]->[2]";
            push(@jack_activation_pipes, $p);
            next;
        }

        ## This is an internal pipe
        my $lhs_name_port = &$name_port($tail[0]->[2]) or die;
        my $lhs = "effect_".$name_number{$lhs_name_port->[0]}.":".
            $lhs_name_port->[1];
        my $rhs_name_port = &$name_port($head[0]->[2]) or die;
        my $rhs = "effect_".$name_number{$rhs_name_port->[0]}.":".
            $rhs_name_port->[1];
        my $p = "$lhs $rhs";
        push(@jack_internal_pipes, $p);
    }

    # my  = ();
    # my @jack_activation_pipes = ();

    my %result = (
        "effects" => \%effects,
        "index" => $index,
        "jack_activation_pipes" => \@jack_activation_pipes,
        "jack_internal_pipes" => \@jack_internal_pipes,
        "number_name" => \%number_name,
        "pedal_board_name" => $pedal_board_name
        );
    return %result;
    
}

### List all the pedal definitions
sub list_pedals {
    my $pedal_dir =  &get_pedal_dir;
    opendir(my $dir, $pedal_dir) or die $!;
    my @files =     readdir($dir);
    my @pedals =
        grep{$_ !~ /^\./} ## Not hidden file
    grep{/\S\S/} ## Not just one character
    @files;
    wantarray and return @pedals;
    return join("\n", @pedals);
}

### List the mod-host simulators.  Assume every simulator has a jack
### pipe of form: /effect_\d+/
sub list_mod_host_simulators {
    my %result = map{/effect_(\d+)/ and $1 => 1} grep {/effect_\d+/} `jack_lsp`;
    return sort {$a<=>$b} keys %result;
}

## The mod-host and jack commands for all the pedal boards
sub get_modep_simulation_commands( $ ){

    my $ignore_ref = shift or die;
    my @pedal_boards = @$ignore_ref;
    

    ## Find the start index.  Sometimes, dispite our best efforts,
    ## there are effects in place already
    my $index = 1;
    my @effects = sort {$a <=> $b} map{/effect_(\d+)/; $1} grep{/^effect_\d+/} `jack_lsp`;
    if(@effects){
        $index = $effects[$#effects];
        $index++;
    }

    ## Get the pedal board definitions
    my @fn = map{chomp;$_}grep {$_ !~ /manifest.ttl$/} grep{/\.ttl$/} `find $MODEP_PEDALS -type f`;
    my @commands = ();
    foreach my $fn (@fn){
        my %ex = &One20Proof::process_lv2_turtle($fn, $index);
        $index = $ex{index};
        push(@commands, \%ex);
    }

    # The add commands for the mod-host initialisation
    my @add_mod_host = ();

    # The param_set commands to put each effect in the designed state
    my @param_set = ();

    # Jack commands to run when we set up all the pedal boards.  Sets
    # up the connections between each effect in a pedalboard.
    my @jack_initial = (); 

    # Jack commands to run to set up a pedal board.  Indexed by name of board
    my %jack_activation = ();
    
    my %number_name = ();
    foreach my $ex (@commands){
        my %ex = %$ex;
        my $pedal_board_name = $ex{pedal_board_name} or die;
        my @effect_keys = sort keys %{$ex{effects}};
        push @jack_initial, @{$ex{jack_internal_pipes}} ;
        my @act_pipes = @{$ex{jack_activation_pipes}}; 
        $jack_activation{$pedal_board_name} = 
            \@act_pipes or die $pedal_board_name;
        foreach my $effect_name (@effect_keys){
            my $add = $ex{effects}->{$effect_name}->{add} or die $effect_name;

            map{$number_name{$_} = $ex{number_name}->{$_}} keys %{$ex{number_name}};

            push @add_mod_host, $ex{effects}->{$effect_name}->{add} or
                die $effect_name;
            push @param_set, @{$ex{effects}->{$effect_name}->{param}} or
                die $effect_name;

            
        }
    }
    return (
        add => \@add_mod_host,
        param => \@param_set,
        jack_initial => \@jack_initial,
        jack_activation => \%jack_activation,
        number_name => \%number_name
        );
}

### Getters for directories

sub get_instruments_dir {
    return "$ENV{Home120Proof}/Instruments";
}

sub get_bin {
    return "$ENV{Home120Proof}/bin";
}

sub get_pedal_dir {
    return "$ENV{Home120Proof}/pedal/PEDALS";
}

### Getters for binary programmes

sub get_lpx_blank_screen {
    return &get_bin()."/lpx_blank_screen";
}

sub get_lpx_colour {
    return &get_bin()."/lpx_colour";
}

sub get_lpx_controll {
    return &get_bin()."/lpx_controll";
}

sub get_lpx_manager {
    return &get_bin()."/lpx_manager";
}

sub get_lpx_mode {
    return &get_bin()."/lpx_mode";
}

sub get_lpx_scale {
    return &get_bin()."/lpx_scale";
}

sub get_mod_host {
    my $result = `which mod-host`;
    chomp $result;    
    return $result;
    # return "/usr/bin/mod-host";
}

sub get_pd {
    my $result = `which pd`;
    chomp $result;
    return $result;
}

sub get_pedal_driver {
    return &get_bin()."/120Proofpd";
}

## Each instrument defined in "Instruments/" has up to four pedals.
## Each pedal is named for the instrument with a "_A", "_B", "_C", or
## "_D", suffixed.
sub get_pedal_names {
    my $instrument_dir = &get_instruments_dir();
    opendir(my $dir, $instrument_dir) or die $!;
    my @names = grep{$_ !~ /^\./} # No hidden directories
    grep {$_ ne "xiz"} # A file to store Yoshimi definitions
    grep {-d "$instrument_dir/$_"} # Only directories
    readdir($dir);
    my @res = ();
    foreach my $name (@names){
        push(@res, sprintf("%s_A", $name));
        push(@res, sprintf("%s_B", $name));
        push(@res, sprintf("%s_C", $name));
        push(@res, sprintf("%s_D", $name));
    }
    return @res;
}

sub get_yoshimi {
    my $result = `which yoshimi`;
    chomp $result;
    return $result;
}

## `handle_mh_cmd` and `mod_host` set up the LV2 simulators.
## `mod_host` is passed an array of commands to send to `mod-host`
sub handle_mh_cmd( $$ ) {
    my ($sock, $cmd) = @_;
    # warn "handle_mh_cmd(SOCK, $cmd)\n";
    print $sock "$cmd\n";

    my $result = '';
    my $r = &One20Proof::fhbits($sock);
    my $res = '';
    my ($nfound, $timeleft) =
        select(my $rout = $r, my $wout = undef, my $eout = undef,
               0.5);
    # warn "handle_mh_cmd: \$nfound $nfound\n";
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


1;
