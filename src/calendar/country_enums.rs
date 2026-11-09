//! ISO 3166-1 alpha-2 country codes (249 entries) grouped by region.
//!
//! Regions follow the UN geoscheme (M49), with these decisions for edge cases:
//! - Russia -> Europe; Turkey, Cyprus, Armenia, Azerbaijan, Georgia, Kazakhstan -> Asia
//! - IO (British Indian Ocean Territory) and TF (French Southern Territories) -> Africa
//! - BV (Bouvet Island) and GS (South Georgia and the South Sandwich Islands) -> Americas
//! - AQ (Antarctica) and HM (Heard Island and McDonald Islands) -> Oceania

use std::fmt;
use std::str::FromStr;

/// Error returned when parsing an unknown country code.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseCountryCodeError(pub String);

impl fmt::Display for ParseCountryCodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "unknown ISO 3166-1 alpha-2 country code: {:?}", self.0)
    }
}

impl std::error::Error for ParseCountryCodeError {}

/// Generates a region enum together with `ALL`, `as_str()`, `FromStr` and `Display`.
macro_rules! region_enum {
    ($(#[$meta:meta])* $name:ident { $($code:ident),* $(,)? }) => {
        $(#[$meta])*
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
        pub enum $name {
            $($code),*
        }

        impl $name {
            /// All codes of this region.
            pub const ALL: &'static [$name] = &[$($name::$code),*];

            /// The two-letter code as a &str (e.g. "PL").
            pub const fn as_str(&self) -> &'static str {
                match self {
                    $($name::$code => stringify!($code)),*
                }
            }
        }

        impl FromStr for $name {
            type Err = ParseCountryCodeError;

            /// Parses a code, ignoring letter case.
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                let upper = s.trim().to_ascii_uppercase();
                match upper.as_str() {
                    $(stringify!($code) => Ok($name::$code),)*
                    _ => Err(ParseCountryCodeError(s.to_string())),
                }
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str(self.as_str())
            }
        }
    };
}

region_enum! {
    /// Europe (51)
    Europe {
        AD, AL, AT, AX, BA, BE, BG, BY, CH, CZ, DE, DK, EE, ES, FI, FO, FR,
        GB, GG, GI, GR, HR, HU, IE, IM, IS, IT, JE, LI, LT, LU, LV, MC, MD,
        ME, MK, MT, NL, NO, PL, PT, RO, RS, RU, SE, SI, SJ, SK, SM, UA, VA,
    }
}

region_enum! {
    /// Asia (51)
    Asia {
        AE, AF, AM, AZ, BD, BH, BN, BT, CN, CY, GE, HK, ID, IL, IN, IQ, IR,
        JO, JP, KG, KH, KP, KR, KW, KZ, LA, LB, LK, MM, MN, MO, MV, MY, NP,
        OM, PH, PK, PS, QA, SA, SG, SY, TH, TJ, TL, TM, TR, TW, UZ, VN, YE,
    }
}

region_enum! {
    /// Africa (60)
    Africa {
        AO, BF, BI, BJ, BW, CD, CF, CG, CI, CM, CV, DJ, DZ, EG, EH, ER, ET,
        GA, GH, GM, GN, GQ, GW, IO, KE, KM, LR, LS, LY, MA, MG, ML, MR, MU,
        MW, MZ, NA, NE, NG, RE, RW, SC, SD, SH, SL, SN, SO, SS, ST, SZ, TD,
        TF, TG, TN, TZ, UG, YT, ZA, ZM, ZW,
    }
}

region_enum! {
    /// Americas (57)
    Americas {
        AG, AI, AR, AW, BB, BL, BM, BO, BQ, BR, BS, BV, BZ, CA, CL, CO, CR,
        CU, CW, DM, DO, EC, FK, GD, GF, GL, GP, GS, GT, GY, HN, HT, JM, KN,
        KY, LC, MF, MQ, MS, MX, NI, PA, PE, PM, PR, PY, SR, SV, SX, TC, TT,
        US, UY, VC, VE, VG, VI,
    }
}

