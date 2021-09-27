use rust_decimal::Decimal;
use std::fmt;

pub mod generators;

pub trait CommaDelimited {
    fn to_csv(&self) -> String;
}

pub trait SqlInsert {
    fn insert_header() -> String;
}

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
        cid: u32,
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
            contract_id: Some(cid),
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
        write!(
            fmt,
            "{},{},{},{},{},{},{},{}",
            self.variable_symbol,
            self.contract_name,
            self.identification_number.unwrap_or_default(),
            if self.vat_identification_number.is_some() {
                self.vat_identification_number.as_ref().unwrap()
            } else {
                "''"
            },
            self.notify_limit.unwrap_or_default(),
            self.email,
            self.phone_number,
            self.bonus_amount.unwrap_or_default(),
        )
    }
}

impl SqlInsert for Contract {
    fn insert_header() -> String {
        "contract(contract_id, contract_name, variable_symbol, identification_number, vat_identification_number, \
            deleted_at, notify_limit, email, phone_number, bonus_amount)".to_string()
    }
}

impl CommaDelimited for Contract {
    fn to_csv(&self) -> String {
        format!(
            "{id},{name},{vs},{id_number},{vat_id},{del},{not},{email},{pn},{bonus}\n",
            id = if self.contract_id.is_some() {
                self.contract_id.unwrap().to_string()
            } else {
                "nul_val".to_string()
            },
            name = self.contract_name,
            vs = self.variable_symbol,
            id_number = if self.identification_number.is_some() {
                self.identification_number.unwrap().to_string()
            } else {
                "nul_val".to_string()
            },
            vat_id = self
                .vat_identification_number
                .as_ref()
                .unwrap_or(&"nul_val".to_string()),
            del = self.deleted_at.as_ref().unwrap_or(&"nul_val".to_string()),
            not = if self.notify_limit.is_some() {
                self.notify_limit.unwrap().to_string()
            } else {
                "nul_val".to_string()
            },
            email = self.email,
            pn = self.phone_number,
            bonus = if self.bonus_amount.is_some() {
                self.bonus_amount.unwrap().to_string()
            } else {
                "nul_val".to_string()
            },
        )
    }
}

#[derive(Debug)]
pub struct Address {
    address_id: Option<u32>,
    city: String,
    district: Option<String>,
    street_name: String,
    house_number: i32,
    zip_code: i32,
    contract_id: u32,
}

impl Address {
    pub fn new(
        aid: u32,
        city: String,
        district: Option<String>,
        street_name: String,
        house_number: i32,
        zip_code: i32,
        contract_id: u32,
    ) -> Address {
        Address {
            address_id: Some(aid),
            city,
            district,
            street_name,
            house_number,
            zip_code,
            contract_id,
        }
    }
}

impl SqlInsert for Address {
    fn insert_header() -> String {
        "address(address_id, city, district, street_name, house_number, zip_code, contract_id)"
            .to_string()
    }
}

impl CommaDelimited for Address {
    fn to_csv(&self) -> String {
        format!(
            "{id},{city},{dis},{street},{num},{zip},{cid}\n",
            id = if self.address_id.is_some() {
                self.address_id.unwrap().to_string()
            } else {
                "nul_val".to_string()
            },
            city = self.city,
            dis = self.district.as_ref().unwrap_or(&"nul_val".to_string()),
            street = self.street_name,
            num = self.house_number,
            zip = self.zip_code,
            cid = self.contract_id,
        )
    }
}

#[derive(Debug)]
pub struct Participant {
    pub participant_id: Option<u32>,
    pub name: String,
    pub access_level: u8,
    pub contract_id: u32,
    pub password: String,
    pub balance_limit: Option<Decimal>,
    pub created_at: Option<String>,
    pub deleted_at: Option<String>,
}

