// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// calenda-rs: A Rust library for global calendars.
// Copyright (C) 2024 https://github.com/avhz
//
// Dual licensed under Apache 2.0 and MIT.
//
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

// #![forbid(missing_docs)]

use time::macros::date;

pub mod calendar;
pub mod constants;
pub mod countries;
pub mod errors;
pub mod functions;
pub mod holiday;

// /// Macro to create a `Holiday` constant.
// #[macro_export]
// macro_rules! holiday {
//     ($name:literal, $date:expr, $description:literal) => {
//         $crate::holiday::Holiday {
//             name: $name,
//             date: $date,
//             description: Some($description),
//         }
//     };
// }

// holiday!("New Year's Day", date!(2024-01-01), "New Year's Day is the first day of the year, in the Gregorian calendar, and falls exactly one week after the Christmas Day of the previous year.");

// /// Macro to generate holiday constants for a given country.
// ///
// /// It takes a country code and a list of holidays as input.
// macro_rules! impl_holidays_for_country {
//     ($($country_code:ident: $name:literal, $date:literal, $description:literal,)*) => {
//         $(
//             /// $name
//             pub const $country_code: Holiday = Holiday {
//                 name: $alpha_2,
//                 date: $alpha_3,
//                 description: Some($numeric),
//             };
//         )*
//     };
// }

// impl_holidays_for_country! {
//     AU: "Australia Day", date!($2024-01-26), "Australia Day is the official national day of Australia. Celebrated annually on 26 January, it marks the anniversary of the 1788 arrival of the First Fleet of British ships at Port Jackson, New South Wales, and the raising of the Flag of Great Britain at Sydney Cove by Governor Arthur Phillip.",
//     AU: "Good Friday", date!($2024-04-19), "Good Friday is a Christian holiday commemorating the crucifixion of Jesus and his death at Calvary. It is observed during Holy Week as part of the Paschal Triduum on the Friday preceding Easter Sunday, and may coincide with the Jewish observance of Passover.",
//     AU: "Easter Monday", date!(2024-04-22), "Easter Monday is the day after Easter Sunday and is a holiday in some countries. Easter Monday in the Western Christian liturgical calendar is the second day of Eastertide and analogously in the Byzantine Rite is the second day of Bright Week.",
//     AU: "ANZAC Day", date!(2024-04-25), "Anzac Day is a national day of remembrance in Australia and New Zealand that broadly commemorates all Australians and New Zealanders who served and died in all wars, conflicts, and peacekeeping operations and the contribution and suffering of all those who have served.",
//     AU: "Queen's Birthday", date!(2024-06-10), "The Queen's Official Birthday, or the King's Official Birthday, is the selected day in some Commonwealth realms on which the birthday of the monarch is officially celebrated in those countries.",
//     AU: "Bank Holiday", date!(2024-08-05), "A bank holiday is a public holiday in the United Kingdom, some Commonwealth countries, Hong Kong and the Republic of Ireland. There is no automatic right to time off on these days, but banks close and the majority of the working population is granted time off work or extra pay for working on these days, depending on their contract.",
//     AU: "Labour Day", date!(2024-10-07), "Labour Day is an annual holiday to celebrate the achievements of workers. Labour Day has its origins in the labour union movement, specifically the eight-hour day movement, which advocated eight hours for work, eight hours for recreation, and eight hours for rest.",
//     AU: "Christmas Day", date!(2024-12-25), "Christmas is an annual festival commemorating the birth of Jesus Christ, observed primarily on December 25 as a religious and cultural celebration among billions of people around the world.",
// }

// macro_rules! impl_holidays_for_country {
//     ($country_code:ident, $start_year:literal, $end_year:literal, $($name:literal, $month:literal, $day:literal, $description:literal,)*) => {
//         $(
//             $(
//                 /// $name for year $current_year
//                 pub const $country_code: Holiday = Holiday {
//                     name: $name,
//                     date: date!($current_year-$month-$day),
//                     description: Some($description),
//                 };
//             )*
//         )*
//     };
// }

// impl_holidays_for_country! {
//     AU, 2022, 2024,
//     "Christmas Day", 12, 25, "Christmas is an annual festival commemorating the birth of Jesus Christ, observed primarily on December 25 as a religious and cultural celebration among billions of people around the world.",
//     // Add more holidays for other years as needed
// }