region_enum! {
    /// Oceania (30)
    Oceania {
        AQ, AS, AU, CC, CK, CX, FJ, FM, GU, HM, KI, MH, MP, NC, NF, NR, NU,
        NZ, PF, PG, PN, PW, SB, TK, TO, TV, UM, VU, WF, WS,
    }
}

/// ISO 3166-1 alpha-2 country code, grouped by region.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CountryCode {
    Europe(Europe),
    Asia(Asia),
    Africa(Africa),
    Americas(Americas),
    Oceania(Oceania),
}

impl CountryCode {
    /// The two-letter code as a &str (e.g. "PL").
    pub const fn as_str(&self) -> &'static str {
        match self {
            CountryCode::Europe(c) => c.as_str(),
            CountryCode::Asia(c) => c.as_str(),
            CountryCode::Africa(c) => c.as_str(),
            CountryCode::Americas(c) => c.as_str(),
            CountryCode::Oceania(c) => c.as_str(),
        }
    }

    /// Iterates over all 249 codes, region by region:
    /// Europe, Asia, Africa, Americas, Oceania.
    pub fn iter() -> impl Iterator<Item = CountryCode> {
        Europe::ALL
            .iter()
            .copied()
            .map(CountryCode::Europe)
            .chain(Asia::ALL.iter().copied().map(CountryCode::Asia))
            .chain(Africa::ALL.iter().copied().map(CountryCode::Africa))
            .chain(Americas::ALL.iter().copied().map(CountryCode::Americas))
            .chain(Oceania::ALL.iter().copied().map(CountryCode::Oceania))
    }
}

impl From<Europe> for CountryCode {
    fn from(c: Europe) -> Self {
        CountryCode::Europe(c)
    }
}

impl From<Asia> for CountryCode {
    fn from(c: Asia) -> Self {
        CountryCode::Asia(c)
    }
}

impl From<Africa> for CountryCode {
    fn from(c: Africa) -> Self {
        CountryCode::Africa(c)
    }
}

impl From<Americas> for CountryCode {
    fn from(c: Americas) -> Self {
        CountryCode::Americas(c)
    }
}

impl From<Oceania> for CountryCode {
    fn from(c: Oceania) -> Self {
        CountryCode::Oceania(c)
    }
}

impl FromStr for CountryCode {
    type Err = ParseCountryCodeError;

    /// Tries each region in turn; letter case is ignored.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse::<Europe>()
            .map(CountryCode::Europe)
            .or_else(|_| s.parse::<Asia>().map(CountryCode::Asia))
            .or_else(|_| s.parse::<Africa>().map(CountryCode::Africa))
            .or_else(|_| s.parse::<Americas>().map(CountryCode::Americas))
            .or_else(|_| s.parse::<Oceania>().map(CountryCode::Oceania))
            .map_err(|_| ParseCountryCodeError(s.to_string()))
    }
}

impl fmt::Display for CountryCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn region_sizes() {
        assert_eq!(Europe::ALL.len(), 51);
        assert_eq!(Asia::ALL.len(), 51);
        assert_eq!(Africa::ALL.len(), 60);
        assert_eq!(Americas::ALL.len(), 57);
        assert_eq!(Oceania::ALL.len(), 30);
    }

    #[test]
    fn total_is_249_and_unique() {
        let all: Vec<&str> = CountryCode::iter().map(|c| c.as_str()).collect();
        assert_eq!(all.len(), 249);
        let unique: HashSet<_> = all.iter().collect();
        assert_eq!(unique.len(), 249);
    }

    #[test]
    fn roundtrip() {
        for c in CountryCode::iter() {
            assert_eq!(c.as_str().parse::<CountryCode>().unwrap(), c);
        }
    }

    #[test]
    fn parse_and_region() {
        assert_eq!("pl".parse::<CountryCode>().unwrap(), CountryCode::Europe(Europe::PL));
        assert_eq!("JP".parse::<CountryCode>().unwrap(), CountryCode::Asia(Asia::JP));
        assert_eq!("za".parse::<CountryCode>().unwrap(), Africa::ZA.into());
        assert_eq!("US".parse::<CountryCode>().unwrap(), Americas::US.into());
        assert_eq!("NZ".parse::<CountryCode>().unwrap(), Oceania::NZ.into());
        assert!("XX".parse::<CountryCode>().is_err());
    }
}