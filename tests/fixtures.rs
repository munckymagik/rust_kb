extern crate testing_rust;

use testing_rust::*;

#[test]
fn using_local_variables() {
    let restaurant = Restaurant {
        name: "We make Pizza!",
        iban: "GB99ABCD12345612345678"
    };
    let payment = Payment {
        payee: &restaurant,
        amount: 1000,
        currency: "EUR",
    };

    assert_eq!(payment.payee.name(), "We make Pizza!");
    assert_eq!(payment.payee.iban(), "GB99ABCD12345612345678");
    assert_eq!(payment.amount, 1000);
    assert_eq!(payment.currency, "EUR");
}

#[test]
fn shadowing_to_override_variables_context() {
    let restaurant = Restaurant {
        name: "We make Pizza!",
        iban: "GB99ABCD12345612345678"
    };
    let payment = Payment {
        payee: &restaurant,
        amount: 1000,
        currency: "EUR",
    };

    assert_eq!(payment.payee.name(), "We make Pizza!");
    assert_eq!(payment.payee.iban(), "GB99ABCD12345612345678");
    assert_eq!(payment.amount, 1000);
    assert_eq!(payment.currency, "EUR");

    // When the payment is in a different currency
    {
        let payment = Payment {
            payee: &restaurant,
            amount: 2000,
            currency: "GBP",
        };

        assert_eq!(payment.payee.name(), "We make Pizza!");
        assert_eq!(payment.payee.iban(), "GB99ABCD12345612345678");
        assert_eq!(payment.amount, 2000);
        assert_eq!(payment.currency, "GBP");
    }

    // The original variable is back in scope now
    assert_eq!(payment.amount, 1000);
    assert_eq!(payment.currency, "EUR");
}
