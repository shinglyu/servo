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
    // Serialization is not actually specced
    // though these are the values expected by basic-shape
    // https://github.com/w3c/csswg-drafts/issues/368
    let drop_shadow = parse_longhand!(filter, "drop-shadow(1px, 1px, 1px, black)");
    let color = parse(CSSColor::parse, "black").unwrap();
    assert_eq!(drop_shadow, filter::SpecifiedValue(vec![
        filter::SpecifiedFilter::DropShadow(
           Length::Absolute(Au(60)),
           Length::Absolute(Au(60)),
           Length::Absolute(Au(60)),
           Some(color)
        )
    ]));

    // Only keywords can be reordered
    //assert!(parse(Position::parse, "40% left").is_err());
}
