#!/usr/bin/bash
mypubkeys="$(./print.all.pubkeys.from.files.sh | grep -v -E "====|\./my")";
myfilter="$(echo $mypubkeys|sed -e 's/ /|/g')";
#echo $myfilter;
solana largest-accounts | grep -E "$myfilter";
