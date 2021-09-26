mod entities;
use fake::{Fake, StringFaker};
use entities::*;
use entities::generators::*;
use std::vec::Vec;
use postgres::{Client, NoTls};
use std::io::Write;
use std::convert::TryInto;

fn insert_with_copy<T>(collection: &Vec<T>) -> ()
    where T: SqlInsert + CommaDelimited
{
    let mut client = Client::connect("host=localhost user=lukas password=lukas", NoTls).expect("Failed joining to postgres");
    let query = format!("COPY {} FROM STDIN WITH DELIMITER AS ',' NULL AS 'nul_val'", T::insert_header());
    let mut writer = client
        .copy_in(&query[..])
        .expect("Failed to create copy in writer");
    for item in collection {
        let csv = item.to_csv();
        writer.write_all(csv.as_bytes()).expect("Error while writing to STDIN to copy");
    }

    writer.finish().expect("Failed to finish copying");
}

fn get_last_identities() -> (u32, u32, u32, u32, u32, u32, u32) {
    let mut client = Client::connect("host=localhost user=lukas password=lukas", NoTls).expect("Failed joining to postgres");
    let cid: i64 = client.query_one("select last_value from contract_contract_id_seq", &[]).expect("Failed to get contract id value").get(0);
    let pid: i64 = client.query_one("select last_value from participant_participant_id_seq", &[]).expect("Failed to get participant id value").get(0);
    let aid: i64 = client.query_one("select last_value from address_address_id_seq", &[]).expect("Failed to get address id value").get(0);
    let vid: i64 = client.query_one("select last_value from voip_number_number_id_seq", &[]).expect("Failed to get voip_number id value").get(0);
    let prid: i64 = client.query_one("select last_value from price_list_price_list_id_seq", &[]).expect("Failed to get price_list id value").get(0);
    let iiid: i64 = client.query_one("select last_value from invoice_item_item_id_seq", &[]).expect("Failed to get invoice_item id value").get(0);
    let cdrid: i64 = client.query_one("select last_value from call_detail_record_call_id_seq", &[]).expect("Failed to get call_detail_record id value").get(0);
    (
       cid.try_into().unwrap(),
       pid.try_into().unwrap(),
       aid.try_into().unwrap(),
       vid.try_into().unwrap(),
       prid.try_into().unwrap(),
       iiid.try_into().unwrap(),
       cdrid.try_into().unwrap(),
    )

}


fn main() -> () {
    let (mut cid, mut pid, mut aid, mut vid, mut prid, mut iid, mut cdrid) = get_last_identities();

    let mut vs_symbol = 100_000;
    let contracts_total = 100_000;
    let mut contracts: Vec<Contract> = Vec::<Contract>::with_capacity(contracts_total);
    for _ in 1..=contracts_total {
        vs_symbol += 1;
        cid += 1;
        contracts.push(gen_contract(cid, vs_symbol));
    }
    insert_with_copy(&contracts);


    let string_faker = StringFaker::with(
        String::from("0123456789abcdef").into_bytes(),
        64..65,
    );

    let mut participants: Vec<Participant> = Vec::<Participant>::with_capacity(contracts_total * 2);
    {
        let mut addresses: Vec<Address> = Vec::<Address>::with_capacity(contracts_total);
        for c in contracts.iter() {
            let participaints_count = (1..=4).fake::<u8>();
            let mut idx = 0;
            aid += 1;
            while idx < participaints_count {
                pid += 1;
                participants.push(gen_participant(pid, c.contract_id.unwrap(), &string_faker));
                idx += 1;
            }
            addresses.push(gen_address(aid, c.contract_id.unwrap()));
        }

        insert_with_copy(&addresses);
    }
    insert_with_copy(&participants);

}
