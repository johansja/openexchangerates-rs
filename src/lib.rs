//! A library for accessing OpenExchangeRates API.

#[cfg_attr(feature = "serde_macros", feature(plugin, custom_derive))]
#[cfg_attr(feature = "serde_macros", plugin(serde_macros))]

extern crate chrono;
extern crate hyper;
extern crate serde;
extern crate serde_json;

#[cfg(test)]
#[macro_use]
extern crate yup_hyper_mock;

use std::borrow::Cow;
use std::collections::BTreeMap;
use std::io::Read;

use chrono::*;

pub mod error;

#[cfg(feature = "serde_macros")]
include!("serde_types.in.rs");

#[cfg(feature = "serde_codegen")]
include!(concat!(env!("OUT_DIR"), "/serde_types.rs"));

pub struct Client<'a> {
    app_id: Cow<'a, str>,
    hc: hyper::Client,
}

impl<'a> Client<'a> {
    /// Create a new client that is ready to interact with the API.
    pub fn new<S>(app_id: S) -> Client<'a>
        where S: Into<Cow<'a, str>>
    {
        Client {
            app_id: app_id.into(),
            hc: hyper::Client::new(),
        }
    }

    /// Get the latest exchange rates.
    ///
    /// The corresponding endpoint in OpenExchangeRates is documented in [here](https://docs.openexchangerates.org/docs/latest-json).
    pub fn latest(self) -> Result<ExchangeRate, error::Error> {
        let url = &format!("https://openexchangerates.org/api/latest.json?app_id={}",
                           self.app_id);
        let mut res = try!(self.hc.get(url).send());

        let mut body = String::new();
        try!(res.read_to_string(&mut body));

        let deserialized: ExchangeRate = try!(serde_json::from_str(&body));
        Ok(deserialized)
    }

    /// Get a list of supported currencies.
    ///
    /// The corresponding endpoint in OpenExchangeRates is documented in [here](https://docs.openexchangerates.org/docs/currencies-json).
    pub fn currencies(self) -> Result<Currencies, error::Error> {
        let url = &format!("https://openexchangerates.org/api/currencies.json?app_id={}",
                           self.app_id);
        let mut res = try!(self.hc.get(url).send());

        let mut body = String::new();
        try!(res.read_to_string(&mut body));

        let deserialized: Currencies = try!(serde_json::from_str(&body));
        Ok(deserialized)
    }

    /// Get the exchange rate for a particular date.
    ///
    /// The corresponding endpoint in OpenExchangeRates is documented in [here](https://docs.openexchangerates.org/docs/historical-json).
    pub fn historical(self, date: NaiveDate) -> Result<ExchangeRate, error::Error> {
        let url = &format!("https://openexchangerates.org/api/historical/{}.json?app_id={}",
                           date.format("%Y-%m-%d"),
                           self.app_id);
        let mut res = try!(self.hc.get(url).send());

        let mut body = String::new();
        try!(res.read_to_string(&mut body));

        let deserialized: ExchangeRate = try!(serde_json::from_str(&body));
        Ok(deserialized)
    }

    /// Get statistics about your App ID.
    ///
    /// The corresponding endpoint in OpenExchangeRates is documented in [here](https://docs.openexchangerates.org/docs/usage-json).
    pub fn usage(self) -> Result<Usage, error::Error> {
        let url = &format!("https://openexchangerates.org/api/usage.json?app_id={}",
                           self.app_id);
        let mut res = try!(self.hc.get(url).send());

        let mut body = String::new();
        try!(res.read_to_string(&mut body));

        let deserialized: Usage = try!(serde_json::from_str(&body));
        Ok(deserialized)
    }
}

#[cfg(test)]
mod tests {
    use std::borrow::Cow;

    use chrono::*;
    use hyper;

    use super::*;

    #[test]
    fn new_client() {
        let app_id = "1234";
        let client = Client::new(app_id);
        assert_eq!(client.app_id, app_id);
    }

    #[test]
    fn new_client_string() {
        let app_id = "1234".to_owned();
        Client::new(app_id);
    }

