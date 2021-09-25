use fake::{Fake, faker};
mod entities;
use entities::*;
use std::vec::Vec;
use postgres::{Client, NoTls};
use rust_decimal::Decimal;
use std::io::Write;

fn gen_contract(vs: i32) -> Contract {
    use faker::name::en::Name;
    use faker::company::en::CompanyName;
    use faker::internet::en::FreeEmail;
    use faker::number::en::NumberWithFormat;
    use faker::boolean::en::Boolean;

    let is_company: bool = Boolean(25).fake();
    let has_bonus: bool = Boolean(25).fake();
    let has_limit: bool = Boolean(25).fake();
    let name: String = if is_company { Name().fake() } else { CompanyName().fake() };
    let bonus: Option<Decimal> = if has_bonus { Some(Decimal::from((50..500).fake::<u32>())) } else { None };
    let limit: Option<Decimal> = if has_limit { Some(Decimal::from((20..500).fake::<u32>())) } else { None };
    let id: Option<i32> = if is_company { Some((111111..9999999).fake::<i32>()) } else { None };
    let vat_id: Option<String> = if is_company { Some(format!("CZ{}", id.unwrap())) } else { None };

    Contract::new(
        name,
        vs,
        FreeEmail().fake(),
        NumberWithFormat("+420 6## ### ###").fake(),
        bonus,
        limit,
        id,
        vat_id,
    )
}

fn insert_with_inserts(upper_limit: usize, contracts: &Vec<Contract>) {
    let transaction_ops_count = 20000;
    let mut client = Client::connect("host=localhost user=lukas password=lukas", NoTls).expect("Failed joining to postgres");
    let mut i:usize = 0;
    loop {
        let mut transaction = client.transaction().expect("Failed to create new transaction");
        let statement = transaction.prepare("INSERT INTO contract(variable_symbol, contract_name, identification_number, vat_identification_number, \
            notify_limit, email, phone_number, bonus_amount) VALUES \
            ($1, $2, $3, $4, $5, $6, $7, $8)").expect("Failed to create a prepared statement");
        
            for j in 0..=transaction_ops_count {
                transaction.execute(&statement, &[
                    &contracts[i + j].variable_symbol,
                    &contracts[i + j].contract_name,
                    &contracts[i + j].identification_number,
                    &contracts[i + j].vat_identification_number,
                    &contracts[i + j].notify_limit,
                    &contracts[i + j].email,
                    &contracts[i + j].phone_number,
                    &contracts[i + j].bonus_amount,
                ]).expect("Failed to execute prepared statement");
        }

        transaction.commit().expect("Failed to commit values");

        i += transaction_ops_count;
        if i >= upper_limit {
            break;
        }
    }
}

fn insert_with_copy(upper_limit: i32, contracts: &mut Vec<Contract>) {
    let mut client = Client::connect("host=localhost user=lukas password=lukas", NoTls).expect("Failed joining to postgres");
    let mut writer = client.copy_in("COPY contract(variable_symbol, contract_name, identification_number, vat_identification_number, \
        notify_limit, email, phone_number, bonus_amount) FROM stdin DELIMITER ','").expect("Failed to create copy in writer");
    let mut id = 1;
    for c in contracts {
        c.contract_id = Some(id);
        id += 1;
        writer.write_all((c.to_string() + "\n").as_bytes()).expect("Error while writing to STDIN to copy");
    }

    writer.finish().expect("Failed to finish copying");
}

fn main() -> () {
    let mut vs_symbol = 100_000;
    let upper_limit = 100_000;
    let mut contracts: Vec<Contract> = Vec::<Contract>::with_capacity(100_100);
    for _ in 1..=upper_limit {
        vs_symbol += 1;
        contracts.push(gen_contract(vs_symbol));
    }
    insert_with_copy(upper_limit, &mut contracts);
   

    // println!("Faked string: {:#?}", contracts);
}
