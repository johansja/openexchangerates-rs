extern crate hyper;
extern crate rustc_serialize;
extern crate chrono;

#[cfg(test)]
#[macro_use]
extern crate yup_hyper_mock as hyper_mock;

use std::collections::BTreeMap;
use std::error;
use std::fmt;
use std::io::{self, Read};

use rustc_serialize::json;
use chrono::*;

#[derive(Debug)]
pub enum Error {
    Hyper(hyper::Error),
    Io(io::Error),
    Decode(json::DecoderError),
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Hyper(ref err) => err.description(),
            Error::Io(ref err) => err.description(),
            Error::Decode(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Error::Hyper(ref err) => Some(err),
            Error::Io(ref err) => Some(err),
            Error::Decode(ref err) => Some(err),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Hyper(ref err) => write!(f, "Hyper error: {}", err),
            Error::Io(ref err) => write!(f, "IO error: {}", err),
            Error::Decode(ref err) => write!(f, "Decode error: {}", err),
        }
    }
}

impl From<hyper::Error> for Error {
    fn from(err: hyper::Error) -> Error {
        Error::Hyper(err)
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::Io(err)
    }
}

impl From<json::DecoderError> for Error {
    fn from(err: json::DecoderError) -> Error {
        Error::Decode(err)
    }
}

#[derive(RustcDecodable, Debug)]
pub struct ExchangeRate {
    disclaimer: String,
    license: String,
    timestamp: i64,
    base: String,
    rates: BTreeMap<String, f32>,
}

pub type Currencies = BTreeMap<String, String>;

pub struct Client {
    app_id: &'static str,
    hc: hyper::Client,
}

impl Client {
    pub fn new(app_id: &'static str) -> Client {
        Client {
            app_id: app_id,
            hc: hyper::Client::new(),
        }
    }

    pub fn latest(self) -> Result<ExchangeRate, Error> {
        let url = &format!("https://openexchangerates.org/api/latest.json?app_id={}",
                           self.app_id);
        let mut res = try!(self.hc.get(url).send());

        let mut body = String::new();
        try!(res.read_to_string(&mut body));

        let decoded: ExchangeRate = try!(json::decode(&body));
        Ok(decoded)
    }

    pub fn currencies(self) -> Result<Currencies, Error> {
        let url = &format!("https://openexchangerates.org/api/currencies.json?app_id={}",
                           self.app_id);
        let mut res = try!(self.hc.get(url).send());

        let mut body = String::new();
        try!(res.read_to_string(&mut body));

        let decoded: Currencies = try!(json::decode(&body));
        Ok(decoded)
    }

    pub fn historical(self, date: date::Date<UTC>) -> Result<ExchangeRate, Error> {
        let url = &format!("https://openexchangerates.org/api/historical/{}.json?app_id={}",
                           date.format("%Y-%m-%d"),
                           self.app_id);
        let mut res = try!(self.hc.get(url).send());

        let mut body = String::new();
        try!(res.read_to_string(&mut body));

        let decoded: ExchangeRate = try!(json::decode(&body));
        Ok(decoded)
    }
}

#[cfg(test)]
mod tests {
    use chrono::*;
    use hyper;

    use super::Client;

    #[test]
    fn new_client() {
        let app_id = "1234";
        let client = Client::new(app_id);
        assert_eq!(client.app_id, app_id);
    }

