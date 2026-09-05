use claims::assert_err;
use proptest::proptest;
use seeyou_cup::{Distance, Elevation, RunwayDimension};

#[test]
fn test_dimension_unicode_unit_errors() {
    for input in ["°x", "€x", "✈x"] {
        assert_eq!(
            assert_err!(input.parse::<Elevation>()),
            "Invalid elevation unit: 'x'"
        );
        assert_eq!(
            assert_err!(input.parse::<RunwayDimension>()),
            "Invalid runway dimension unit: 'x'"
        );
        assert_eq!(
            assert_err!(input.parse::<Distance>()),
            "Invalid distance unit: 'x'"
        );
    }
}

#[test]
fn test_dimension_proptest() {
    proptest!(|(s in "\\PC*")| {
        let _ = s.parse::<Elevation>();
        let _ = s.parse::<RunwayDimension>();
        let _ = s.parse::<Distance>();
    });
}
