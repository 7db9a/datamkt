use eosio::*;
use eosio_cdt::*;
use PrimaryTableCursor as ptc;

#[eosio::action]
fn createsub(
    owner: AccountName,
    sub_name: String,
) {

    require_auth(owner);

    let _self = current_receiver();
    let substbl = Subprofile::table(_self, _self);

    // Check if the owner already has a sub of the same name.
    let sub_count = substbl.iter().filter_map(|x| x.get().ok())
                             .filter(|x| x.owner == owner)
                             .filter(|x| x.sub_name == sub_name)
                             .count();
    check(sub_count == 0, "sub name already exists");

    let sub_id = substbl.available_primary_key().expect("failed to get primary key");


    let sub = Subprofile {
        sub_id,
        sub_name,
        owner,
    };

    substbl.emplace(owner, &sub).check("write");
}

eosio_cdt::abi!(createsub);

#[eosio::table("subprofile")]
struct Subprofile {
 #[eosio(primary_key)]
 sub_id: u64,
 sub_name: String,
 owner: AccountName,
}

#[eosio::table("sub")]
struct Sub {
 #[eosio(primary_key)]
 item_id: u64,
 item_name: String,
 ipfs_hash: String,
 #[eosio(secondary_key)]
 sub_id: u64,
 last_contributor: AccountName,
}
