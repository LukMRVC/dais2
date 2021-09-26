use fake::{Fake, faker};
use rust_decimal::Decimal;
use super::{Contract, Address, Participant};

pub fn gen_contract(cid: u32, vs: i32) -> Contract {
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
    use fake::faker::address::en::{CityName, StreetName, ZipCode, BuildingNumber};
    
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

pub fn gen_participant(pid: u32, contract_id: u32, f: &fake::StringFaker<std::ops::Range<usize>>) -> Participant {
    use faker::name::en::FirstName;
    use faker::boolean::en::Boolean;
    let has_limit: bool = Boolean(25).fake();

    Participant::new(
        pid,
        FirstName().fake(),
        (1..=3).fake::<u8>(),
        contract_id,
        f.fake::<String>(),
        if has_limit { Some(Decimal::from( (10..100).fake::<u32>())) } else { None },
        None,
        None,
    )
}