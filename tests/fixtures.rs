extern crate testing_rust;

use testing_rust::*;

#[test]
fn using_local_variables() {
    let restaurant = Restaurant {
        name: "We make Pizza!",
    };
    let payment = Payment {
        restaurant: &restaurant,
        amount: 1000,
        currency: "EUR",
    };

    assert_eq!(payment.restaurant, &restaurant);
    assert_eq!(payment.restaurant.name, "We make Pizza!");
    assert_eq!(payment.amount, 1000);
    assert_eq!(payment.currency, "EUR");
}
