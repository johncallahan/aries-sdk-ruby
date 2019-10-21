#![recursion_limit="256"]
#[macro_use] extern crate helix;
#[macro_use] extern crate serde_json;
extern crate indyrs as indy;

use indy::pool;
use indy::wallet;
use indy::did;
use indy::ledger;
use indy::anoncreds;
use std::string::String;

use indy::future::Future;

use std::env;
use std::fs;
use std::io::Write;
use std::path::PathBuf;

ruby! {
    class AriesWallet {
        struct {
            name: String,
            handle: i32
        }

        def initialize(helix, name: String) {
            let handle = 0;
            AriesWallet { helix, name, handle }
        }

        def create(&self) {
            let config = format!("{{\"id\":\"wallet{}\"}}", self.name);
            let credentials = String::from("{\"key\":\"8dvfYSt5d1taSd6yJdpjq4emkwsPDDLYxkNFysFD2cZY\",\"key_derivation_method\":\"RAW\"}");

            wallet::create_wallet(&config, &credentials).wait().unwrap();
        }
        def open(&mut self) {
            let config = format!("{{\"id\":\"wallet{}\"}}", self.name);
            let credentials = String::from("{\"key\":\"8dvfYSt5d1taSd6yJdpjq4emkwsPDDLYxkNFysFD2cZY\",\"key_derivation_method\":\"RAW\"}");

            self.handle = wallet::open_wallet(&config, &credentials).wait().unwrap();
        }
        def close(&self) {
            wallet::close_wallet(self.handle).wait().unwrap();
        }
        def delete(&self) {
            let config = format!("{{\"id\":\"wallet{}\"}}", self.name);
            let credentials = String::from("{\"key\":\"8dvfYSt5d1taSd6yJdpjq4emkwsPDDLYxkNFysFD2cZY\",\"key_derivation_method\":\"RAW\"}");

            wallet::delete_wallet(&config, &credentials).wait().unwrap();
        }
        def get_handle(&self) -> i32 {
            return self.handle
        }
    }

    class AriesPool {
        struct {
            name: String,
            handle: i32
        }

        def initialize(helix, name: String) {
            let handle = 0;
            const PROTOCOL_VERSION: usize = 2;
            pool::set_protocol_version(PROTOCOL_VERSION).wait().unwrap();
            AriesPool { helix, name, handle }
        }
        
        def create(&self) {
            let pool_config_file = create_genesis_txn_file_for_pool(&self.name);
            let pool_config = json!({
                "genesis_txn" : &pool_config_file
            });
            pool::create_pool_ledger_config(&self.name, Some(&pool_config.to_string())).wait().unwrap();
        }
        def open(&mut self) {
            self.handle = pool::open_pool_ledger(&self.name, None).wait().unwrap();
        }
        def close(&self) {
            pool::close_pool_ledger(self.handle).wait().unwrap();
        }
        def delete(&self) {
            pool::delete_pool_ledger(&self.name).wait().unwrap();
        }
        def get_handle(&self) -> i32 {
            return self.handle
        }
    }

    class AriesDID {
        struct {
            did: String,
            verkey: String
        }

        def initialize(helix) {
            let did: String = "".to_string();
            let verkey: String = "".to_string();
            AriesDID { helix, did, verkey }
        }

        def create(&mut self, wallet: &AriesWallet, value: String) {
            let (did,verkey) = create_did(wallet.handle, &value);
            self.did = did;
            self.verkey = verkey;
        }

        def get_did(&self) -> String {
            return self.did.to_string();
        }

        def build_nym(steward_did: &AriesDID, trustee_did: &AriesDID) -> String {
            return ledger::build_nym_request(&steward_did.did, &trustee_did.did, Some(&trustee_did.verkey), None, Some("TRUST_ANCHOR")).wait().unwrap();
        }

        def get_verkey(&self) -> String {
            return self.verkey.to_string();
        }
    }

    class AriesJson {
        def to_string(data: String) -> String {
            let value: serde_json::Value = serde_json::from_str(&data).unwrap();
            let result = value.to_string();
            return result;
        }
    }

    class AriesCredential {
        struct {
            schema_id: String,
            schema_json: String
        }

        def initialize(helix) {
            let schema_id: String = "".to_string();
            let schema_json: String = "".to_string();
            AriesCredential { helix, schema_id, schema_json }
        }
        
        def issuer_create_schema(&mut self,issuer_did: &AriesDID, name: String, version: String, attributes: String) {
            let (schema_id,schema_json) = anoncreds::issuer_create_schema(&issuer_did.did, &name, &version, &attributes).wait().unwrap();
            self.schema_id = schema_id;
            self.schema_json = schema_json;
        }
    }
}

