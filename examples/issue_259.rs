use std::thread;

use ibapi::prelude::*;

fn main() {
    env_logger::init();

    let client = Client::connect("127.0.0.1:4002", 100).expect("connection failed");

    let group = "All";

    println!("Requesting account summary 1 for group: {group}");

    let subscription = client
        .account_summary(group, AccountSummaryTags::ALL)
        .expect("error requesting account summary");

    for update in &subscription {
        match update {
            AccountSummaries::Summary(summary) => println!("{summary:?}"),
           AccountSummaries::End => subscription.cancel(),
            // AccountSummaries::End => println!("End of account summary for group: {group}"),
        }
    }

    // thread::sleep(std::time::Duration::from_secs(200));

    println!("Requesting account summary 2 for group: {group}");

    let subscription = client
        .account_summary(group, AccountSummaryTags::ALL)
        .expect("error requesting account summary");

    for update in &subscription {
        match update {
            AccountSummaries::Summary(summary) => println!("{summary:?}"),
            AccountSummaries::End => subscription.cancel(),
        }
    }
}
