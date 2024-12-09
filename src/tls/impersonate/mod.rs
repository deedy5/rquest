#![allow(missing_docs, missing_debug_implementations)]

pub mod chrome;
pub mod edge;
pub mod okhttp;
pub mod safari;

use super::{Http2Settings, TlsSettings};
use chrome::*;
use edge::*;
use http::{HeaderMap, HeaderName};
use okhttp::*;
use safari::*;
use std::{borrow::Cow, fmt::Debug, str::FromStr};
use typed_builder::TypedBuilder;
use Impersonate::*;

mod impersonate_imports {
    pub use super::ImpersonateSettings;
    pub use crate::conditional_headers;
    pub use crate::static_join;
    pub use http::{
        header::{
            ACCEPT, ACCEPT_ENCODING, ACCEPT_LANGUAGE, CACHE_CONTROL, DNT,
            UPGRADE_INSECURE_REQUESTS, USER_AGENT,
        },
        HeaderMap, HeaderValue,
    };
    pub use std::borrow::Cow;
    pub use std::sync::LazyLock;

    #[macro_export]
    macro_rules! conditional_headers {
        ($with_headers:expr, $initializer:expr) => {
            if $with_headers {
                static HEADER_INITIALIZER: LazyLock<HeaderMap> = LazyLock::new($initializer);
                Some(Cow::Borrowed(&*HEADER_INITIALIZER))
            } else {
                None
            }
        };
    }
}

mod http2_imports {
    pub use hyper::PseudoOrder::{self, *};
    pub use hyper::SettingsOrder::{self, *};
}

mod tls_imports {
    pub use crate::static_join;
    pub use crate::tls::{cert_compression::CertCompressionAlgorithm, TlsSettings, Version};
    pub use boring::ssl::SslCurve;
    pub use std::borrow::Cow;
    pub use typed_builder::TypedBuilder;

    #[macro_export]
    macro_rules! static_join {
        ($sep:expr, $first:expr $(, $rest:expr)*) => {
            concat!($first $(, $sep, $rest)*)
        };
    }
}

/// Impersonate Settings.
#[derive(TypedBuilder)]
pub struct ImpersonateSettings {
    /// The SSL connector builder.
    pub(crate) tls: TlsSettings,

    /// HTTP/2 settings.
    pub(crate) http2: Http2Settings,

    /// Http headers
    #[builder(default, setter(into))]
    pub(crate) headers: Option<Cow<'static, HeaderMap>>,

    /// Http headers order
    #[builder(default, setter(strip_option))]
    pub(crate) headers_order: Option<Cow<'static, [HeaderName]>>,
}

macro_rules! impersonate_match {
    ($ver:expr, $with_headers:expr, $($variant:pat => $path:path),+) => {
        match $ver {
            $(
                $variant => {
                    $path($with_headers)
                },
            )+
        }
    }
}

/// Get the connection settings for the given impersonate version
#[inline]
pub fn tls_settings(ver: Impersonate, with_headers: bool) -> ImpersonateSettings {
    impersonate_match!(
        ver,
        with_headers,
        // Chrome
        Chrome100 => v100::get_settings,
        Chrome101 => v101::get_settings,
        Chrome104 => v104::get_settings,
        Chrome105 => v105::get_settings,
        Chrome106 => v106::get_settings,
        Chrome107 => v107::get_settings,
        Chrome108 => v108::get_settings,
        Chrome109 => v109::get_settings,
        Chrome114 => v114::get_settings,
        Chrome116 => v116::get_settings,
        Chrome117 => v117::get_settings,
        Chrome118 => v118::get_settings,
        Chrome119 => v119::get_settings,
        Chrome120 => v120::get_settings,
        Chrome123 => v123::get_settings,
        Chrome124 => v124::get_settings,
        Chrome126 => v126::get_settings,
        Chrome127 => v127::get_settings,
        Chrome128 => v128::get_settings,
        Chrome129 => v129::get_settings,
        Chrome130 => v130::get_settings,
        Chrome131 => v131::get_settings,

        // Safari
        SafariIos17_2 => safari_ios_17_2::get_settings,
        SafariIos17_4_1 => safari_ios_17_4_1::get_settings,
        SafariIos16_5 => safari_ios_16_5::get_settings,
        Safari15_3 => safari15_3::get_settings,
        Safari15_5 => safari15_5::get_settings,
        Safari15_6_1 => safari15_6_1::get_settings,
        Safari16 => safari16::get_settings,
        Safari16_5 => safari16_5::get_settings,
        Safari17_0 => safari17_0::get_settings,
        Safari17_2_1 => safari17_2_1::get_settings,
        Safari17_4_1 => safari17_4_1::get_settings,
        Safari17_5 => safari17_5::get_settings,
        Safari18 => safari18::get_settings,
        SafariIPad18 => safari_ipad_18::get_settings,

        // OkHttp
        OkHttp3_9 => okhttp3_9::get_settings,
        OkHttp3_11 => okhttp3_11::get_settings,
        OkHttp3_13 => okhttp3_13::get_settings,
        OkHttp3_14 => okhttp3_14::get_settings,
        OkHttp4_9 => okhttp4_9::get_settings,
        OkHttp4_10 => okhttp4_10::get_settings,
        OkHttp5 => okhttp5::get_settings,

        // Edge
        Edge101 => edge101::get_settings,
        Edge122 => edge122::get_settings,
        Edge127 => edge127::get_settings
    )
}

