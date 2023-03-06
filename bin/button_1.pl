#!/usr/bin/perl -w
use strict;
print "------- $0 \n";
print `sudo systemctl stop modep-mod-host.service`;
print `sudo -u patch --preserve-env $ENV{Home120Proof}/bin/Mistress 2>&1 >> $ENV{Home120Proof}/output/Mistress.log`;
print "------- $0 $< $ENV{Home120Proof}$ENV{Home120Proof}\n";#  
