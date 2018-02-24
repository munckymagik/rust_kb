pub trait Payable {
    fn name(&self) -> &str;
    fn iban(&self) -> &str;
}

#[derive(Debug, PartialEq)]
pub struct Restaurant<'a> {
    pub name: &'a str,
    pub iban: &'a str
}

impl<'a> Payable for Restaurant<'a> {
    fn name(&self) -> &str { self.name }
    fn iban(&self) -> &str { self.iban }
}

pub struct Payment<'a> {
    pub payee: &'a Payable,
    pub amount: i64,
    pub currency: &'a str,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
