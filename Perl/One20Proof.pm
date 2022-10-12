package One20Proof;
use POSIX "setsid";

sub pkill( $ ){
    my $prog_name = shift or die;
    
    if(`pgrep $prog_name`){
	`pkill $prog_name`;
	$? and die "$?: Failed to kill $prog_name\n";
    }
}

sub run_daemon( $ ) {
    my $cmd = shift or die "Must pass command";
    defined(my $pid = fork())   or die "can't fork: $!";
    return($pid) if $pid;               # non-zero now means I am the parent
    (setsid() != -1)            or die "Can't start a new session: $!";
    `$cmd`;
}

1;
