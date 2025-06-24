use ibapi::Client;
use ibapi::accounts::{AccountSummaries};
use ibapi::contracts::{Contract,ContractDetails};

fn main() {
    env_logger::init();

    let connection_url = "127.0.0.1:4002";
    let client = Client::connect(connection_url, 100).expect("connection to TWS failed!");

    get_cash_balance(&client);

    let contract = Contract {
        symbol: "TSLA".to_string(),
        contract_id: 76792991,
        ..Default::default()
    };
    let _cd = get_contract_details(&client,&contract);
}

fn get_contract_details(client: &Client,contract: &Contract) -> Option<ContractDetails> {
    let results = client.contract_details(&contract).expect("Error");

    return results.first().cloned();
}

fn get_cash_balance(client: &Client) {
    let group = "All";
    let subscription = client.account_summary(group, &["$LEDGER:ALL"]).expect("error requesting account summary");

    for update in &subscription {
        match update {
            AccountSummaries::Summary(summary) => {
                if summary.tag == "CashBalance" { 
                    println!("{:#?}",summary);
                }
            }
            AccountSummaries::End => {
                break;
            }
        }
    };
}
