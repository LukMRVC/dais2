mod entities;
use entities::generators::*;
use entities::*;
use fake::{Fake, StringFaker};
use postgres::{Client, Config, NoTls};
use std::convert::TryInto;
use std::env;
use std::io::Write;
use std::vec::Vec;

fn insert_with_copy<T>(cfg: &Config, collection: &Vec<T>) -> ()
where
    T: SqlInsert + CommaDelimited + RecreatesForeignKeys,
{
    let mut client = cfg.connect(NoTls).expect("Failed joining to postgres");
    let query: Option<&'static str> = T::drop_fk();
    if query.is_some() {
        let queries = query.unwrap().split(';');
        for q in queries {
            client
            .execute(q, &[])
            .expect("Failed to drop foreign keys");
        }
    }

    let query = format!(
        "COPY {} FROM STDIN WITH DELIMITER AS ',' NULL AS 'nul_val'",
        T::insert_header()
    );
    let mut writer = client
        .copy_in(&query[..])
        .expect("Failed to create copy in writer");
    for item in collection {
        let csv = item.to_csv();
        writer
            .write_all(csv.as_bytes())
            .expect("Error while writing to STDIN to copy");
    }
    writer.finish().expect("Failed to finish copying");

    let query = T::recreate_fk();
    if query.is_some() {
        let queries = query.unwrap().split(';');
        for q in queries {
            client
            .execute(q, &[])
            .expect("Failed to disable triggers");
        }
    }
    
}

fn get_last_identities(cfg: &Config) -> (u32, u32, u32, u32, u32, u32, u32, i32) {
    let mut client = cfg.connect(NoTls).expect("Failed joining to postgres");
    let cid: i64 = client
        .query_one("select last_value from contract_contract_id_seq", &[])
        .expect("Failed to get contract id value")
        .get(0);
    let pid: i64 = client
        .query_one("select last_value from participant_participant_id_seq", &[])
        .expect("Failed to get participant id value")
        .get(0);
    let aid: i64 = client
        .query_one("select last_value from address_address_id_seq", &[])
        .expect("Failed to get address id value")
        .get(0);
    let vid: i64 = client
        .query_one("select last_value from voip_number_number_id_seq", &[])
        .expect("Failed to get voip_number id value")
        .get(0);
    let prid: i64 = client
        .query_one("select last_value from price_list_price_list_id_seq", &[])
        .expect("Failed to get price_list id value")
        .get(0);
    let iiid: i64 = client
        .query_one("select last_value from invoice_item_item_id_seq", &[])
        .expect("Failed to get invoice_item id value")
        .get(0);
    let cdrid: i64 = client
        .query_one("select last_value from call_detail_record_call_id_seq", &[])
        .expect("Failed to get call_detail_record id value")
        .get(0);
    let max_invoice_number: i32 = client
        .query_one(
            "select greatest(max(invoice_number), 97000000) from invoice",
            &[],
        )
        .expect("Failed to get max invoice number")
        .get(0);
    (
        cid.try_into().unwrap(),
        pid.try_into().unwrap(),
        aid.try_into().unwrap(),
        vid.try_into().unwrap(),
        prid.try_into().unwrap(),
        iiid.try_into().unwrap(),
        cdrid.try_into().unwrap(),
        max_invoice_number,
    )
}