    mock_connector!(LatestConnector {
        "https://openexchangerates.org" =>
r###"HTTP/1.1 200 OK
Date: Sat, 02 Apr 2016 10:17:23 GMT
Server: Apache
Last-Modified: Sat, 02 Apr 2016 10:00:08 GMT
Cache-Control: public
ETag: "9b382c0cfef957d53f96d1fe7630adfe"
Access-Control-Allow-Origin: *
Content-Length: 4213
Connection: close
Content-Type: application/json; charset=utf-8

{
  "disclaimer": "Exchange rates provided for informational purposes only and do not constitute financial advice of any kind. Although every attempt is made to ensure quality, no guarantees are made of accuracy, validity, availability, or fitness for any purpose. All usage subject to acceptance of Terms: https://openexchangerates.org/terms/",
  "license": "Data sourced from various providers; resale prohibited; no warranties given of any kind. All usage subject to License Agreement: https://openexchangerates.org/license/",
  "timestamp": 1459591208,
  "base": "USD",
  "rates": {
    "AED": 3.673042,
    "AFN": 68.660002,
    "ALL": 121.9671,
    "AMD": 480.950003,
    "ANG": 1.7888,
    "AOA": 160.589498,
    "ARS": 14.77299,
    "AUD": 1.30258,
    "AWG": 1.793333,
    "AZN": 1.521,
    "BAM": 1.717244,
    "BBD": 2,
    "BDT": 78.368321,
    "BGN": 1.717111,
    "BHD": 0.376972,
    "BIF": 1555.4425,
    "BMD": 1,
    "BND": 1.349318,
    "BOB": 6.820635,
    "BRL": 3.559709,
    "BSD": 1,
    "BTC": 0.002385058088,
    "BTN": 66.267185,
    "BWP": 10.852188,
    "BYR": 20051.525,
    "BZD": 1.994946,
    "CAD": 1.301511,
    "CDF": 928.6645,
    "CHF": 0.958473,
    "CLF": 0.024598,
    "CLP": 669.031001,
    "CNY": 6.479217,
    "COP": 3035.536706,
    "CRC": 535.190095,
    "CUC": 1,
    "CUP": 1.000025,
    "CVE": 97.5979,
    "CZK": 23.73241,
    "DJF": 177.684751,
    "DKK": 6.540791,
    "DOP": 45.73138,
    "DZD": 108.22848,
    "EEK": 13.73835,
    "EGP": 8.878437,
    "ERN": 15.0015,
    "ETB": 21.52748,
    "EUR": 0.877796,
    "FJD": 2.067533,
    "FKP": 0.702794,
    "GBP": 0.702794,
    "GEL": 2.299625,
    "GGP": 0.702794,
    "GHS": 3.843708,
    "GIP": 0.702794,
    "GMD": 42.71179,
    "GNF": 7598.317598,
    "GTQ": 7.714015,
    "GYD": 206.358002,
    "HKD": 7.754301,
    "HNL": 22.60846,
    "HRK": 6.593482,
    "HTG": 62.047737,
    "HUF": 275.320401,
    "IDR": 13163.266667,
    "ILS": 3.77435,
    "IMP": 0.702794,
    "INR": 66.256,
    "IQD": 1088.549988,
    "IRR": 30222.5,
    "ISK": 123.2991,
    "JEP": 0.702794,
    "JMD": 121.6909,
    "JOD": 0.709178,
    "JPY": 111.778101,
    "KES": 101.564929,
    "KGS": 69.8588,
    "KHR": 4002.549976,
    "KMF": 432.220398,
    "KPW": 900.09,
    "KRW": 1150.008332,
    "KWD": 0.301913,
    "KYD": 0.824377,
    "KZT": 342.59609,
    "LAK": 8122.772598,
    "LBP": 1507.818341,
    "LKR": 146.457101,
    "LRD": 84.66847,
    "LSL": 14.717975,
    "LTL": 3.018716,
    "LVL": 0.618233,
    "LYD": 1.360674,
    "MAD": 9.633868,
    "MDL": 19.54001,
    "MGA": 3187.408317,
    "MKD": 54.29857,
    "MMK": 1214.565025,
    "MNT": 2047.5,
    "MOP": 7.987331,
    "MRO": 342.900667,
    "MTL": 0.683602,
    "MUR": 35.222513,
    "MVR": 15.22,
    "MWK": 679.567751,
    "MXN": 17.3395,
    "MYR": 3.892578,
    "MZN": 50.545001,
    "NAD": 14.72483,
    "NGN": 199.0619,
    "NIO": 28.22777,
    "NOK": 8.310984,
    "NPR": 105.9966,
    "NZD": 1.448348,
    "OMR": 0.385144,
    "PAB": 1,
    "PEN": 3.345842,
    "PGK": 3.0894,
    "PHP": 46.04603,
    "PKR": 104.7395,
    "PLN": 3.72937,
    "PYG": 5643.96666,
    "QAR": 3.641214,
    "RON": 3.923577,
    "RSD": 107.77516,
    "RUB": 67.67019,
    "RWF": 759.490752,
    "SAR": 3.750389,
    "SBD": 7.937802,
    "SCR": 13.339688,
    "SDG": 6.099201,
    "SEK": 8.134106,
    "SGD": 1.350273,
    "SHP": 0.702794,
    "SLL": 3994.5,
    "SOS": 613.442122,
    "SRD": 5.097,
    "STD": 21519.25,
    "SVC": 8.744603,
    "SYP": 219.582332,
    "SZL": 14.72038,
    "THB": 35.16262,
    "TJS": 7.8696,
    "TMT": 3.501633,
    "TND": 2.006912,
    "TOP": 2.241557,
    "TRY": 2.822329,
    "TTD": 6.570187,
    "TWD": 32.29852,
    "TZS": 2186.86335,
    "UAH": 26.10184,
    "UGX": 3369.931667,
    "USD": 1,
    "UYU": 31.86962,
    "UZS": 2880.88501,
    "VEF": 9.983559,
    "VND": 22305.35,
    "VUV": 107.815,
    "WST": 2.506777,
    "XAF": 576.288951,
    "XAG": 0.0664455,
    "XAU": 0.000819,
    "XCD": 2.70102,
    "XDR": 0.709879,
    "XOF": 580.687711,
    "XPD": 0.00178,
    "XPF": 104.963937,
    "XPT": 0.001049,
    "YER": 215.020999,
    "ZAR": 14.70453,
    "ZMK": 5253.075255,
    "ZMW": 10.879925,
    "ZWL": 322.387247
  }
}"###
    });

