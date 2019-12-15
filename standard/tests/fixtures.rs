extern crate testing_rust;

use testing_rust::*;

#[test]
fn using_local_variables() {
    let restaurant = Restaurant {
        name: "We make Pizza!",
        iban: "GB99ABCD12345612345678",
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
        iban: "GB99ABCD12345612345678",
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

#[test]
fn shadowing_to_reassign_without_mutating_value() {
    struct OriginalRestaurant<'a>(&'a Restaurant<'a>);

    let restaurant = Restaurant {
        name: "We make Pizza!",
        iban: "GB99ABCD12345612345678",
    };

    let original_restaurant = OriginalRestaurant(&restaurant);

    assert_eq!(restaurant.name, "We make Pizza!");

    let restaurant = Restaurant {
        name: "We also make Pizza!",
        iban: "GB01ABCD01234501234567",
    };

    assert_ne!(original_restaurant.0, &restaurant);
    assert_eq!(restaurant.name, "We also make Pizza!");
    assert_eq!(original_restaurant.0.name, "We make Pizza!");
}

#[test]
fn shadowing_to_reassign() {
    let v = vec![1, 2];

    let v = v.into_iter().filter(|&n| n < 2).collect::<Vec<_>>();
    assert_eq!(v, &[1]);

    let v = v.into_iter().filter(|&n| n < 1).collect::<Vec<_>>();
    assert_eq!(v, &[]);
}