    mock_connector!(LatestConnector {
        "https://openexchangerates.org" =>
r###"HTTP/1.1 200 OK
Date: Tue, 05 Apr 2016 11:34:18 GMT
Server: Apache
Last-Modified: Tue, 05 Apr 2016 11:00:02 GMT
Cache-Control: public
ETag: "686eef0d2c12ae0108d2310cccb414a6"
Access-Control-Allow-Origin: *
Content-Length: 4206
Connection: close
Content-Type: application/json; charset=utf-8

{
  "disclaimer": "Exchange rates provided for informational purposes only and do not constitute financial advice of any kind. Although every attempt is made to ensure quality, no guarantees are made of accuracy, validity, availability, or fitness for any purpose. All usage subject to acceptance of Terms: https://openexchangerates.org/terms/",
  "license": "Data sourced from various providers; resale prohibited; no warranties given of any kind. All usage subject to License Agreement: https://openexchangerates.org/license/",
  "timestamp": 1459854002,
  "base": "USD",
  "rates": {
    "AED": 3.673125,
    "AFN": 68.589998,
    "ALL": 121.9326,
    "AMD": 481.667501,
    "ANG": 1.788775,
    "AOA": 160.587502,
    "ARS": 14.72719,
    "AUD": 1.32684,
    "AWG": 1.793333,
    "AZN": 1.512738,
    "BAM": 1.721123,
    "BBD": 2,
    "BDT": 78.21121,
    "BGN": 1.720664,
    "BHD": 0.377016,
    "BIF": 1561.969988,
    "BMD": 1,
    "BND": 1.354349,
    "BOB": 6.814022,
    "BRL": 3.612869,
    "BSD": 1,
    "BTC": 0.0023746257,
    "BTN": 66.387817,
    "BWP": 10.864388,
    "BYR": 20174.55,
    "BZD": 1.994083,
    "CAD": 1.31303,
    "CDF": 927.915,
    "CHF": 0.958776,
    "CLF": 0.024602,
    "CLP": 670.372097,
    "CNY": 6.475759,
    "COP": 3051.173333,
    "CRC": 534.406195,
    "CUC": 1,
    "CUP": 0.9993,
    "CVE": 96.813399,
    "CZK": 23.79069,
    "DJF": 177.627877,
    "DKK": 6.54783,
    "DOP": 45.70438,
    "DZD": 108.262899,
    "EEK": 13.7566,
    "EGP": 8.876027,
    "ERN": 15.0015,
    "ETB": 21.45419,
    "EUR": 0.880403,
    "FJD": 2.075617,
    "FKP": 0.703584,
    "GBP": 0.703584,
    "GEL": 2.286425,
    "GGP": 0.703584,
    "GHS": 3.848313,
    "GIP": 0.703584,
    "GMD": 42.81891,
    "GNF": 7577.457451,
    "GTQ": 7.70903,
    "GYD": 206.160169,
    "HKD": 7.756089,
    "HNL": 22.60784,
    "HRK": 6.61159,
    "HTG": 61.952012,
    "HUF": 275.380602,
    "IDR": 13207,
    "ILS": 3.797811,
    "IMP": 0.703584,
    "INR": 66.41976,
    "IQD": 1088.549988,
    "IRR": 30224.5,
    "ISK": 123.5441,
    "JEP": 0.703584,
    "JMD": 121.6279,
    "JOD": 0.709282,
    "JPY": 110.5841,
    "KES": 101.4874,
    "KGS": 69,
    "KHR": 4014.802525,
    "KMF": 432.933426,
    "KPW": 899.91,
    "KRW": 1154.609982,
    "KWD": 0.301793,
    "KYD": 0.82461,
    "KZT": 343.50969,
    "LAK": 8111.537451,
    "LBP": 1510.758341,
    "LKR": 145.115399,
    "LRD": 84.651538,
    "LSL": 14.931688,
    "LTL": 3.020352,
    "LVL": 0.618811,
    "LYD": 1.367899,
    "MAD": 9.644687,
    "MDL": 19.54317,
    "MGA": 3188.638333,
    "MKD": 54.2471,
    "MMK": 1206.867488,
    "MNT": 2042.333333,
    "MOP": 7.985845,
    "MRO": 342.718667,
    "MTL": 0.683602,
    "MUR": 35.103338,
    "MVR": 15.076667,
    "MWK": 680.282423,
    "MXN": 17.56133,
    "MYR": 3.917198,
    "MZN": 50.450001,
    "NAD": 14.88925,
    "NGN": 198.744101,
    "NIO": 28.21809,
    "NOK": 8.349952,
    "NPR": 106.1939,
    "NZD": 1.476865,
    "OMR": 0.385002,
    "PAB": 1,
    "PEN": 3.34308,
    "PGK": 3.1142,
    "PHP": 46.22462,
    "PKR": 104.6345,
    "PLN": 3.733942,
    "PYG": 5643.435026,
    "QAR": 3.641405,
    "RON": 3.930383,
    "RSD": 107.9216,
    "RUB": 68.85775,
    "RWF": 759.751002,
    "SAR": 3.750091,
    "SBD": 7.937802,
    "SCR": 13.4225,
    "SDG": 6.096855,
    "SEK": 8.148025,
    "SGD": 1.356264,
    "SHP": 0.703584,
    "SLL": 3991,
    "SOS": 606.250753,
    "SRD": 5.097,
    "STD": 21505.05,
    "SVC": 8.740228,
    "SYP": 219.552332,
    "SZL": 14.89155,
    "THB": 35.28028,
    "TJS": 7.8696,
    "TMT": 3.501533,
    "TND": 2.010213,
    "TOP": 2.232678,
    "TRY": 2.829322,
    "TTD": 6.573825,
    "TWD": 32.39399,
    "TZS": 2186.119967,
    "UAH": 26.04258,
    "UGX": 3366.955,
    "USD": 1,
    "UYU": 31.66288,
    "UZS": 2883.63501,
    "VEF": 9.972852,
    "VND": 22306.566667,
    "VUV": 107.785,
    "WST": 2.520399,
    "XAF": 577.467725,
    "XAG": 0.0666625,
    "XAU": 0.0008105,
    "XCD": 2.70102,
    "XDR": 0.710543,
    "XOF": 579.402525,
    "XPD": 0.001801,
    "XPF": 104.935788,
    "XPT": 0.001061,
    "YER": 215.126,
    "ZAR": 14.93348,
    "ZMK": 5253.075255,
    "ZMW": 10.762725,
    "ZWL": 322.387247
  }
}"###
    });

