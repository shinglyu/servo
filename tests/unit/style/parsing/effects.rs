/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use app_units::Au;
use cssparser::Parser;
use media_queries::CSSErrorReporterTest;
use style::parser::ParserContext;
use style::stylesheets::Origin;
use url::Url;
use parsing::parse;

#[test]
fn test_drop_shadow_filter() {
    use style::properties::longhands::filter;
    use style::values::specified::Length;
    use style::values::specified::CSSColor;

    // Fully-specified
    let expected_color = parse(CSSColor::parse, "black").unwrap();
    let expected = filter::SpecifiedValue(vec![
        filter::SpecifiedFilter::DropShadow(
           Length::Absolute(Au(60)),
           Length::Absolute(Au(120)),
           Length::Absolute(Au(180)),
           Some(expected_color)
        )
    ]);

    let drop_shadow = parse_longhand!(filter, "drop-shadow(1px 2px 3px black)");
    assert_eq!(drop_shadow, expected);

    // No color
    let expected = filter::SpecifiedValue(vec![
        filter::SpecifiedFilter::DropShadow(
           Length::Absolute(Au(60)),
           Length::Absolute(Au(120)),
           Length::Absolute(Au(180)),
           None
        )
    ]);
    let drop_shadow = parse_longhand!(filter, "drop-shadow(1px 2px 3px)");
    assert_eq!(drop_shadow, expected);

    // No blur radius and color
    let expected = filter::SpecifiedValue(vec![
        filter::SpecifiedFilter::DropShadow(
           Length::Absolute(Au(60)),
           Length::Absolute(Au(120)),
           Length::Absolute(Au(0)),
           None
        )
    ]);
    let drop_shadow = parse_longhand!(filter, "drop-shadow(1px 2px)");
    assert_eq!(drop_shadow, expected);

    // No blur radius but has color
    let expected_color = parse(CSSColor::parse, "red").unwrap();
    let expected = filter::SpecifiedValue(vec![
        filter::SpecifiedFilter::DropShadow(
           Length::Absolute(Au(60)),
           Length::Absolute(Au(120)),
           Length::Absolute(Au(0)),
           Some(expected_color)
        )
    ]);
    let drop_shadow = parse_longhand!(filter, "drop-shadow(1px 2px red)");
    assert_eq!(drop_shadow, expected);

    assert_parse_longhand_will_fail!(filter, "drop-shadow(1px)"); // offset x & y are required
    assert_parse_longhand_will_fail!(filter, "drop-shadow(1px 2px 3px 4px)"); // spread radius
    assert_parse_longhand_will_fail!(filter, "drop-shadow(1px 2px red 4px)"); // wrong order
}