    #[test]
    fn latest_works() {
        let client = Client {
            app_id: "1234",
            hc: hyper::Client::with_connector(LatestConnector::default()),
        };

        let res = client.latest();
        assert!(res.is_ok());

        let rate = res.unwrap();
        assert!(!rate.disclaimer.is_empty());
        assert!(!rate.license.is_empty());
        assert!(rate.timestamp != 0);
        assert_eq!(rate.base, "USD");
        assert_eq!(rate.rates.len(), 171);
        assert_eq!(rate.rates.get("MYR"), Some(&3.892578_f32));
    }

    mock_connector!(CurrenciesConnector {
        "https://openexchangerates.org" =>
r###"HTTP/1.1 200 OK
Date: Mon, 04 Apr 2016 12:17:13 GMT
Server: Apache
Last-Modified: Tue, 30 Jun 2015 12:20:33 GMT
Cache-Control: public
ETag: "b3eb21df82ca046c024091278c757705"
Access-Control-Allow-Origin: *
Content-Length: 4912
Connection: close
Content-Type: application/json; charset=utf-8

{
  "AED": "United Arab Emirates Dirham",
  "AFN": "Afghan Afghani",
  "ALL": "Albanian Lek",
  "AMD": "Armenian Dram",
  "ANG": "Netherlands Antillean Guilder",
  "AOA": "Angolan Kwanza",
  "ARS": "Argentine Peso",
  "AUD": "Australian Dollar",
  "AWG": "Aruban Florin",
  "AZN": "Azerbaijani Manat",
  "BAM": "Bosnia-Herzegovina Convertible Mark",
  "BBD": "Barbadian Dollar",
  "BDT": "Bangladeshi Taka",
  "BGN": "Bulgarian Lev",
  "BHD": "Bahraini Dinar",
  "BIF": "Burundian Franc",
  "BMD": "Bermudan Dollar",
  "BND": "Brunei Dollar",
  "BOB": "Bolivian Boliviano",
  "BRL": "Brazilian Real",
  "BSD": "Bahamian Dollar",
  "BTC": "Bitcoin",
  "BTN": "Bhutanese Ngultrum",
  "BWP": "Botswanan Pula",
  "BYR": "Belarusian Ruble",
  "BZD": "Belize Dollar",
  "CAD": "Canadian Dollar",
  "CDF": "Congolese Franc",
  "CHF": "Swiss Franc",
  "CLF": "Chilean Unit of Account (UF)",
  "CLP": "Chilean Peso",
  "CNY": "Chinese Yuan",
  "COP": "Colombian Peso",
  "CRC": "Costa Rican Col\u00f3n",
  "CUC": "Cuban Convertible Peso",
  "CUP": "Cuban Peso",
  "CVE": "Cape Verdean Escudo",
  "CZK": "Czech Republic Koruna",
  "DJF": "Djiboutian Franc",
  "DKK": "Danish Krone",
  "DOP": "Dominican Peso",
  "DZD": "Algerian Dinar",
  "EEK": "Estonian Kroon",
  "EGP": "Egyptian Pound",
  "ERN": "Eritrean Nakfa",
  "ETB": "Ethiopian Birr",
  "EUR": "Euro",
  "FJD": "Fijian Dollar",
  "FKP": "Falkland Islands Pound",
  "GBP": "British Pound Sterling",
  "GEL": "Georgian Lari",
  "GGP": "Guernsey Pound",
  "GHS": "Ghanaian Cedi",
  "GIP": "Gibraltar Pound",
  "GMD": "Gambian Dalasi",
  "GNF": "Guinean Franc",
  "GTQ": "Guatemalan Quetzal",
  "GYD": "Guyanaese Dollar",
  "HKD": "Hong Kong Dollar",
  "HNL": "Honduran Lempira",
  "HRK": "Croatian Kuna",
  "HTG": "Haitian Gourde",
  "HUF": "Hungarian Forint",
  "IDR": "Indonesian Rupiah",
  "ILS": "Israeli New Sheqel",
  "IMP": "Manx pound",
  "INR": "Indian Rupee",
  "IQD": "Iraqi Dinar",
  "IRR": "Iranian Rial",
  "ISK": "Icelandic Kr\u00f3na",
  "JEP": "Jersey Pound",
  "JMD": "Jamaican Dollar",
  "JOD": "Jordanian Dinar",
  "JPY": "Japanese Yen",
  "KES": "Kenyan Shilling",
  "KGS": "Kyrgystani Som",
  "KHR": "Cambodian Riel",
  "KMF": "Comorian Franc",
  "KPW": "North Korean Won",
  "KRW": "South Korean Won",
  "KWD": "Kuwaiti Dinar",
  "KYD": "Cayman Islands Dollar",
  "KZT": "Kazakhstani Tenge",
  "LAK": "Laotian Kip",
  "LBP": "Lebanese Pound",
  "LKR": "Sri Lankan Rupee",
  "LRD": "Liberian Dollar",
  "LSL": "Lesotho Loti",
  "LTL": "Lithuanian Litas",
  "LVL": "Latvian Lats",
  "LYD": "Libyan Dinar",
  "MAD": "Moroccan Dirham",
  "MDL": "Moldovan Leu",
  "MGA": "Malagasy Ariary",
  "MKD": "Macedonian Denar",
  "MMK": "Myanma Kyat",
  "MNT": "Mongolian Tugrik",
  "MOP": "Macanese Pataca",
  "MRO": "Mauritanian Ouguiya",
  "MTL": "Maltese Lira",
  "MUR": "Mauritian Rupee",
  "MVR": "Maldivian Rufiyaa",
  "MWK": "Malawian Kwacha",
  "MXN": "Mexican Peso",
  "MYR": "Malaysian Ringgit",
  "MZN": "Mozambican Metical",
  "NAD": "Namibian Dollar",
  "NGN": "Nigerian Naira",
  "NIO": "Nicaraguan C\u00f3rdoba",
  "NOK": "Norwegian Krone",
  "NPR": "Nepalese Rupee",
  "NZD": "New Zealand Dollar",
  "OMR": "Omani Rial",
  "PAB": "Panamanian Balboa",
  "PEN": "Peruvian Nuevo Sol",
  "PGK": "Papua New Guinean Kina",
  "PHP": "Philippine Peso",
  "PKR": "Pakistani Rupee",
  "PLN": "Polish Zloty",
  "PYG": "Paraguayan Guarani",
  "QAR": "Qatari Rial",
  "RON": "Romanian Leu",
  "RSD": "Serbian Dinar",
  "RUB": "Russian Ruble",
  "RWF": "Rwandan Franc",
  "SAR": "Saudi Riyal",
  "SBD": "Solomon Islands Dollar",
  "SCR": "Seychellois Rupee",
  "SDG": "Sudanese Pound",
  "SEK": "Swedish Krona",
  "SGD": "Singapore Dollar",
  "SHP": "Saint Helena Pound",
  "SLL": "Sierra Leonean Leone",
  "SOS": "Somali Shilling",
  "SRD": "Surinamese Dollar",
  "STD": "S\u00e3o Tom\u00e9 and Pr\u00edncipe Dobra",
  "SVC": "Salvadoran Col\u00f3n",
  "SYP": "Syrian Pound",
  "SZL": "Swazi Lilangeni",
  "THB": "Thai Baht",
  "TJS": "Tajikistani Somoni",
  "TMT": "Turkmenistani Manat",
  "TND": "Tunisian Dinar",
  "TOP": "Tongan Pa\u02bbanga",
  "TRY": "Turkish Lira",
  "TTD": "Trinidad and Tobago Dollar",
  "TWD": "New Taiwan Dollar",
  "TZS": "Tanzanian Shilling",
  "UAH": "Ukrainian Hryvnia",
  "UGX": "Ugandan Shilling",
  "USD": "United States Dollar",
  "UYU": "Uruguayan Peso",
  "UZS": "Uzbekistan Som",
  "VEF": "Venezuelan Bol\u00edvar Fuerte",
  "VND": "Vietnamese Dong",
  "VUV": "Vanuatu Vatu",
  "WST": "Samoan Tala",
  "XAF": "CFA Franc BEAC",
  "XAG": "Silver (troy ounce)",
  "XAU": "Gold (troy ounce)",
  "XCD": "East Caribbean Dollar",
  "XDR": "Special Drawing Rights",
  "XOF": "CFA Franc BCEAO",
  "XPD": "Palladium Ounce",
  "XPF": "CFP Franc",
  "XPT": "Platinum Ounce",
  "YER": "Yemeni Rial",
  "ZAR": "South African Rand",
  "ZMK": "Zambian Kwacha (pre-2013)",
  "ZMW": "Zambian Kwacha",
  "ZWL": "Zimbabwean Dollar"
}
"###
    });