fn create_did(wallet_handle: i32, value: &str) -> (String,String) {
    let (did,verkey) = did::create_and_store_my_did(wallet_handle, value).wait().unwrap();
    return (did,verkey);
}

fn create_genesis_txn_file_for_pool(pool_name: &str) -> String {
    let test_pool_ip = env::var("TEST_POOL_IP").unwrap_or("127.0.0.1".to_string());

    let node_txns = format!(
        r#"{{"reqSignature":{{}},"txn":{{"data":{{"data":{{"alias":"Node1","blskey":"4N8aUNHSgjQVgkpm8nhNEfDf6txHznoYREg9kirmJrkivgL4oSEimFF6nsQ6M41QvhM2Z33nves5vfSn9n1UwNFJBYtWVnHYMATn76vLuL3zU88KyeAYcHfsih3He6UHcXDxcaecHVz6jhCYz1P2UZn2bDVruL5wXpehgBfBaLKm3Ba","blskey_pop":"RahHYiCvoNCtPTrVtP7nMC5eTYrsUA8WjXbdhNc8debh1agE9bGiJxWBXYNFbnJXoXhWFMvyqhqhRoq737YQemH5ik9oL7R4NTTCz2LEZhkgLJzB3QRQqJyBNyv7acbdHrAT8nQ9UkLbaVL9NBpnWXBTw4LEMePaSHEw66RzPNdAX1","client_ip":"{0}","client_port":9702,"node_ip":"{0}","node_port":9701,"services":["VALIDATOR"]}},"dest":"Gw6pDLhcBcoQesN72qfotTgFa7cbuqZpkX3Xo6pLhPhv"}},"metadata":{{"from":"Th7MpTaRZVRYnPiabds81Y"}},"type":"0"}},"txnMetadata":{{"seqNo":1,"txnId":"fea82e10e894419fe2bea7d96296a6d46f50f93f9eeda954ec461b2ed2950b62"}},"ver":"1"}}
           {{"reqSignature":{{}},"txn":{{"data":{{"data":{{"alias":"Node2","blskey":"37rAPpXVoxzKhz7d9gkUe52XuXryuLXoM6P6LbWDB7LSbG62Lsb33sfG7zqS8TK1MXwuCHj1FKNzVpsnafmqLG1vXN88rt38mNFs9TENzm4QHdBzsvCuoBnPH7rpYYDo9DZNJePaDvRvqJKByCabubJz3XXKbEeshzpz4Ma5QYpJqjk","blskey_pop":"Qr658mWZ2YC8JXGXwMDQTzuZCWF7NK9EwxphGmcBvCh6ybUuLxbG65nsX4JvD4SPNtkJ2w9ug1yLTj6fgmuDg41TgECXjLCij3RMsV8CwewBVgVN67wsA45DFWvqvLtu4rjNnE9JbdFTc1Z4WCPA3Xan44K1HoHAq9EVeaRYs8zoF5","client_ip":"{0}","client_port":9704,"node_ip":"{0}","node_port":9703,"services":["VALIDATOR"]}},"dest":"8ECVSk179mjsjKRLWiQtssMLgp6EPhWXtaYyStWPSGAb"}},"metadata":{{"from":"EbP4aYNeTHL6q385GuVpRV"}},"type":"0"}},"txnMetadata":{{"seqNo":2,"txnId":"1ac8aece2a18ced660fef8694b61aac3af08ba875ce3026a160acbc3a3af35fc"}},"ver":"1"}}
           {{"reqSignature":{{}},"txn":{{"data":{{"data":{{"alias":"Node3","blskey":"3WFpdbg7C5cnLYZwFZevJqhubkFALBfCBBok15GdrKMUhUjGsk3jV6QKj6MZgEubF7oqCafxNdkm7eswgA4sdKTRc82tLGzZBd6vNqU8dupzup6uYUf32KTHTPQbuUM8Yk4QFXjEf2Usu2TJcNkdgpyeUSX42u5LqdDDpNSWUK5deC5","blskey_pop":"QwDeb2CkNSx6r8QC8vGQK3GRv7Yndn84TGNijX8YXHPiagXajyfTjoR87rXUu4G4QLk2cF8NNyqWiYMus1623dELWwx57rLCFqGh7N4ZRbGDRP4fnVcaKg1BcUxQ866Ven4gw8y4N56S5HzxXNBZtLYmhGHvDtk6PFkFwCvxYrNYjh","client_ip":"{0}","client_port":9706,"node_ip":"{0}","node_port":9705,"services":["VALIDATOR"]}},"dest":"DKVxG2fXXTU8yT5N7hGEbXB3dfdAnYv1JczDUHpmDxya"}},"metadata":{{"from":"4cU41vWW82ArfxJxHkzXPG"}},"type":"0"}},"txnMetadata":{{"seqNo":3,"txnId":"7e9f355dffa78ed24668f0e0e369fd8c224076571c51e2ea8be5f26479edebe4"}},"ver":"1"}}
           {{"reqSignature":{{}},"txn":{{"data":{{"data":{{"alias":"Node4","blskey":"2zN3bHM1m4rLz54MJHYSwvqzPchYp8jkHswveCLAEJVcX6Mm1wHQD1SkPYMzUDTZvWvhuE6VNAkK3KxVeEmsanSmvjVkReDeBEMxeDaayjcZjFGPydyey1qxBHmTvAnBKoPydvuTAqx5f7YNNRAdeLmUi99gERUU7TD8KfAa6MpQ9bw","blskey_pop":"RPLagxaR5xdimFzwmzYnz4ZhWtYQEj8iR5ZU53T2gitPCyCHQneUn2Huc4oeLd2B2HzkGnjAff4hWTJT6C7qHYB1Mv2wU5iHHGFWkhnTX9WsEAbunJCV2qcaXScKj4tTfvdDKfLiVuU2av6hbsMztirRze7LvYBkRHV3tGwyCptsrP","client_ip":"{0}","client_port":9708,"node_ip":"{0}","node_port":9707,"services":["VALIDATOR"]}},"dest":"4PS3EDQ3dW1tci1Bp6543CfuuebjFrg36kLAUcskGfaA"}},"metadata":{{"from":"TWwCRQRZ2ZHMJFn9TzLp7W"}},"type":"0"}},"txnMetadata":{{"seqNo":4,"txnId":"aa5e817d7cc626170eca175822029339a444eb0ee8f0bd20d3b0b76e566fb008"}},"ver":"1"}}"#, test_pool_ip);

    let pool_config_pathbuf = write_genesis_txn_to_file(pool_name, node_txns.as_str());
    pool_config_pathbuf.as_os_str().to_str().unwrap().to_string()
}

fn write_genesis_txn_to_file(pool_name: &str,
                             txn_file_data: &str) -> PathBuf {
    let mut txn_file_path = env::temp_dir();
    txn_file_path.push("indy_client");
    txn_file_path.push(format!("{}.txn", pool_name));

    if !txn_file_path.parent().unwrap().exists() {
        fs::DirBuilder::new()
            .recursive(true)
            .create(txn_file_path.parent().unwrap()).unwrap();
    }

    let mut f = fs::File::create(txn_file_path.as_path()).unwrap();
    f.write_all(txn_file_data.as_bytes()).unwrap();
    f.flush().unwrap();
    f.sync_all().unwrap();

    txn_file_path
}
