use super::*;
use fake::{faker, Fake};
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
    pid: u32,
    f: &fake::StringFaker<std::ops::Range<usize>>,
) -> VoipNumber {
    use chrono::prelude::*;
    use fake::faker::boolean::en::Boolean;
    use fake::faker::chrono::en::DateTimeBetween;
    use fake::faker::number::en::NumberWithFormat;

    let end_dt: DateTime<Utc> = Utc::now();
    let start_dt: DateTime<Utc> = Utc.ymd(2018, 1, 1).and_hms(0, 0, 0);
    let is_in_quarantine = Boolean(20).fake();

    VoipNumber {
        number_id: Some(nid),
        phone_country_code: 420,
        number: NumberWithFormat("5########")
            .fake::<String>()
            .parse::<u32>()
            .unwrap(),
        participant_id: pid,
        password: f.fake::<String>(),
        current_state: (1..4).fake::<u8>(),
        foreign_block: Boolean(35).fake(),
        quarantine_until: if is_in_quarantine {
            Some(DateTimeBetween(start_dt, end_dt).fake())
        } else {
            None
        },
        activated: DateTimeBetween(start_dt, end_dt).fake(),
        deleted_at: None,
    }
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
