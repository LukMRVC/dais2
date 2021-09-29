use super::*;
use fake::{
    faker::{self},
    Fake, Faker,
};
use rust_decimal::Decimal;

pub fn gen_contract(cid: u32, vs: i32) -> Contract {
    use faker::boolean::en::Boolean;
    use faker::company::en::CompanyName;
    use faker::internet::en::FreeEmail;
    use faker::name::en::Name;
    use faker::number::en::NumberWithFormat;

    let is_company: bool = Boolean(25).fake();
    let has_bonus: bool = Boolean(25).fake();
    // let is_deleted: bool = Boolean(10).fake();
    let has_limit: bool = Boolean(25).fake();
    let name: String = if is_company {
        Name().fake()
    } else {
        CompanyName().fake()
    };
    let bonus: Option<Decimal> = if has_bonus {
        Some(Decimal::from((50..500).fake::<u32>()))
    } else {
        None
    };
    let limit: Option<Decimal> = if has_limit {
        Some(Decimal::from((20..500).fake::<u32>()))
    } else {
        None
    };
    let id: Option<i32> = if is_company {
        Some((111111..9999999).fake::<i32>())
    } else {
        None
    };
    let vat_id: Option<String> = if is_company {
        Some(format!("CZ{}", id.unwrap()))
    } else {
        None
    };

    Contract::new(
        cid,
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

pub fn gen_address(aid: u32, contract_id: u32) -> Address {
    use fake::faker::address::en::{BuildingNumber, CityName, StreetName, ZipCode};
    Address::new(
        aid,
        CityName().fake(),
        None,
        StreetName().fake(),
        BuildingNumber().fake::<String>().parse::<i32>().unwrap(),
        ZipCode().fake::<String>().parse::<i32>().unwrap(),
        contract_id,
    )
}

pub fn gen_participant(
    pid: u32,
    contract_id: u32,
    f: &fake::StringFaker<std::ops::Range<usize>>,
) -> Participant {
    use faker::boolean::en::Boolean;
    use faker::name::en::FirstName;
    let has_limit: bool = Boolean(25).fake();

    Participant::new(
        pid,
        FirstName().fake(),
        (1..=3).fake::<u8>(),
        contract_id,
        f.fake::<String>(),
        if has_limit {
            Some(Decimal::from((10..100).fake::<u32>()))
        } else {
            None
        },
        None,
        None,
    )
}

pub fn gen_voip_number(
    nid: u32,
    pid: Option<u32>,
    f: &fake::StringFaker<std::ops::Range<usize>>,
) -> VoipNumber {
    use chrono::prelude::*;
    use fake::faker::boolean::en::Boolean;
    use fake::faker::chrono::en::DateTimeBetween;
    use fake::faker::number::en::NumberWithFormat;

    let end_dt: DateTime<Utc> = Utc::now();
    let start_dt: DateTime<Utc> = Utc.ymd(2020, 1, 1).and_hms(0, 0, 0);
    let is_in_quarantine = Boolean(20).fake();

    VoipNumber::new(
        Some(nid),
        420,
        NumberWithFormat("5########")
            .fake::<String>()
            .parse::<u32>()
            .unwrap(),
        pid,
        f.fake::<String>(),
        (1..4).fake::<u8>(),
        Boolean(35).fake(),
        if is_in_quarantine {
            Some(DateTimeBetween(start_dt, end_dt).fake())
        } else {
            None
        },
        DateTimeBetween(start_dt, end_dt).fake(),
        None,
    )
}

pub fn gen_price_list(id: u32, pcc: u16, price: u16, t1: u8, t2: u8) -> PriceList {
    PriceList {
        phone_country_code: pcc,
        price_list_id: Some(id),
        price_per_second: price,
        tariffication_first: t1,
        tariffication_second: t2,
    }
}

pub fn gen_cdr(
    id: u32,
    pcc: u16,
    price_list_id: u32,
    number_str: String,
    number_id: u32,
) -> CallDetailRecord {
    use chrono::prelude::*;
    use fake::faker::boolean::en::Boolean;
    use fake::faker::chrono::en::DateTimeBetween;
    use fake::faker::number::en::NumberWithFormat;

    let end_dt: DateTime<Utc> = Utc::now();
    let start_dt: DateTime<Utc> = Utc.ymd(2020, 1, 1).and_hms(0, 0, 0);

    let dispositions: [String; 3] = [
        "HANGUP".to_string(),
        "ANSWER".to_string(),
        "ERROR".to_string(),
    ];
    let disposition_pick = (0..3).fake::<usize>();
    let is_incoming = Boolean(50).fake();
    let mut num1: String;
    let mut num2: String;
    if is_incoming {
        num1 = String::from(format!("+{}", pcc));
        num1 += &NumberWithFormat("#########").fake::<String>();
        num2 = number_str;
    } else {
        num1 = number_str;
        num2 = String::from(format!("+{}", pcc));
        num2 += &NumberWithFormat("#########").fake::<String>();
    }

    CallDetailRecord::new(
        Some(id),
        dispositions[disposition_pick].clone(),
        num1,
        num2,
        (1..300).fake::<u16>(),
        DateTimeBetween(start_dt, end_dt).fake(),
        number_id,
        is_incoming,
        Some(price_list_id),
    )
}

pub fn gen_invoice_item(item_id: u32, item_name: String) -> InvoiceItem {
    InvoiceItem::new(Some(item_id), item_name, Faker.fake::<f32>())
}

pub fn gen_invoice(invoice_number: u64, amount: f32, contract_id: u32) -> Invoice {
    use chrono::prelude::*;
    use fake::faker::chrono::en::DateTimeBetween;
    use faker::boolean::en::Boolean;

    let end_dt: DateTime<Utc> = Utc::now();
    let start_dt: DateTime<Utc> = Utc.ymd(2020, 1, 1).and_hms(0, 0, 0);
    let mut paid = None;
    let is_paid = Boolean(80).fake();
    if is_paid {
        paid = Some(DateTimeBetween(start_dt, end_dt).fake())
    }

    let created_at: chrono::DateTime<Utc> = DateTimeBetween(start_dt, end_dt).fake();
    let maturity = created_at + chrono::Duration::days(14);

    Invoice::new(
        invoice_number,
        amount,
        21,
        created_at.to_rfc3339(),
        created_at.to_rfc3339(),
        maturity.to_rfc3339(),
        paid,
        contract_id,
    )
}

pub fn gen_number_request(num_id: u32, part_id: u32) -> NumberRequest {
    use chrono::prelude::*;
    use fake::faker::chrono::en::DateTimeBetween;
    let end_dt: DateTime<Utc> = Utc::now();
    let start_dt: DateTime<Utc> = Utc.ymd(2020, 1, 1).and_hms(0, 0, 0);
    let requested = DateTimeBetween(start_dt, end_dt).fake();
    NumberRequest::new(part_id, num_id, requested)
}
