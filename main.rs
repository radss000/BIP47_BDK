use bip47::{ChannelId, PaymentCode, SharedSecret};
use bdk::{Address, PrivateKey, PublicKey, Transaction, TransactionInput, TransactionOutput, SigHashType};
use std::collections::HashMap;


fn main() {
    // Générer deux nouvelles clés privées et publiques
    let private_key1 = PrivateKey::new();
    let public_key1 = PublicKey::from_private(&private_key1);
    let private_key2 = PrivateKey::new();
    let public_key2 = PublicKey::from_private(&private_key2);

    // Générer un nouveau code de paiement à partir de la première clé publique
    let payment_code1 = PaymentCode::from_public_key(&public_key1);

    // Générer un nouveau code de paiement à partir de la deuxième clé publique
    let payment_code2 = PaymentCode::from_public_key(&public_key2);

    // Échanger les codes de paiement pour créer un secret partagé
    let shared_secret = SharedSecret::new(&payment_code1, &payment_code2);

    // Générer un nouvel identifiant de canal à partir du secret partagé
    let channel_id = ChannelId::from_shared_secret(&shared_secret);

    // Générer deux adresses Bitcoin à partir des clés publiques
    let address1 = Address::p2pkh(&public_key1, false);
    let address2 = Address::p2pkh(&public_key2, false);

    // Créer une nouvelle tx de sortie multisig à deux clés
    let output = TransactionOutput {
        value: 1000,
        script_pubkey: address1.script_multisig(2, vec![public_key1.to_bytes(), public_key2.to_bytes()]),
    };

    // Créer une nouvelle txn d'entrée en dépensant la sortie précédemment créée
    let input = TransactionInput {
        previous_output: None,
        script_sig: private_key1.sign_multisig(
            &output.script_pubkey,
            SigHashType::All,
            0,
            &[private_key1.public_key().to_bytes()],
        ).script_sig(),
        sequence: 0xffffffff,
    };

    //Créer une tx en utilisant l'inpu et output crées 
    let tx = Transaction {
        version: 1,
        inputs: vec![input],
        outputs: vec![output],
        lock_time: 0,
    };
    // Vérifier la signature de la tx
    let result = tx.inputs[0].verify_multisig(
        &tx,
        0,
        &output.script_pubkey,
        &[public_key1.to_bytes(), public_key2.to_bytes()],
    );

    if result {
        println!("La signature de la transaction est valide");
    } else {
        println!("La signature de la transaction est invalide");
    }
    
    

struct SamouraiWallet {

    invoices: HashMap<u32, Invoice>,
}
//Structure de données pour déclarer et prendre en charge les ivoices 
#[derive(Debug)]
struct Invoice {
    invoice_number: u32,
    amount: u64,
    due_date: String,
    buyer: String,
}

impl Wallet {

    fn add_invoice(&mut self, invoice: Invoice) {
        self.invoices.insert(invoice.invoice_number, invoice);
    }

    fn get_invoice(&self, invoice_number: u32) -> Option<&Invoice> {
        self.invoices.get(&invoice_number)
    }
}

fn main() {
    let mut wallet = Wallet {
        invoices: HashMap::new(),
    };

    // Ajouter une nouvelle invoice
    let invoice = Invoice {
        invoice_number: 1,
        amount: 100,
        due_date: "2022-12-31".to_string(),
        buyer: "John Doe".to_string(),
    };
    wallet.add_invoice(invoice);

    // Rechercher une invoice par son numéro
    let invoice_number = 1;
    let invoice = wallet.get_invoice(invoice_number);
    if let Some(invoice) = invoice {
        println!("Invoice trouvée : {:?}", invoice);
    } else {
        println!("Invoice introuvable pour le numéro de facture {}", invoice_number);
    }
}

//Prise en charge des comptes connectés à l'appareil 

struct SamouraiWallet {
    

    accounts: HashMap<String, Account>,
}

#[derive(Debug)]
struct Account {
    username: String,
    email: String,
    private_key: PrivateKey,
    public_key: PublicKey,
}

impl SamouraiWallet {


    fn add_account(&mut self, account: Account) {
        self.accounts.insert(account.username.clone(), account);
    }

    fn get_account(&self, username: &str) -> Option<&Account> {
        self.accounts.get(username)
    }
}
