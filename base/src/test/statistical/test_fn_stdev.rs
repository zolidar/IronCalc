#![allow(clippy::unwrap_used)]

use crate::test::util::new_empty_model;

#[test]
fn smoke_test() {
    let mut model = new_empty_model();
    model._set("A1", "=STDEV.P(10, 12, 23, 23, 16, 23, 21)");
    model._set("A2", "=STDEV.S(10, 12, 23, 23, 16, 23, 21)");
    model.evaluate();

    assert_eq!(model._get_text("A1"), *"5.174505793");

    assert_eq!(model._get_text("A2"), *"5.589105048");
}

#[test]
fn numbers() {
    let mut model = new_empty_model();

    model._set("A2", "24");
    model._set("A3", "25");
    model._set("A4", "27");
    model._set("A5", "23");
    model._set("A6", "45");
    model._set("A7", "23.5");
    model._set("A8", "34");
    model._set("A9", "23");
    model._set("A10", "23");
    model._set("A11", "TRUE");
    model._set("A12", "'23");
    model._set("A13", "Text");
    model._set("A14", "FALSE");
    model._set("A15", "45");

    model._set("B1", "=STDEV.P(A2:A15)");
    model._set("B2", "=STDEV.S(A2:A15)");
    model._set("B3", "=STDEVA(A2:A15)");
    model._set("B4", "=STDEVPA(A2:A15)");
    model.evaluate();

    assert_eq!(model._get_text("B1"), *"8.483071378");
    assert_eq!(model._get_text("B2"), *"8.941942369");
    assert_eq!(model._get_text("B3"), *"15.499955689");
    assert_eq!(model._get_text("B4"), *"14.936131032");
}

const SPREAD_FUNCTIONS: [&str; 13] = [
    "VAR.P", "VAR.S", "VARA", "VARPA", "VAR", "VARP", "STDEV.P", "STDEV.S", "STDEVA", "STDEVPA",
    "STDEV", "STDEVP", "DEVSQ",
];

#[test]
fn equal_values_give_exactly_zero() {
    for value in [
        "14580.73",
        "123456.789",
        "4567.891",
        "0.1",
        "1",
        "5",
        "0.3",
        "-98765432.1",
    ] {
        let mut model = new_empty_model();
        model._set("A1", value);
        model._set("A2", value);
        model._set("A3", value);
        for (i, name) in SPREAD_FUNCTIONS.iter().enumerate() {
            let row = i + 1;
            model._set(&format!("B{row}"), &format!("={name}(A1:A3)"));
            model._set(&format!("C{row}"), &format!("=B{row}=0"));
            model._set(
                &format!("D{row}"),
                &format!("={name}({value}, {value}, {value})"),
            );
            model._set(&format!("E{row}"), &format!("=D{row}=0"));
        }
        model.evaluate();
        for (i, name) in SPREAD_FUNCTIONS.iter().enumerate() {
            let row = i + 1;
            assert_eq!(
                model._get_text(&format!("B{row}")),
                "0",
                "{name} of {value}"
            );
            assert_eq!(
                model._get_text(&format!("C{row}")),
                "TRUE",
                "{name} of {value}"
            );
            assert_eq!(
                model._get_text(&format!("E{row}")),
                "TRUE",
                "{name} of {value}"
            );
        }
    }
}

#[test]
fn large_magnitude_near_equal_values() {
    let mut model = new_empty_model();
    model._set("A1", "=VAR.S(1000000.1, 1000000.2, 1000000.3)");
    model._set("A2", "=STDEV.P(1000000.1, 1000000.2, 1000000.3)");
    model._set("A3", "=DEVSQ(1000000.1, 1000000.2, 1000000.3)");
    model.evaluate();

    assert_eq!(model._get_text("A1"), *"0.01");
    assert_eq!(model._get_text("A2"), *"0.081649658");
    assert_eq!(model._get_text("A3"), *"0.02");
}
