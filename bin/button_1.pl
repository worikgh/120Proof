#!/usr/bin/perl -w
use strict;
print "------- $0 \n";
print `systemctl stop modep-mod-host.service`;
print `sudo -u patch --preserve-env /home/patch/120Proof/bin/Mistress`;
print "------- $0 $< $ENV{Home120Proof}\n";#  
