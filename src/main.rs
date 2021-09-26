mod entities;
use fake::StringFaker;
use entities::*;
use entities::generators::*;
use std::vec::Vec;
use postgres::{Client, NoTls};
use std::io::Write;



fn insert_with_copy(upper_limit: i32, contracts: &Vec<Contract>) {
    let mut client = Client::connect("host=localhost user=lukas password=lukas", NoTls).expect("Failed joining to postgres");
    let mut writer = client.copy_in("COPY contract(variable_symbol, contract_name, identification_number, vat_identification_number, \
        notify_limit, email, phone_number, bonus_amount) FROM stdin DELIMITER ','").expect("Failed to create copy in writer");
    for c in contracts {
        writer.write_all((c.to_string() + "\n").as_bytes()).expect("Error while writing to STDIN to copy");
    }

    writer.finish().expect("Failed to finish copying");
}

fn main() -> () {
    let mut vs_symbol = 100_000;
    let upper_limit = 100_000;
    let mut contracts: Vec<Contract> = Vec::<Contract>::with_capacity(100_100);
    // for _ in 1..=upper_limit {
    //     vs_symbol += 1;
    //     contracts.push(gen_contract(vs_symbol));
    // }

    let hex_chars = '0'..'f';
    println!("{:#?}", hex_chars);
    let string_faker = StringFaker::with(
        String::from("0123456789abcdef").into_bytes(),
        64,
    );
    
    gen_participant(25, &string_faker);

    insert_with_copy(upper_limit, &contracts);
}