fn main() -> () {
    use fake::faker::boolean::en::Boolean;
    let args: Vec<String> = env::args().collect();
    if args.len() <= 6 {
        println!(
            "Please provide arguments in form of {{hostname}} {{user}} {{password}} {{dbname}} {{contract_count}} {{calls_count}}"
        );
        return ();
    }

    let db_hostname = &args[1];
    let db_user = &args[2];
    let db_pass = &args[3];
    let db_name = &args[4];
    let contracts_total: usize = args[5].parse::<usize>().unwrap();
    let calls_count: usize = args[6].parse::<usize>().unwrap();
    let mut cfg: Config = Client::configure();
    cfg.host(db_hostname);
    cfg.user(db_user);
    cfg.password(db_pass);
    cfg.dbname(db_name);

    let (mut cid, mut pid, mut aid, mut vid, prid, iid, cdrid, in_num) = get_last_identities(&cfg);

    let mut in_num = in_num as i64;

    let mut vs_symbol = 100_000;
    let mut contracts: Vec<Contract> = Vec::<Contract>::with_capacity(contracts_total);
    for _ in 1..=contracts_total {
        vs_symbol += 1;
        cid += 1;
        contracts.push(gen_contract(cid, vs_symbol));
    }
    println!("INSERTING contracts");
    insert_with_copy(&cfg, &contracts);

    let password_faker = StringFaker::with(String::from("0123456789abcdef").into_bytes(), 64..65);
    {
        let mut participants: Vec<Participant> =
            Vec::<Participant>::with_capacity(contracts_total * 2);

        let mut number_requests: Vec<NumberRequest> =
            Vec::<NumberRequest>::with_capacity(contracts_total);
        {
            let mut addresses: Vec<Address> = Vec::<Address>::with_capacity(contracts_total);
            for c in contracts.iter() {
                let participaints_count = (1..=4).fake::<u8>();
                let mut idx = 0;
                aid += 1;
                while idx < participaints_count {
                    pid += 1;
                    participants.push(gen_participant(
                        pid,
                        c.contract_id.unwrap(),
                        &password_faker,
                    ));
                    idx += 1;
                }
                addresses.push(gen_address(aid, c.contract_id.unwrap()));
            }
            println!("INSERTING addresses");
            insert_with_copy(&cfg, &addresses);
        }
        println!("INSERTING participants");
        insert_with_copy(&cfg, &participants);

        let mut voip_numbers: Vec<VoipNumber> =
            Vec::<VoipNumber>::with_capacity(participants.len() * 2);
        let password_faker =
            StringFaker::with(String::from("0123456789abcdef").into_bytes(), 32..33);

        println!(
            "GENERATING voip_numbers, maximum of {}",
            participants.len() * 4
        );
        for p in participants.iter() {
            let numbers_count = (1..=4).fake::<u8>();
            for _ in 0..numbers_count {
                vid += 1;
                voip_numbers.push(gen_voip_number(vid, p.participant_id, &password_faker));
            }
            let has_number_request = Boolean(10).fake();
            if has_number_request {
                vid += 1;
                let vn = gen_voip_number(vid, None, &password_faker);
                number_requests.push(gen_number_request(
                    vn.number_id.unwrap(),
                    p.participant_id.unwrap(),
                ));
                voip_numbers.push(vn);
            }
        }

        println!("INSERTING voip_numbers");
        insert_with_copy(&cfg, &voip_numbers);
        println!("INSERTING number_requests");
        insert_with_copy(&cfg, &number_requests);

        let price_lists = vec![
            gen_price_list(prid + 1, 49, 30, 60, 20),
            gen_price_list(prid + 2, 420, 10, 1, 1),
            gen_price_list(prid + 3, 421, 15, 60, 1),
            gen_price_list(prid + 4, 48, 20, 60, 10),
            gen_price_list(prid + 5, 43, 35, 60, 20),
        ];

        println!("INSERTING price_list");
        insert_with_copy(&cfg, &price_lists);
        println!("GENERATING cdrs");

        let mut calls: Vec<CallDetailRecord> = Vec::<CallDetailRecord>::with_capacity(calls_count);
        for n in 1..=calls_count {
            let rnd = (0..5).fake::<usize>();
            let rnd_num = (0..voip_numbers.len()).fake::<usize>();
            calls.push(gen_cdr(
                cdrid + (n as u32),
                price_lists[rnd].phone_country_code,
                price_lists[rnd].price_list_id.unwrap(),
                voip_numbers[rnd_num].number.to_string(),
                voip_numbers[rnd_num].number_id.unwrap(),
            ));
        }
        println!("INSERTING cdrs");
        insert_with_copy(&cfg, &calls);
    }

    let mut i_items: Vec<InvoiceItem> = Vec::<InvoiceItem>::with_capacity(10);
    i_items.push(gen_invoice_item(iid + 1, String::from("Calls")));
    i_items.push(gen_invoice_item(iid + 2, String::from("Phone 3CX")));
    i_items.push(gen_invoice_item(iid + 3, String::from("Phone 4G")));
    i_items.push(gen_invoice_item(iid + 4, String::from("Phone 10L")));
    i_items.push(gen_invoice_item(iid + 5, String::from("Phone cable")));
    i_items.push(gen_invoice_item(iid + 6, String::from("Phone 787FU")));

    let mut invoices: Vec<Invoice> = Vec::<Invoice>::with_capacity(contracts_total * 3);
    let mut iih: Vec<InvoiceHasItems> = Vec::<InvoiceHasItems>::with_capacity(contracts_total * 5);

    println!("Generating invoices");
    for c in contracts.iter() {
        let invoices_count = (0..8).fake::<u8>();
        for _ in 1..invoices_count {
            in_num += 1;
            let items_count = (2..4).fake::<u8>();
            let mut total_price = 0f32;
            let mut picked_items: Vec<usize> = vec![];
            for _i in 1..items_count {
                let mut rnd_item: usize;
                loop {
                    rnd_item = (0..6).fake::<usize>();

                    if !picked_items.contains(&rnd_item) {
                        break;
                    }
                }

                picked_items.push(rnd_item);

                total_price += i_items[rnd_item].unit_cost;

                iih.push(InvoiceHasItems::new(
                    in_num.unsigned_abs(),
                    i_items[rnd_item].item_id.unwrap(),
                    i_items[rnd_item].unit_cost,
                    1,
                ));
            }

            invoices.push(gen_invoice(
                in_num.unsigned_abs(),
                total_price,
                c.contract_id.unwrap(),
            ));
        }
    }

    println!("inserting invoice items");
    insert_with_copy(&cfg, &i_items);
    println!("Inserting invoices");
    insert_with_copy(&cfg, &invoices);
    println!("Inserting invoice has items");
    insert_with_copy(&cfg, &iih);
}
