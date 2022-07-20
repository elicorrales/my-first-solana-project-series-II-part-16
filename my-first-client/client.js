const {
  Keypair,
  Connection,
  TransactionInstruction,
  Transaction,
  SystemInstruction,
  SystemProgram,
} = require('@solana/web3.js');

const wallet = Keypair.fromSecretKey(
  new Uint8Array([
    139, 92, 115, 77, 215, 145, 197, 65, 89, 39, 118, 140, 252, 151, 13, 143,
    59, 242, 131, 15, 197, 217, 141, 226, 14, 82, 133, 121, 90, 72, 189, 71,
    33, 36, 12, 18, 191, 132, 216, 76, 21, 25, 98, 4, 97, 187, 48, 186, 193,
    142, 63, 243, 4, 235, 151, 241, 147, 71, 127, 48, 25, 154, 135, 78
  ])
);

const program = Keypair.fromSecretKey(
  new Uint8Array([
    214, 156, 249, 227, 56, 119, 47, 204, 103, 253, 132, 100, 251, 46, 169, 47,
    228, 225, 83, 9, 60, 243, 198, 197, 114, 175, 18, 0, 240, 156, 168, 26, 39,
    113, 57, 12, 202, 253, 63, 149, 192, 113, 151, 90, 72, 123, 39, 180, 167,
    230, 31, 36, 146, 154, 103, 194, 4, 88, 44, 243, 216, 158, 98, 172
  ])
);

if (process.argv.length < 3) {
  console.log('\n\nNEED A KEYPAIR JSON ARRAY STRING\n\n');
  return;
}

let acctKeypairString = process.argv[2];
let acctKeypair = Keypair.fromSecretKey(new Uint8Array(JSON.parse(acctKeypairString)));
let acctPublicKey = acctKeypair.publicKey;
console.log(acctPublicKey.toString());


(async () => {
  console.log('Establishing Connection...');
  let rpcUrl = 'http://localhost:8899';
  let connection = new Connection(rpcUrl, 'confirmed');

  console.log('Get Version...');
  const version = await connection.getVersion()
  console.log('Version: ', version);

  let acctToBeCreatedTransInstruction = SystemProgram.createAccount({
    fromPubkey: wallet.publicKey,
    newAccountPubkey: acctKeypair.publicKey,
    programId: program.publicKey,
    lamports: 100000000000,
    space: 1
  })

  let instruction = new TransactionInstruction(acctToBeCreatedTransInstruction);

  let transaction = new Transaction();
  transaction.add(instruction);

  await connection.sendTransaction(transaction, [wallet, acctKeypair]);

  console.log('Done.');

})()
