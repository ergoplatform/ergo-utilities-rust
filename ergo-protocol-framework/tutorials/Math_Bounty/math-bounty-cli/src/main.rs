use ergo_node_interface::*;
use math_bounty_lib::*;
use nano_get::get;

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

    if args.len() == 2 {
        // User wishes to submit nanoErgs to create a new `MathBountyBox`
        if args[1] == "bounty" {
            let bounty_amount_in_nano_ergs = args[2].parse::<u64>().unwrap();

            let ergs_box_for_bounty = ErgsBox::
        }
        // User wishes to solve the math problem to be rewarded with the
        // bounty.
        if args[1] == "solve" {
            let math_problem_answer = args[2].parse::<u64>().unwrap();
        }
    }

    println!("Args: {:?}", args);
}
