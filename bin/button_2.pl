#!/usr/bin/perl -w
use strict;
print "------- $0 \n";
print `sudo  --preserve-env=Home120Proof  -u patch /home/patch/120Proof/bin/Mistress  KILL` ;
# print `systemctl restart jack.service`  ;
print `sudo systemctl start modep-mod-ui.service`;
print "------- $0 $< $ENV{Home120Proof}\n";#  
