extern crate hyper;
extern crate rustc_serialize;
extern crate chrono;

#[cfg(test)]
#[macro_use]
extern crate yup_hyper_mock as hyper_mock;

use std::cmp::PartialEq;
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

#[derive(RustcDecodable, Debug, PartialEq)]
pub struct Usage {
    status: u32,
    data: UsageData,
}

#[derive(RustcDecodable, Debug, PartialEq)]
pub struct UsageData {
    app_id: String,
    status: String,
    plan: UsageDataPlan,
    usage: UsageDataUsage,
}

#[derive(RustcDecodable, Debug, PartialEq)]
pub struct UsageDataPlan {
    name: String,
    quota: String,
    update_frequency: String,
    features: BTreeMap<String, bool>,
}

#[derive(RustcDecodable, Debug, PartialEq)]
pub struct UsageDataUsage {
    requests: i64,
    requests_quota: i64,
    requests_remaining: i64,
    days_elapsed: i64,
    days_remaining: i64,
    daily_average: i64,
}

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
        let url = &format!("https://openexchangerates.org/api/latest.\
                            json?app_id={}&prettyprint=true",
                           self.app_id);
        let mut res = try!(self.hc.get(url).send());

        let mut body = String::new();
        try!(res.read_to_string(&mut body));

        let decoded: ExchangeRate = try!(json::decode(&body));
        Ok(decoded)
    }

    pub fn currencies(self) -> Result<Currencies, Error> {
        let url = &format!("https://openexchangerates.org/api/currencies.\
                            json?app_id={}&prettyprint=true",
                           self.app_id);
        let mut res = try!(self.hc.get(url).send());

        let mut body = String::new();
        try!(res.read_to_string(&mut body));

        let decoded: Currencies = try!(json::decode(&body));
        Ok(decoded)
    }

    pub fn historical(self, date: date::Date<UTC>) -> Result<ExchangeRate, Error> {
        let url = &format!("https://openexchangerates.org/api/historical/{}.\
                            json?app_id={}&prettyprint=true",
                           date.format("%Y-%m-%d"),
                           self.app_id);
        let mut res = try!(self.hc.get(url).send());

        let mut body = String::new();
        try!(res.read_to_string(&mut body));

        let decoded: ExchangeRate = try!(json::decode(&body));
        Ok(decoded)
    }

    pub fn usage(self) -> Result<Usage, Error> {
        let url = &format!("https://openexchangerates.org/api/usage.\
                            json?app_id={}&prettyprint=true",
                           self.app_id);
        let mut res = try!(self.hc.get(url).send());

        let mut body = String::new();
        try!(res.read_to_string(&mut body));

        let decoded: Usage = try!(json::decode(&body));
        Ok(decoded)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use chrono::*;
    use hyper;

    use super::*;

    #[test]
    fn new_client() {
        let app_id = "1234";
        let client = Client::new(app_id);
        assert_eq!(client.app_id, app_id);
    }

    mock_connector!(LatestConnector {
        "https://openexchangerates.org" =>
r###"HTTP/1.1 200 OK
Date: Mon, 04 Apr 2016 13:54:34 GMT
Server: Apache
Last-Modified: Mon, 04 Apr 2016 13:00:12 GMT
Cache-Control: public
ETag: "e0d7fa02111ead144533a31a5c53bb43"
Access-Control-Allow-Origin: *
Content-Length: 3133
Connection: close
Content-Type: application/json; charset=utf-8

{"disclaimer":"Exchange rates provided for informational purposes only and do not constitute financial advice of any kind. Although every attempt is made to ensure quality, no guarantees are made of accuracy, validity, availability, or fitness for any purpose. All usage subject to acceptance of Terms: https://openexchangerates.org/terms/","license":"Data sourced from various providers; resale prohibited; no warranties given of any kind. All usage subject to License Agreement: https://openexchangerates.org/license/","timestamp":1459774812,"base":"USD","rates":{"AED":3.67305,"AFN":68.640002,"ALL":122.0919,"AMD":481.202503,"ANG":1.788775,"AOA":160.586164,"ARS":14.78841,"AUD":1.3097,"AWG":1.793333,"AZN":1.518325,"BAM":1.718531,"BBD":2,"BDT":78.317689,"BGN":1.717851,"BHD":0.377048,"BIF":1561.099976,"BMD":1,"BND":1.350513,"BOB":6.8198,"BRL":3.555329,"BSD":1,"BTC":0.002392859707,"BTN":66.248567,"BWP":10.851038,"BYR":20107.5,"BZD":1.994988,"CAD":1.303381,"CDF":928.5,"CHF":0.958443,"CLF":0.024602,"CLP":669.313001,"CNY":6.479638,"COP":3036.733366,"CRC":535.401,"CUC":1,"CUP":0.999875,"CVE":97.264067,"CZK":23.74416,"DJF":177.686251,"DKK":6.536585,"DOP":45.7518,"DZD":108.232379,"EEK":13.73955,"EGP":8.858182,"ERN":15.0015,"ETB":21.4908,"EUR":0.877919,"FJD":2.068333,"FKP":0.699395,"GBP":0.699395,"GEL":2.28145,"GGP":0.699395,"GHS":3.84584,"GIP":0.699395,"GMD":42.69638,"GNF":7589,"GTQ":7.71235,"GYD":206.251669,"HKD":7.754978,"HNL":22.6101,"HRK":6.593234,"HTG":62.0206,"HUF":274.439502,"IDR":13155.3,"ILS":3.781952,"IMP":0.699395,"INR":66.12196,"IQD":1088.549988,"IRR":30227,"ISK":123.136,"JEP":0.699395,"JMD":121.693,"JOD":0.70889,"JPY":111.5253,"KES":101.519,"KGS":69.7246,"KHR":3986.425049,"KMF":432.524048,"KPW":900.09,"KRW":1147.501681,"KWD":0.301754,"KYD":0.824517,"KZT":343.737988,"LAK":8121.074951,"LBP":1510.166667,"LKR":145.369001,"LRD":84.66847,"LSL":14.695775,"LTL":3.018756,"LVL":0.618265,"LYD":1.363988,"MAD":9.628296,"MDL":19.54693,"MGA":3188.399984,"MKD":54.20388,"MMK":1216.300025,"MNT":2045.333333,"MOP":7.98779,"MRO":342.97,"MTL":0.683602,"MUR":35.129875,"MVR":15.223333,"MWK":684.289998,"MXN":17.34651,"MYR":3.89525,"MZN":50.545001,"NAD":14.70674,"NGN":198.878001,"NIO":28.20582,"NOK":8.296672,"NPR":105.9814,"NZD":1.458052,"OMR":0.385034,"PAB":1,"PEN":3.34268,"PGK":3.1177,"PHP":46.16022,"PKR":104.697001,"PLN":3.721743,"PYG":5636.368327,"QAR":3.641636,"RON":3.919929,"RSD":107.70816,"RUB":67.83302,"RWF":759.118752,"SAR":3.750577,"SBD":7.936214,"SCR":13.37976,"SDG":6.099213,"SEK":8.123661,"SGD":1.350553,"SHP":0.699395,"SLL":3994.5,"SOS":614.682494,"SRD":5.097,"STD":21565.25,"SVC":8.744588,"SYP":219.575665,"SZL":14.70882,"THB":35.20092,"TJS":7.8696,"TMT":3.501533,"TND":2.00699,"TOP":2.228814,"TRY":2.810007,"TTD":6.57526,"TWD":32.31278,"TZS":2187.566699,"UAH":26.13856,"UGX":3369.516667,"USD":1,"UYU":31.77574,"UZS":2880.88501,"VEF":9.9816,"VND":22339,"VUV":107.815,"WST":2.50899,"XAF":576.745237,"XAG":0.0665405,"XAU":0.000819,"XCD":2.70102,"XDR":0.710468,"XOF":580.296177,"XPD":0.00178,"XPF":104.989638,"XPT":0.001049,"YER":214.926,"ZAR":14.67131,"ZMK":5253.075255,"ZMW":10.893125,"ZWL":322.387247}}
"###
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
        assert_eq!(rate.rates.get("MYR"), Some(&3.89525_f32));
    }

    mock_connector!(CurrenciesConnector {
        "https://openexchangerates.org" =>
r###"HTTP/1.1 200 OK
Date: Mon, 04 Apr 2016 13:50:52 GMT
Server: Apache
Last-Modified: Tue, 30 Jun 2015 12:20:33 GMT
Cache-Control: public
ETag: "906ffb418b536a61b39ebfb3f1703534"
Access-Control-Allow-Origin: *
Content-Length: 4227
Connection: close
Content-Type: application/json; charset=utf-8

{"AED":"United Arab Emirates Dirham","AFN":"Afghan Afghani","ALL":"Albanian Lek","AMD":"Armenian Dram","ANG":"Netherlands Antillean Guilder","AOA":"Angolan Kwanza","ARS":"Argentine Peso","AUD":"Australian Dollar","AWG":"Aruban Florin","AZN":"Azerbaijani Manat","BAM":"Bosnia-Herzegovina Convertible Mark","BBD":"Barbadian Dollar","BDT":"Bangladeshi Taka","BGN":"Bulgarian Lev","BHD":"Bahraini Dinar","BIF":"Burundian Franc","BMD":"Bermudan Dollar","BND":"Brunei Dollar","BOB":"Bolivian Boliviano","BRL":"Brazilian Real","BSD":"Bahamian Dollar","BTC":"Bitcoin","BTN":"Bhutanese Ngultrum","BWP":"Botswanan Pula","BYR":"Belarusian Ruble","BZD":"Belize Dollar","CAD":"Canadian Dollar","CDF":"Congolese Franc","CHF":"Swiss Franc","CLF":"Chilean Unit of Account (UF)","CLP":"Chilean Peso","CNY":"Chinese Yuan","COP":"Colombian Peso","CRC":"Costa Rican Col\u00f3n","CUC":"Cuban Convertible Peso","CUP":"Cuban Peso","CVE":"Cape Verdean Escudo","CZK":"Czech Republic Koruna","DJF":"Djiboutian Franc","DKK":"Danish Krone","DOP":"Dominican Peso","DZD":"Algerian Dinar","EEK":"Estonian Kroon","EGP":"Egyptian Pound","ERN":"Eritrean Nakfa","ETB":"Ethiopian Birr","EUR":"Euro","FJD":"Fijian Dollar","FKP":"Falkland Islands Pound","GBP":"British Pound Sterling","GEL":"Georgian Lari","GGP":"Guernsey Pound","GHS":"Ghanaian Cedi","GIP":"Gibraltar Pound","GMD":"Gambian Dalasi","GNF":"Guinean Franc","GTQ":"Guatemalan Quetzal","GYD":"Guyanaese Dollar","HKD":"Hong Kong Dollar","HNL":"Honduran Lempira","HRK":"Croatian Kuna","HTG":"Haitian Gourde","HUF":"Hungarian Forint","IDR":"Indonesian Rupiah","ILS":"Israeli New Sheqel","IMP":"Manx pound","INR":"Indian Rupee","IQD":"Iraqi Dinar","IRR":"Iranian Rial","ISK":"Icelandic Kr\u00f3na","JEP":"Jersey Pound","JMD":"Jamaican Dollar","JOD":"Jordanian Dinar","JPY":"Japanese Yen","KES":"Kenyan Shilling","KGS":"Kyrgystani Som","KHR":"Cambodian Riel","KMF":"Comorian Franc","KPW":"North Korean Won","KRW":"South Korean Won","KWD":"Kuwaiti Dinar","KYD":"Cayman Islands Dollar","KZT":"Kazakhstani Tenge","LAK":"Laotian Kip","LBP":"Lebanese Pound","LKR":"Sri Lankan Rupee","LRD":"Liberian Dollar","LSL":"Lesotho Loti","LTL":"Lithuanian Litas","LVL":"Latvian Lats","LYD":"Libyan Dinar","MAD":"Moroccan Dirham","MDL":"Moldovan Leu","MGA":"Malagasy Ariary","MKD":"Macedonian Denar","MMK":"Myanma Kyat","MNT":"Mongolian Tugrik","MOP":"Macanese Pataca","MRO":"Mauritanian Ouguiya","MTL":"Maltese Lira","MUR":"Mauritian Rupee","MVR":"Maldivian Rufiyaa","MWK":"Malawian Kwacha","MXN":"Mexican Peso","MYR":"Malaysian Ringgit","MZN":"Mozambican Metical","NAD":"Namibian Dollar","NGN":"Nigerian Naira","NIO":"Nicaraguan C\u00f3rdoba","NOK":"Norwegian Krone","NPR":"Nepalese Rupee","NZD":"New Zealand Dollar","OMR":"Omani Rial","PAB":"Panamanian Balboa","PEN":"Peruvian Nuevo Sol","PGK":"Papua New Guinean Kina","PHP":"Philippine Peso","PKR":"Pakistani Rupee","PLN":"Polish Zloty","PYG":"Paraguayan Guarani","QAR":"Qatari Rial","RON":"Romanian Leu","RSD":"Serbian Dinar","RUB":"Russian Ruble","RWF":"Rwandan Franc","SAR":"Saudi Riyal","SBD":"Solomon Islands Dollar","SCR":"Seychellois Rupee","SDG":"Sudanese Pound","SEK":"Swedish Krona","SGD":"Singapore Dollar","SHP":"Saint Helena Pound","SLL":"Sierra Leonean Leone","SOS":"Somali Shilling","SRD":"Surinamese Dollar","STD":"S\u00e3o Tom\u00e9 and Pr\u00edncipe Dobra","SVC":"Salvadoran Col\u00f3n","SYP":"Syrian Pound","SZL":"Swazi Lilangeni","THB":"Thai Baht","TJS":"Tajikistani Somoni","TMT":"Turkmenistani Manat","TND":"Tunisian Dinar","TOP":"Tongan Pa\u02bbanga","TRY":"Turkish Lira","TTD":"Trinidad and Tobago Dollar","TWD":"New Taiwan Dollar","TZS":"Tanzanian Shilling","UAH":"Ukrainian Hryvnia","UGX":"Ugandan Shilling","USD":"United States Dollar","UYU":"Uruguayan Peso","UZS":"Uzbekistan Som","VEF":"Venezuelan Bol\u00edvar Fuerte","VND":"Vietnamese Dong","VUV":"Vanuatu Vatu","WST":"Samoan Tala","XAF":"CFA Franc BEAC","XAG":"Silver (troy ounce)","XAU":"Gold (troy ounce)","XCD":"East Caribbean Dollar","XDR":"Special Drawing Rights","XOF":"CFA Franc BCEAO","XPD":"Palladium Ounce","XPF":"CFP Franc","XPT":"Platinum Ounce","YER":"Yemeni Rial","ZAR":"South African Rand","ZMK":"Zambian Kwacha (pre-2013)","ZMW":"Zambian Kwacha","ZWL":"Zimbabwean Dollar"}
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
Date: Mon, 04 Apr 2016 13:57:32 GMT
Server: Apache
Last-Modified: Sat, 16 Feb 2013 23:00:00 GMT
Cache-Control: public
ETag: "d0fb175dd5bd15ad22cfbe2ecef0f9f4"
Access-Control-Allow-Origin: *
Content-Length: 3199
Connection: close
Content-Type: application/json; charset=utf-8

{"disclaimer":"Exchange rates are provided for informational purposes only, and do not constitute financial advice of any kind. Although every attempt is made to ensure quality, NO guarantees are given whatsoever of accuracy, validity, availability, or fitness for any purpose - please use at your own risk. All usage is subject to your acceptance of the Terms and Conditions of Service, available at: http://openexchangerates.org/terms/","license":"Data sourced from various providers with public-facing APIs; copyright may apply; resale is prohibited; no warranties given of any kind. All usage is subject to your acceptance of the License Agreement available at: http://openexchangerates.org/license/","timestamp":1361055600,"base":"USD","rates":{"AED":3.672869,"AFN":51.769999,"ALL":104.798751,"AMD":406.549996,"ANG":1.7887,"AOA":95.946132,"ARS":5.009655,"AUD":0.969258,"AWG":1.789967,"AZN":0.7847,"BAM":1.465127,"BBD":2,"BDT":78.95455,"BGN":1.465315,"BHD":0.376979,"BIF":1573.6325,"BMD":1,"BND":1.235676,"BOB":6.984993,"BRL":1.963769,"BSD":1,"BTC":0.036871,"BTN":54.149416,"BWP":7.960765,"BYR":8650.918333,"BZD":2.018223,"CAD":1.004479,"CDF":918.967757,"CHF":0.921763,"CLF":0.02122,"CLP":471.190578,"CNY":6.243627,"COP":1784.424248,"CRC":500.270201,"CUP":22.687419,"CVE":82.724563,"CZK":19.006606,"DJF":177.153124,"DKK":5.582097,"DOP":40.789208,"DZD":78.29744,"EEK":11.7331,"EGP":6.728837,"ETB":18.3724,"EUR":0.748104,"FJD":1.770065,"FKP":0.644426,"GBP":0.644426,"GEL":1.65605,"GHS":1.899084,"GIP":0.635095,"GMD":32.96705,"GNF":7074.0775,"GTQ":7.824749,"GYD":202.634999,"HKD":7.754526,"HNL":19.917224,"HRK":5.675358,"HTG":42.493713,"HUF":218.905721,"IDR":9676.719572,"ILS":3.688885,"INR":54.211281,"IQD":1162.80125,"IRR":12269.0975,"ISK":128.82375,"JEP":0.644426,"JMD":94.631907,"JOD":0.70857,"JPY":93.242464,"KES":87.44366,"KGS":47.821867,"KHR":3992.76375,"KMF":368.414177,"KPW":900,"KRW":1079.374741,"KWD":0.282232,"KYD":0.822275,"KZT":150.384562,"LAK":7901.858701,"LBP":1505.93588,"LKR":126.66026,"LRD":74.25,"LSL":8.826443,"LTL":2.583805,"LVL":0.523754,"LYD":1.259093,"MAD":8.361509,"MDL":12.099609,"MGA":2198.315,"MKD":47.086109,"MMK":857.898,"MNT":1387.5,"MOP":7.980274,"MRO":298.60675,"MUR":30.658802,"MVR":15.3975,"MWK":361.842376,"MXN":12.687136,"MYR":3.094163,"MZN":30.55,"NAD":8.81998,"NGN":157.245031,"NIO":24.397725,"NOK":5.542418,"NPR":86.563924,"NZD":1.181559,"OMR":0.384974,"PAB":1,"PEN":2.567481,"PGK":2.0499,"PHP":40.612969,"PKR":98.067139,"PLN":3.133177,"PYG":4053.87125,"QAR":3.640933,"RON":3.281269,"RSD":83.334276,"RUB":30.117065,"RWF":622.261922,"SAR":3.750351,"SBD":7.07897,"SCR":12.691349,"SDG":4.41191,"SEK":6.321318,"SGD":1.236699,"SHP":0.644426,"SLL":4317.211208,"SOS":1598.59,"SRD":3.28125,"STD":18397.45,"SVC":8.741834,"SYP":70.817901,"SZL":8.815176,"THB":29.872007,"TJS":4.7545,"TMT":2.85065,"TND":1.557717,"TOP":1.727115,"TRY":1.767556,"TTD":6.390928,"TWD":29.629389,"TZS":1616.365596,"UAH":8.115192,"UGX":2637.08879,"USD":1,"UYU":18.998253,"UZS":2006.519989,"VEF":4.295129,"VND":20824.31866,"VUV":90,"WST":2.273163,"XAF":491.357387,"XCD":2.701275,"XDR":0.65495,"XOF":491.51925,"XPF":89.529112,"YER":214.756258,"ZAR":8.832957,"ZMK":5232.196666,"ZWL":322.387247}}
"###
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

    mock_connector!(UsageConnector {
        "https://openexchangerates.org" =>
r###"HTTP/1.1 200 OK
Date: Tue, 05 Apr 2016 00:47:17 GMT
Server: Apache
Access-Control-Allow-Origin: *
Content-Length: 399
Connection: close
Content-Type: application/json; charset=utf-8

{"status":200,"data":{"app_id":"c63ada06e39e411f871525b9a1c90a01","status":"active","plan":{"name":"Forever Free","quota":"1,000 requests/month","update_frequency":"hourly","features":{"base":false,"symbols":false,"experimental":true,"time-series":false,"convert":false}},"usage":{"requests":9,"requests_quota":1000,"requests_remaining":991,"days_elapsed":10,"days_remaining":20,"daily_average":0}}}
"###
    });

    #[test]
    fn usage_works() {
        let client = Client {
            app_id: "1234",
            hc: hyper::Client::with_connector(UsageConnector::default()),
        };

        let res = client.usage();
        assert!(res.is_ok());

        let usage = res.unwrap();
        let mut features = BTreeMap::new();
        features.insert("base".to_string(), false);
        features.insert("symbols".to_string(), false);
        features.insert("experimental".to_string(), true);
        features.insert("time-series".to_string(), false);
        features.insert("convert".to_string(), false);
        assert_eq!(usage, Usage{
            status: 200,
            data: UsageData{
                app_id: "c63ada06e39e411f871525b9a1c90a01".to_string(),
                status: "active".to_string(),
                plan: UsageDataPlan{
                    name: "Forever Free".to_string(),
                    quota: "1,000 requests/month".to_string(),
                    update_frequency: "hourly".to_string(),
                    features: features,
                },
                usage: UsageDataUsage{
                    requests: 9,
                    requests_quota: 1000,
                    requests_remaining: 991,
                    days_elapsed: 10,
                    days_remaining: 20,
                    daily_average: 0,
                },
            },
        });
    }
}
