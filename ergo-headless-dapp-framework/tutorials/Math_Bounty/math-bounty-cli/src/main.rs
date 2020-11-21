use ergo_node_interface::*;
use math_bounty_headless::*;
use reqwest::blocking::get;

fn main() {
    // Get a `NodeInterface`
    let node = acquire_node_interface_from_local_config();
    // Get the current Ergo Blockchain block height
    let block_height = node.current_block_height().unwrap();
    // Get the first address in the user's wallet
    let user_address = node.wallet_addresses().unwrap()[0].clone();

    // Acquire CLI arguments
    let args: Vec<String> = std::env::args().collect();
    let tx_fee = 1000000;

    if args.len() == 3 {
        // User wishes to submit Ergs to create a new `MathBountyBox`
        if args[1] == "bounty" {
            // Taking user input as Ergs and converting to nanoErgs
            let bounty_amount_in_nano_ergs = erg_to_nano_erg(args[2].parse::<f64>().unwrap());

            // Acquire the ergs_box_for_bounty
            let ergs_box_for_bounty =
                get_ergs_box_for_bounty(user_address.clone(), bounty_amount_in_nano_ergs);

            println!("Ergs Box For Bounty: {:?}", ergs_box_for_bounty);
        }
        // User wishes to solve the math problem to be rewarded with the
        // bounty.
        if args[1] == "solve" {
            let math_problem_answer = args[2].parse::<u64>().unwrap();
        }
    }
}

pub fn get_ergs_box_for_bounty(user_address: String, bounty_amount_in_nano_ergs: u64) -> ErgsBox {
    // Take the generalized `BoxSpec` from an `ErgsBox` and modify it
    // for our use case. Specifically change the address to be our
    // user's address, and change the value_range so that the box
    // has enough to cover the bounty amount.
    let ergs_box_for_bounty_spec = ErgsBox::box_spec()
        .modified_address(Some(user_address))
        .modified_value_range(Some(bounty_amount_in_nano_ergs..u64::MAX));
    // Acquire the Ergo Explorer API endpoint in order to find
    // the our `ergs_box_for_bounty`.
    let ergs_box_for_bounty_url = ergs_box_for_bounty_spec
        .explorer_endpoint("https://api.ergoplatform.com/api/v0/")
        .unwrap();
    // Make a get request to the Ergo Explorer API endpoint
    let get_response = get(&ergs_box_for_bounty_url).unwrap().text().unwrap();
    // Process the `get_response` into `ErgsBox`es which match our
    // `ergs_box_for_bounty_spec`
    let list_of_ergs_boxes =
        ErgsBox::process_explorer_response_custom(&get_response, ergs_box_for_bounty_spec).unwrap();

    // Return the first `ErgsBox` from the list
    list_of_ergs_boxes[0].clone()
}
