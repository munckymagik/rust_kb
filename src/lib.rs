#[derive(Debug, PartialEq)]
pub struct Restaurant<'a> {
    pub name: &'a str,
}

pub struct Payment<'a> {
    pub restaurant: &'a Restaurant<'a>,
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
