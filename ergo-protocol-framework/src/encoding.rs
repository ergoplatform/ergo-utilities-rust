use crate::{P2PKAddressString, P2SAddressString};
use ergo_lib::chain::address::{Address, AddressEncoder, NetworkPrefix};
use ergo_lib::ErgoTree;

pub fn p2s_string_to_ergo_tree() -> ErgoTree {
    let encoder = AddressEncoder::new(NetworkPrefix::Mainnet);
    let address = encoder
        .parse_address_from_str(&p2s_address)
        .map_err(|_| BoxVerificationError::InvalidP2SAddress)?;
    let ergo_tree = ErgoTree::sigma_parse_bytes(address.content_bytes())
        .map_err(|_| BoxVerificationError::InvalidP2SAddress)?;
}
