use rust_decimal::Decimal;
use std::fmt;

#[derive(Debug)]
pub struct Contract {
    pub contract_id: Option<u32>,
    pub contract_name: String,
    pub variable_symbol: i32,
    pub identification_number: Option<i32>,
    pub vat_identification_number: Option<String>,
    pub created_at: Option<String>,
    pub deleted_at: Option<String>,
    pub notify_limit: Option<Decimal>,
    pub email: String,
    pub phone_number: String,
    pub bonus_amount: Option<Decimal>,
}

impl Contract {
    pub fn new(
        cn: String,
        vs: i32,
        email: String,
        pn: String,
        bonus: Option<Decimal>,
        notify: Option<Decimal>,
        id: Option<i32>,
        vat_id: Option<String>,
    ) -> Contract {
        Contract {
            contract_id: None,
            contract_name: cn,
            variable_symbol: vs,
            email,
            phone_number: pn,
            created_at: None,
            deleted_at: None,
            bonus_amount: bonus,
            notify_limit: notify,
            identification_number: id,
            vat_identification_number: vat_id,
        }
    }
}

impl fmt::Display for Contract {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt,"{},{},{},{},{},{},{},{}",
            self.variable_symbol,
            self.contract_name,
            self.identification_number.unwrap_or_default(),
            if self.vat_identification_number.is_some() { self.vat_identification_number.as_ref().unwrap() } else { "''" },
            self.notify_limit.unwrap_or_default(),
            self.email,
            self.phone_number,
            self.bonus_amount.unwrap_or_default(),
        )
    }
}

#[derive(Debug)]
pub struct Address {
    address_id: Option<i32>,
    city: String,
    district: Option<String>,
    street_name: String,
    house_number: i32,
    zip_code: i32,
    contract_id: u32,
}

impl Address {
    pub fn new(
        city: String,
        district: Option<String>,
        street_name: String,
        house_number: i32,
        zip_code: i32,
        contract_id: u32,
    ) -> Address {
        Address {
            address_id: None,
            city,
            district,
            street_name,
            house_number,
            zip_code,
            contract_id,
        }
    }
}

pub struct Participant {
    participant_id: Option<u32>,
    name: String,
    access_level: u8,
    contract_id: u32,
    password: String,
    balance_limit: Option<f32>,
    created_at: Option<String>,
    deleted_at: Option<String>,
}

impl Participant {
    pub fn new(
        name: String,
        access_level: u8,
        contract_id: u32,
        password: String,
        balance_limit: Option<f32>,
        created_at: Option<String>,
        deleted_at: Option<String>,
    ) -> Participant {
        Participant {
            participant_id: None,
            name,
            access_level,
            contract_id,
            balance_limit,
            password,
            created_at,
            deleted_at,
        }
    }
}

pub struct VoipNumber {
    number_id: Option<u32>,
    phone_country_code: u16,
    number: u32,
    participant_id: u32,
    password: String,
    current_state: u8,
    foreign_block: bool,
    quarantine_until: Option<String>,
    activated: Option<String>,
    deleted_at: Option<String>,
}

impl VoipNumber {
    pub fn new(
        phone_country_code: u16,
        number: u32,
        participant_id: u32,
        password: String,
        current_state: u8,
        foreign_block: bool,
        quarantine_until: Option<String>,
        activated: Option<String>,
        deleted_at: Option<String>,
    ) -> VoipNumber {
        VoipNumber {
            number_id: None,
            phone_country_code,
            number,
            participant_id,
            password,
            current_state,
            foreign_block,
            quarantine_until,
            activated,
            deleted_at,
        }
    }
}

pub struct NumberRequest {
    participant_id: u32,
    number_id: u32,
    requested: String,
}

impl NumberRequest {
    pub fn new(participant_id: u32, number_id: u32, requested: String) -> NumberRequest {
        NumberRequest {
            participant_id,
            number_id,
            requested,
        }
    }
}

pub struct PriceList {
    price_list_id: Option<u32>,
    tariffication_first: u8,
    tariffication_second: u8,
    price_per_second: u16,
    phone_country_code: u16,
}

impl PriceList {
    pub fn new(
        tariffication_first: u8,
        tariffication_second: u8,
        price_per_second: u16,
        phone_country_code: u16,
    ) -> PriceList {
        PriceList {
            price_list_id: None,
            tariffication_first,
            tariffication_second,
            price_per_second,
            phone_country_code,
        }
    }
}

pub struct CallDetailRecord {
    call_id: Option<i32>,
    disposition: String,
    source_num: String,
    destination_num: String,
    length_num: u16,
    call_date: String,
    number_id: u32,
    incoming_outgoing: bool,
    price_list_id: Option<u32>,
}

pub struct InvoiceItem {
    item_id: Option<u32>,
    item_name: String,
    unit_cost: f32,
}

impl InvoiceItem {
    pub fn new(item_name: String, unit_cost: f32) -> InvoiceItem {
        InvoiceItem {
            item_id: None,
            item_name,
            unit_cost,
        }
    }
}

pub struct Invoice {
    invoice_number: u64,
    amount: f32,
    tax_value_percent: u8,
    created_at: String,
    taxable_period: String,
    maturity: String,
    paid: Option<String>,
    contract_id: u32,
}

impl Invoice {
    pub fn new(
        invoice_number: u64,
        amount: f32,
        tax_value_percent: u8,
        created_at: String,
        taxable_period: String,
        maturity: String,
        paid: Option<String>,
        contract_id: u32,
    ) -> Invoice {
        Invoice {
            invoice_number,
            amount,
            tax_value_percent,
            created_at,
            taxable_period,
            maturity,
            paid,
            contract_id,
        }
    }
}

pub struct InvoiceHasItems {
    invoice_number: u64,
    invoice_item_id: u32,
    item_unit_cost: f32,
    item_count: u16,
}

impl InvoiceHasItems {
    pub fn new(
        invoice_number: u64,
        invoice_item_id: u32,
        item_unit_cost: f32,
        item_count: u16,
    ) -> InvoiceHasItems {
        InvoiceHasItems {
            invoice_number,
            invoice_item_id,
            item_unit_cost,
            item_count,
        }
    }
}