    #[test]
    fn latest_works() {
        let client = Client {
            app_id: Cow::Borrowed("1234"),
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
        assert_eq!(rate.rates.get("MYR"), Some(&3.917198_f32));
    }

    mock_connector!(CurrenciesConnector {
        "https://openexchangerates.org" =>
r###"HTTP/1.1 200 OK
Date: Tue, 05 Apr 2016 11:33:16 GMT
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
}"###
    });

    #[test]
    fn currencies_works() {
        let client = Client {
            app_id: Cow::Borrowed("1234"),
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
Date: Tue, 05 Apr 2016 11:31:42 GMT
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
            app_id: Cow::Borrowed("1234"),
            hc: hyper::Client::with_connector(HistoricalConnector::default()),
        };

        let res = client.historical(NaiveDate::from_ymd(2013, 2, 16));
        assert!(res.is_ok());

        let rate = res.unwrap();
        assert!(!rate.disclaimer.is_empty());
        assert!(!rate.license.is_empty());
        assert!(rate.timestamp != 0);
        assert_eq!(rate.base, "USD");
        assert_eq!(rate.rates.len(), 161);
        assert_eq!(rate.rates.get("MYR"), Some(&3.094163_f32));
    }

    mock_connector!(UsageConnector {
        "https://openexchangerates.org" =>
r###"HTTP/1.1 200 OK
Date: Tue, 05 Apr 2016 11:35:19 GMT
Server: Apache
Access-Control-Allow-Origin: *
Content-Length: 555
Connection: close
Content-Type: application/json; charset=utf-8

{
  "status": 200,
  "data": {
    "app_id": "1234",
    "status": "active",
    "plan": {
      "name": "Forever Free",
      "quota": "1,000 requests/month",
      "update_frequency": "hourly",
      "features": {
        "base": false,
        "symbols": false,
        "experimental": true,
        "time-series": false,
        "convert": false
      }
    },
    "usage": {
      "requests": 11,
      "requests_quota": 1000,
      "requests_remaining": 989,
      "days_elapsed": 10,
      "days_remaining": 20,
      "daily_average": 1
    }
  }
}"###
    });

    #[test]
    fn usage_works() {
        let client = Client {
            app_id: Cow::Borrowed("1234"),
            hc: hyper::Client::with_connector(UsageConnector::default()),
        };

        let res = client.usage();
        assert!(res.is_ok());

        let usage = res.unwrap();
        assert_eq!(usage,
                   Usage {
                       status: 200,
                       data: UsageData {
                           app_id: "1234".to_string(),
                           status: "active".to_string(),
                           plan: UsageDataPlan {
                               name: "Forever Free".to_string(),
                               quota: "1,000 requests/month".to_string(),
                               update_frequency: "hourly".to_string(),
                               features: Features {
                                   base: false,
                                   symbols: false,
                                   experimental: true,
                                   time_series: false,
                                   convert: false,
                               },
                           },
                           usage: UsageDataUsage {
                               requests: 11,
                               requests_quota: 1000,
                               requests_remaining: 989,
                               days_elapsed: 10,
                               days_remaining: 20,
                               daily_average: 1,
                           },
                       },
                   });
    }
}
