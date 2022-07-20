#!/usr/bin/bash

keypairfiles=$(ls -t $(find . -name "*.json" | grep -v -E "debug|bpfel|release|test-ledger|rustc"));
for file in $keypairfiles;
do
    content=$(cat $file);
    if [ "$content" = "" ]; then continue; fi;
    pubkey=$(solana-keygen pubkey $file);
    echo $file
    echo $pubkey;
    if [ "$1" = "detail" ];
    then
      solana account $pubkey;
      echo;
    fi;
    echo "========================================";
done;
