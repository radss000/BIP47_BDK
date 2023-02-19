use bdk::bitcoin::secp256k1::Secp256k1;
use bdk::bitcoin::util::bip32::{DerivationPath, KeySource};
use bdk::bitcoin::Amount;
use bdk::bitcoin::Network;
use bdk::bitcoincore_rpc::jsonrpc::serde_json::Value;
use bdk::bitcoincore_rpc::{Auth as rpc_auth, Client, RpcApi};

use bdk::blockchain::rpc::{Auth, RpcBlockchain, RpcConfig};
use bdk::blockchain::{ConfigurableBlockchain, NoopProgress};

use bdk::keys::bip39::{Language, Mnemonic, WordCount};
use bdk::keys::DescriptorKey::Secret;
use bdk::keys::{DerivableKey, DescriptorKey, ExtendedKey, GeneratableKey, GeneratedKey};

use bdk::miniscript::miniscript::Segwitv0;

use bdk::wallet::{signer::SignOptions, wallet_name_from_descriptor, AddressIndex};
use bdk::{Wallet, SyncOptions};

use bdk::sled;

use std::str::FromStr;

fn main() {
    // Create a RPC interface
    let rpc_auth = rpc_auth::UserPass("lnd".to_string(), "lightning".to_string());
    let core_rpc = Client::new("http://127.0.0.1:18443/wallet/test", rpc_auth).unwrap();
    // println!("{:#?}", core_rpc.get_blockchain_info().unwrap());

    // Create the test wallet
    // core_rpc.create_wallet("test", None, None, None, None).unwrap();
    let args = [
        Value::String(String::from("test")),
        Value::Bool(false),
        Value::Bool(false),
        Value::Null,
        Value::Bool(false),
        Value::Bool(false),
        Value::Bool(true),
        Value::Bool(false),
    ];
    let _: Value = core_rpc.call("createwallet", &args).unwrap();

    // Get a new address
    // let core_address = core_rpc.get_new_address(None, None).unwrap();

    // Generate 101 blocks and use the above address as coinbase
    // core_rpc.generate_to_address(101, &core_address).unwrap();

    // fetch the new balance
    let core_balance = core_rpc.get_balance(None, None).unwrap();

    // Show balance
    println!("core balance: {:#?}", core_balance);

    let (receive_desc, change_desc) = get_descriptors();

    // Use deterministic wallet name derived from descriptor
    let wallet_name = wallet_name_from_descriptor(
        &receive_desc,
        Some(&change_desc),
        Network::Regtest,
        &Secp256k1::new(),
    )
    .unwrap();

    // Create the datadir to store wallet data
    let mut datadir = dirs_next::home_dir().unwrap();
    datadir.push(".bdk-example");
    let database = sled::open(datadir).unwrap();
    let db_tree = database.open_tree(wallet_name.clone()).unwrap();

    // Set RPC username, password and url
    let auth = Auth::UserPass {
        username: "lnd".to_string(),
        password: "lightning".to_string(),
    };
    let mut rpc_url = "http://".to_string();
    rpc_url.push_str("127.0.0.1:18443");

    // Setup the RPC configuration
    let rpc_config = RpcConfig {
        url: rpc_url,
        auth,
        network: Network::Regtest,
        wallet_name,
        skip_blocks: None,
    };

    // Use the above configuration to create a RPC blockchain backend
    let blockchain = RpcBlockchain::from_config(&rpc_config).unwrap();

    // Combine everything and finally create the BDK wallet structure
    let wallet = Wallet::new(&receive_desc, Some(&change_desc), Network::Regtest, db_tree).unwrap();

    // Sync the wallet
    wallet.sync(&blockchain, SyncOptions::default()).unwrap();

    // Fetch a fresh address to receive coins
    let address = wallet.get_address(AddressIndex::New).unwrap().address;

    println!("bdk address: {:#?}", address);

    // println!("recv: {:#?}, \nchng: {:#?}", receive_desc, change_desc);
}

// generate fresh descriptor strings and return them via (receive, change) tuple
fn get_descriptors() -> (String, String) {
    // Create a new secp context
    let secp = Secp256k1::new();

    // You can also set a password to unlock the mnemonic
    let password = Some("random password".to_string());

    // Generate a fresh mnemonic, and from there a privatekey
    let mnemonic: GeneratedKey<_, Segwitv0> =
        Mnemonic::generate((WordCount::Words12, Language::English)).unwrap();
    let mnemonic = mnemonic.into_key();
    let xkey: ExtendedKey = (mnemonic, password).into_extended_key().unwrap();
    let xprv = xkey.into_xprv(Network::Regtest).unwrap();

    // Create derived privkey from the above master privkey
    // We use the following derivation paths for receive and change keys
    // receive: "m/84h/1h/0h/0"
    // change: "m/84h/1h/0h/1"
    let mut keys = Vec::new();

    for path in ["m/84h/1h/0h/0", "m/84h/1h/0h/1"] {
        let deriv_path: DerivationPath = DerivationPath::from_str(path).unwrap();
        let derived_xprv = &xprv.derive_priv(&secp, &deriv_path).unwrap();
        let origin: KeySource = (xprv.fingerprint(&secp), deriv_path);
        let derived_xprv_desc_key: DescriptorKey<Segwitv0> = derived_xprv
            .into_descriptor_key(Some(origin), DerivationPath::default())
            .unwrap();

        // Wrap the derived key with the wpkh() string to produce a descriptor string
        if let Secret(key, _, _) = derived_xprv_desc_key {
            let mut desc = "wpkh(".to_string();
            desc.push_str(&key.to_string());
            desc.push_str(")");
            keys.push(desc);
        }
    }

    // Return the keys as a tuple
    (keys[0].clone(), keys[1].clone())
}