    #[test]
    fn currencies_works() {
        let client = Client {
            app_id: "1234",
            hc: hyper::Client::with_connector(CurrenciesConnector::default()),
        };

        let res = client.currencies();
        assert!(res.is_ok());

        let currencies = res.unwrap();
        assert_eq!(currencies.len(), 171);
        assert!(currencies.contains_key("MYR"));
        assert_eq!(currencies.get("MYR"),
                   Some(&"Malaysian Ringgit".to_string()));
    }


    mock_connector!(HistoricalConnector {
        "https://openexchangerates.org" =>
r###"HTTP/1.1 200 OK
Date: Mon, 04 Apr 2016 13:00:47 GMT
Server: Apache
Last-Modified: Sat, 16 Feb 2013 23:00:00 GMT
Cache-Control: public
ETag: "528b3c8a4aa18bc29071d7824f285c5d"
Access-Control-Allow-Origin: *
Content-Length: 4189
Connection: close
Content-Type: application/json; charset=utf-8

{
  "disclaimer": "Exchange rates are provided for informational purposes only, and do not constitute financial advice of any kind. Although every attempt is made to ensure quality, NO guarantees are given whatsoever of accuracy, validity, availability, or fitness for any purpose - please use at your own risk. All usage is subject to your acceptance of the Terms and Conditions of Service, available at: http://openexchangerates.org/terms/",
  "license": "Data sourced from various providers with public-facing APIs; copyright may apply; resale is prohibited; no warranties given of any kind. All usage is subject to your acceptance of the License Agreement available at: http://openexchangerates.org/license/",
  "timestamp": 1361055600,
  "base": "USD",
  "rates": {
    "AED": 3.672869,
    "AFN": 51.769999,
    "ALL": 104.798751,
    "AMD": 406.549996,
    "ANG": 1.7887,
    "AOA": 95.946132,
    "ARS": 5.009655,
    "AUD": 0.969258,
    "AWG": 1.789967,
    "AZN": 0.7847,
    "BAM": 1.465127,
    "BBD": 2,
    "BDT": 78.95455,
    "BGN": 1.465315,
    "BHD": 0.376979,
    "BIF": 1573.6325,
    "BMD": 1,
    "BND": 1.235676,
    "BOB": 6.984993,
    "BRL": 1.963769,
    "BSD": 1,
    "BTC": 0.036871,
    "BTN": 54.149416,
    "BWP": 7.960765,
    "BYR": 8650.918333,
    "BZD": 2.018223,
    "CAD": 1.004479,
    "CDF": 918.967757,
    "CHF": 0.921763,
    "CLF": 0.02122,
    "CLP": 471.190578,
    "CNY": 6.243627,
    "COP": 1784.424248,
    "CRC": 500.270201,
    "CUP": 22.687419,
    "CVE": 82.724563,
    "CZK": 19.006606,
    "DJF": 177.153124,
    "DKK": 5.582097,
    "DOP": 40.789208,
    "DZD": 78.29744,
    "EEK": 11.7331,
    "EGP": 6.728837,
    "ETB": 18.3724,
    "EUR": 0.748104,
    "FJD": 1.770065,
    "FKP": 0.644426,
    "GBP": 0.644426,
    "GEL": 1.65605,
    "GHS": 1.899084,
    "GIP": 0.635095,
    "GMD": 32.96705,
    "GNF": 7074.0775,
    "GTQ": 7.824749,
    "GYD": 202.634999,
    "HKD": 7.754526,
    "HNL": 19.917224,
    "HRK": 5.675358,
    "HTG": 42.493713,
    "HUF": 218.905721,
    "IDR": 9676.719572,
    "ILS": 3.688885,
    "INR": 54.211281,
    "IQD": 1162.80125,
    "IRR": 12269.0975,
    "ISK": 128.82375,
    "JEP": 0.644426,
    "JMD": 94.631907,
    "JOD": 0.70857,
    "JPY": 93.242464,
    "KES": 87.44366,
    "KGS": 47.821867,
    "KHR": 3992.76375,
    "KMF": 368.414177,
    "KPW": 900,
    "KRW": 1079.374741,
    "KWD": 0.282232,
    "KYD": 0.822275,
    "KZT": 150.384562,
    "LAK": 7901.858701,
    "LBP": 1505.93588,
    "LKR": 126.66026,
    "LRD": 74.25,
    "LSL": 8.826443,
    "LTL": 2.583805,
    "LVL": 0.523754,
    "LYD": 1.259093,
    "MAD": 8.361509,
    "MDL": 12.099609,
    "MGA": 2198.315,
    "MKD": 47.086109,
    "MMK": 857.898,
    "MNT": 1387.5,
    "MOP": 7.980274,
    "MRO": 298.60675,
    "MUR": 30.658802,
    "MVR": 15.3975,
    "MWK": 361.842376,
    "MXN": 12.687136,
    "MYR": 3.094163,
    "MZN": 30.55,
    "NAD": 8.81998,
    "NGN": 157.245031,
    "NIO": 24.397725,
    "NOK": 5.542418,
    "NPR": 86.563924,
    "NZD": 1.181559,
    "OMR": 0.384974,
    "PAB": 1,
    "PEN": 2.567481,
    "PGK": 2.0499,
    "PHP": 40.612969,
    "PKR": 98.067139,
    "PLN": 3.133177,
    "PYG": 4053.87125,
    "QAR": 3.640933,
    "RON": 3.281269,
    "RSD": 83.334276,
    "RUB": 30.117065,
    "RWF": 622.261922,
    "SAR": 3.750351,
    "SBD": 7.07897,
    "SCR": 12.691349,
    "SDG": 4.41191,
    "SEK": 6.321318,
    "SGD": 1.236699,
    "SHP": 0.644426,
    "SLL": 4317.211208,
    "SOS": 1598.59,
    "SRD": 3.28125,
    "STD": 18397.45,
    "SVC": 8.741834,
    "SYP": 70.817901,
    "SZL": 8.815176,
    "THB": 29.872007,
    "TJS": 4.7545,
    "TMT": 2.85065,
    "TND": 1.557717,
    "TOP": 1.727115,
    "TRY": 1.767556,
    "TTD": 6.390928,
    "TWD": 29.629389,
    "TZS": 1616.365596,
    "UAH": 8.115192,
    "UGX": 2637.08879,
    "USD": 1,
    "UYU": 18.998253,
    "UZS": 2006.519989,
    "VEF": 4.295129,
    "VND": 20824.31866,
    "VUV": 90,
    "WST": 2.273163,
    "XAF": 491.357387,
    "XCD": 2.701275,
    "XDR": 0.65495,
    "XOF": 491.51925,
    "XPF": 89.529112,
    "YER": 214.756258,
    "ZAR": 8.832957,
    "ZMK": 5232.196666,
    "ZWL": 322.387247
  }
}"###
    });

    #[test]
    fn historical_works() {
        let client = Client {
            app_id: "1234",
            hc: hyper::Client::with_connector(HistoricalConnector::default()),
        };

        let res = client.historical(UTC.ymd(2013, 2, 16));
        assert!(res.is_ok());

        let rate = res.unwrap();
        assert!(!rate.disclaimer.is_empty());
        assert!(!rate.license.is_empty());
        assert!(rate.timestamp != 0);
        assert_eq!(rate.base, "USD");
        assert_eq!(rate.rates.len(), 161);
        assert_eq!(rate.rates.get("MYR"), Some(&3.094163_f32));
    }
}