impl Participant {
    pub fn new(
        pid: u32,
        name: String,
        access_level: u8,
        contract_id: u32,
        password: String,
        balance_limit: Option<Decimal>,
        created_at: Option<String>,
        deleted_at: Option<String>,
    ) -> Participant {
        Participant {
            participant_id: Some(pid),
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

impl SqlInsert for Participant {
    fn insert_header() -> String {
        "participant(participant_id, name, access_level, contract_id, password, balance_limit, deleted_at)".to_string()
    }
}

impl CommaDelimited for Participant {
    fn to_csv(&self) -> String {
        format!(
            "{id},{name},{access},{cid},{pass},{limit},{del}\n",
            id = if self.participant_id.is_some() {
                self.participant_id.unwrap().to_string()
            } else {
                "nul_val".to_string()
            },
            name = self.name,
            access = self.access_level,
            cid = self.contract_id,
            pass = self.password,
            limit = if self.balance_limit.is_some() {
                self.balance_limit.unwrap().to_string()
            } else {
                "nul_val".to_string()
            },
            del = self.deleted_at.as_ref().unwrap_or(&"nul_val".to_string()),
        )
    }
}

#[derive(Debug)]
pub struct VoipNumber {
    pub number_id: Option<u32>,
    phone_country_code: u16,
    pub number: u32,
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
    ) -> VoipNumber {
        VoipNumber {
            number_id,
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

impl SqlInsert for VoipNumber {
    fn insert_header() -> String {
        "voip_number(number_id, phone_country_code, number, participant_id, password, current_state, \
            foreign_block, quarantine_until, activated, deleted_at)".to_string()
    }
}

impl CommaDelimited for VoipNumber {
    fn to_csv(&self) -> String {
        format!(
            "{id},{pcc},{num},{pid},{pass},{state},{block},{quar},{act},{del}\n",
            id = if self.number_id.is_some() {
                self.number_id.unwrap().to_string()
            } else {
                "nul_val".to_string()
            },
            pcc = self.phone_country_code,
            num = self.number,
            pid = self.participant_id,
            pass = self.password,
            state = self.current_state,
            block = self.foreign_block,
            quar = self
                .quarantine_until
                .as_ref()
                .unwrap_or(&"nul_val".to_string()),
            act = self.activated.as_ref().unwrap_or(&"nul_val".to_string()),
            del = self.deleted_at.as_ref().unwrap_or(&"nul_val".to_string()),
        )
    }
}

#[derive(Debug)]
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

impl SqlInsert for NumberRequest {
    fn insert_header() -> String {
        "number_request(participant_id, number_id, requested)".to_string()
    }
}

impl CommaDelimited for NumberRequest {
    fn to_csv(&self) -> String {
        format!(
            "{pid},{nid},{req}\n",
            pid = self.participant_id,
            nid = self.number_id,
            req = self.requested
        )
    }
}

#[derive(Debug)]
pub struct PriceList {
    pub price_list_id: Option<u32>,
    tariffication_first: u8,
    tariffication_second: u8,
    price_per_second: u16,
    pub phone_country_code: u16,
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

impl SqlInsert for PriceList {
    fn insert_header() -> String {
        "price_list(price_list_id, tariffication_first, tariffication_second, price_per_second, phone_country_code)".to_string()
    }
}

impl CommaDelimited for PriceList {
    fn to_csv(&self) -> String {
        format!(
            "{pid},{t1},{t2},{pps},{pcc}\n",
            pid = if self.price_list_id.is_some() {
                self.price_list_id.unwrap().to_string()
            } else {
                "nul_val".to_string()
            },
            t1 = self.tariffication_first,
            t2 = self.tariffication_second,
            pps = self.price_per_second,
            pcc = self.phone_country_code,
        )
    }
}

#[derive(Debug)]
pub struct CallDetailRecord {
    call_id: Option<u32>,
    disposition: String,
    source_num: String,
    destination_num: String,
    length: u16,
    call_date: String,
    number_id: u32,
    incoming_outgoing: bool,
    price_list_id: Option<u32>,
}

impl CallDetailRecord {
    pub fn new(
        call_id: Option<u32>,
        disposition: String,
        source_num: String,
        destination_num: String,
        length: u16,
        call_date: String,
        number_id: u32,
        incoming_outgoing: bool,
        price_list_id: Option<u32>,
    ) -> CallDetailRecord {
        CallDetailRecord {
            call_id,
            disposition,
            source_num,
            destination_num,
            length,
            call_date,
            number_id,
            incoming_outgoing,
            price_list_id
        }
    }
}

impl SqlInsert for CallDetailRecord {
    fn insert_header() -> String {
        "call_detail_record(call_id, disposition, source_num, destination_num, length, \
            call_date, number_id, incoming_outgoing, price_list_id)"
            .to_string()
    }
}

impl CommaDelimited for CallDetailRecord {
    fn to_csv(&self) -> String {
        format!(
            "{cid},{dis},{src},{dst},{len},{date},{nid},{io},{list}\n",
            cid = if self.call_id.is_some() {
                self.call_id.unwrap().to_string()
            } else {
                "nul_val".to_string()
            },
            dis = self.disposition,
            src = self.source_num,
            dst = self.destination_num,
            len = self.length,
            date = self.call_date,
            nid = self.number_id,
            io = self.incoming_outgoing,
            list = if self.price_list_id.is_some() {
                self.price_list_id.unwrap().to_string()
            } else {
                "nul_val".to_string()
            },
        )
    }
}

#[derive(Debug)]
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

impl SqlInsert for InvoiceItem {
    fn insert_header() -> String {
        "invoice_item(item_id, item_name, unit_cost)".to_string()
    }
}

impl CommaDelimited for InvoiceItem {
    fn to_csv(&self) -> String {
        format!(
            "{id},{name},{cost}\n",
            id = if self.item_id.is_some() {
                self.item_id.unwrap().to_string()
            } else {
                "nul_val".to_string()
            },
            name = self.item_name,
            cost = self.unit_cost,
        )
    }
}

#[derive(Debug)]
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

impl SqlInsert for Invoice {
    fn insert_header() -> String {
        "invoice(invoice_number, amount, tax_value_percent, created_at, taxable_period, maturity, paid, contract_id)".to_string()
    }
}

impl CommaDelimited for Invoice {
    fn to_csv(&self) -> String {
        format!(
            "{inum},{amount},{tax},{tax_p},{mat},{paid},{cid}\n",
            inum = self.invoice_number,
            amount = self.amount,
            tax = self.tax_value_percent,
            tax_p = self.taxable_period,
            mat = self.maturity,
            paid = self.paid.as_ref().unwrap_or(&"nul_val".to_string()),
            cid = self.contract_id,
        )
    }
}

#[derive(Debug)]
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

impl SqlInsert for InvoiceHasItems {
    fn insert_header() -> String {
        "invoice_has_items(invoice_number, invoice_item_id, item_unit_cost, item_count)".to_string()
    }
}

impl CommaDelimited for InvoiceHasItems {
    fn to_csv(&self) -> String {
        format!(
            "{inum},{iid},{iuc},{ic}\n",
            inum = self.invoice_number,
            iid = self.invoice_item_id,
            iuc = self.item_unit_cost,
            ic = self.item_count,
        )
    }
}