#[derive(Clone, Copy, Debug, Default)]
pub enum Impersonate {
    // Chrome
    Chrome100,
    Chrome101,
    Chrome104,
    Chrome105,
    Chrome106,
    Chrome107,
    Chrome108,
    Chrome109,
    Chrome114,
    Chrome116,
    Chrome117,
    Chrome118,
    Chrome119,
    Chrome120,
    Chrome123,
    Chrome124,
    Chrome126,
    Chrome127,
    Chrome128,
    Chrome129,
    Chrome130,
    #[default]
    Chrome131,

    // Safari
    SafariIos17_2,
    SafariIos17_4_1,
    SafariIos16_5,
    Safari15_3,
    Safari15_5,
    Safari15_6_1,
    Safari16,
    Safari16_5,
    Safari17_0,
    Safari17_2_1,
    Safari17_4_1,
    Safari17_5,
    Safari18,
    SafariIPad18,

    // OkHttp
    OkHttp3_9,
    OkHttp3_11,
    OkHttp3_13,
    OkHttp3_14,
    OkHttp4_9,
    OkHttp4_10,
    OkHttp5,

    // Edge
    Edge101,
    Edge122,
    Edge127,
}

macro_rules! impl_from_str {
    ($(($variant:ident, $string:expr)),* $(,)?) => {
        impl FromStr for Impersonate {
            type Err = String;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s {
                    $( $string => Ok(Impersonate::$variant), )*
                    _ => Err(format!("Unknown impersonate version: {}", s)),
                }
            }
        }
    };
}

impl_from_str! {
    // Chrome
    (Chrome100, "chrome_100"),
    (Chrome101, "chrome_101"),
    (Chrome104, "chrome_104"),
    (Chrome105, "chrome_105"),
    (Chrome106, "chrome_106"),
    (Chrome107, "chrome_107"),
    (Chrome108, "chrome_108"),
    (Chrome109, "chrome_109"),
    (Chrome114, "chrome_114"),
    (Chrome116, "chrome_116"),
    (Chrome117, "chrome_117"),
    (Chrome118, "chrome_118"),
    (Chrome119, "chrome_119"),
    (Chrome120, "chrome_120"),
    (Chrome123, "chrome_123"),
    (Chrome124, "chrome_124"),
    (Chrome126, "chrome_126"),
    (Chrome127, "chrome_127"),
    (Chrome128, "chrome_128"),
    (Chrome129, "chrome_129"),
    (Chrome130, "chrome_130"),
    (Chrome131, "chrome_131"),
    // Safari
    (SafariIos17_2, "safari_ios_17.2"),
    (SafariIos17_4_1, "safari_ios_17.4.1"),
    (SafariIos16_5, "safari_ios_16.5"),
    (Safari15_3, "safari_15.3"),
    (Safari15_5, "safari_15.5"),
    (Safari15_6_1, "safari_15.6.1"),
    (Safari16, "safari_16"),
    (Safari16_5, "safari_16.5"),
    (Safari17_0, "safari_17.0"),
    (Safari17_2_1, "safari_17.2.1"),
    (Safari17_4_1, "safari_17.4.1"),
    (Safari17_5, "safari_17.5"),
    (Safari18, "safari_18"),
    (SafariIPad18, "safari_ipad_18"),

    // OkHttp
    (OkHttp3_9, "okhttp_3.9"),
    (OkHttp3_11, "okhttp_3.11"),
    (OkHttp3_13, "okhttp_3.13"),
    (OkHttp3_14, "okhttp_3.14"),
    (OkHttp4_9, "okhttp_4.9"),
    (OkHttp4_10, "okhttp_4.10"),
    (OkHttp5, "okhttp_5"),

    // Edge
    (Edge101, "edge_101"),
    (Edge122, "edge_122"),
    (Edge127, "edge_127"),
}
