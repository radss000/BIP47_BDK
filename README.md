# BIP47_BDK

This code implements a Samourai Wallet, which is a wallet for managing Bitcoin transactions and invoices.

There are several parts to this code:

Generating private and public keys, and creating payment codes and shared secrets. These are used to establish a payment channel between two users of Samourai Wallet.

Creating a Bitcoin transaction using a two-key multisig output and spending this output with a signed transaction input using a private key. The signature of the transaction is verified to ensure it is valid.

Implementing a Invoice data structure to store information about invoices, and a Wallet data structure that contains a hash map (HashMap) to store these invoices. The Wallet data structure has methods for adding invoices and retrieving invoices using their invoice number.

Using these data structures to add a new invoice to the wallet and to retrieve an existing invoice using its invoice number.

Features are coming such as: 
- Adding support for different cryptocurrency assets beyond just Bitcoin.
- Tracking the current value of the user's cryptocurrency assets.
- Feature for backing up and restoring the wallet. This could allow users to protect their assets in case they lose access to their device or wallet.
And much more !
