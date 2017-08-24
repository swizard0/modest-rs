use modest_sys::myhtml as ffi;

#[derive(Clone, Copy, Debug)]
pub enum Encoding {
    Default,
    NotDetermined,
    Utf16le,
    Utf16be,
    XUserDefined,
    Big5,
    EucJp,
    EucKr,
    Gb18030,
    Gbk,
    Ibm866,
    Iso2022Jp,
    Iso885910,
    Iso885913,
    Iso885914,
    Iso885915,
    Iso885916,
    Iso88592,
    Iso88593,
    Iso88594,
    Iso88595,
    Iso88596,
    Iso88597,
    Iso88598,
    Iso88598I,
    Koi8R,
    Koi8U,
    Macintosh,
    ShiftJis,
    Windows1250,
    Windows1251,
    Windows1252,
    Windows1253,
    Windows1254,
    Windows1255,
    Windows1256,
    Windows1257,
    Windows1258,
    Windows874,
    XMacCyrillic,
}

impl Encoding {
    pub fn from_ffi(enc: ffi::myencoding_t) -> Encoding {
        match enc {
            ffi::myencoding_t::MyENCODING_DEFAULT =>
                Encoding::Default,
            ffi::myencoding_t::MyENCODING_NOT_DETERMINED =>
                Encoding::NotDetermined,
            ffi::myencoding_t::MyENCODING_UTF_16LE =>
                Encoding::Utf16le,
            ffi::myencoding_t::MyENCODING_UTF_16BE =>
                Encoding::Utf16be,
            ffi::myencoding_t::MyENCODING_X_USER_DEFINED =>
                Encoding::XUserDefined,
            ffi::myencoding_t::MyENCODING_BIG5 =>
                Encoding::Big5,
            ffi::myencoding_t::MyENCODING_EUC_JP =>
                Encoding::EucJp,
            ffi::myencoding_t::MyENCODING_EUC_KR =>
                Encoding::EucKr,
            ffi::myencoding_t::MyENCODING_GB18030 =>
                Encoding::Gb18030,
            ffi::myencoding_t::MyENCODING_GBK =>
                Encoding::Gbk,
            ffi::myencoding_t::MyENCODING_IBM866 =>
                Encoding::Ibm866,
            ffi::myencoding_t::MyENCODING_ISO_2022_JP =>
                Encoding::Iso2022Jp,
            ffi::myencoding_t::MyENCODING_ISO_8859_10 =>
                Encoding::Iso885910,
            ffi::myencoding_t::MyENCODING_ISO_8859_13 =>
                Encoding::Iso885913,
            ffi::myencoding_t::MyENCODING_ISO_8859_14 =>
                Encoding::Iso885914,
            ffi::myencoding_t::MyENCODING_ISO_8859_15 =>
                Encoding::Iso885915,
            ffi::myencoding_t::MyENCODING_ISO_8859_16 =>
                Encoding::Iso885916,
            ffi::myencoding_t::MyENCODING_ISO_8859_2 =>
                Encoding::Iso88592,
            ffi::myencoding_t::MyENCODING_ISO_8859_3 =>
                Encoding::Iso88593,
            ffi::myencoding_t::MyENCODING_ISO_8859_4 =>
                Encoding::Iso88594,
            ffi::myencoding_t::MyENCODING_ISO_8859_5 =>
                Encoding::Iso88595,
            ffi::myencoding_t::MyENCODING_ISO_8859_6 =>
                Encoding::Iso88596,
            ffi::myencoding_t::MyENCODING_ISO_8859_7 =>
                Encoding::Iso88597,
            ffi::myencoding_t::MyENCODING_ISO_8859_8 =>
                Encoding::Iso88598,
            ffi::myencoding_t::MyENCODING_ISO_8859_8_I =>
                Encoding::Iso88598I,
            ffi::myencoding_t::MyENCODING_KOI8_R =>
                Encoding::Koi8R,
            ffi::myencoding_t::MyENCODING_KOI8_U =>
                Encoding::Koi8U,
            ffi::myencoding_t::MyENCODING_MACINTOSH =>
                Encoding::Macintosh,
            ffi::myencoding_t::MyENCODING_SHIFT_JIS =>
                Encoding::ShiftJis,
            ffi::myencoding_t::MyENCODING_WINDOWS_1250 =>
                Encoding::Windows1250,
            ffi::myencoding_t::MyENCODING_WINDOWS_1251 =>
                Encoding::Windows1251,
            ffi::myencoding_t::MyENCODING_WINDOWS_1252 =>
                Encoding::Windows1252,
            ffi::myencoding_t::MyENCODING_WINDOWS_1253 =>
                Encoding::Windows1253,
            ffi::myencoding_t::MyENCODING_WINDOWS_1254 =>
                Encoding::Windows1254,
            ffi::myencoding_t::MyENCODING_WINDOWS_1255 =>
                Encoding::Windows1255,
            ffi::myencoding_t::MyENCODING_WINDOWS_1256 =>
                Encoding::Windows1256,
            ffi::myencoding_t::MyENCODING_WINDOWS_1257 =>
                Encoding::Windows1257,
            ffi::myencoding_t::MyENCODING_WINDOWS_1258 =>
                Encoding::Windows1258,
            ffi::myencoding_t::MyENCODING_WINDOWS_874 =>
                Encoding::Windows874,
            ffi::myencoding_t::MyENCODING_X_MAC_CYRILLIC =>
                Encoding::XMacCyrillic,
            ffi::myencoding_t::MyENCODING_LAST_ENTRY =>
                Encoding::Default,
        }
    }

    pub fn to_ffi(&self) -> ffi::myencoding_t {
        match *self {
            Encoding::Default =>
                ffi::myencoding_t::MyENCODING_DEFAULT,
            Encoding::NotDetermined =>
                ffi::myencoding_t::MyENCODING_NOT_DETERMINED,
            Encoding::Utf16le =>
                ffi::myencoding_t::MyENCODING_UTF_16LE,
            Encoding::Utf16be =>
                ffi::myencoding_t::MyENCODING_UTF_16BE,
            Encoding::XUserDefined =>
                ffi::myencoding_t::MyENCODING_X_USER_DEFINED,
            Encoding::Big5 =>
                ffi::myencoding_t::MyENCODING_BIG5,
            Encoding::EucJp =>
                ffi::myencoding_t::MyENCODING_EUC_JP,
            Encoding::EucKr =>
                ffi::myencoding_t::MyENCODING_EUC_KR,
            Encoding::Gb18030 =>
                ffi::myencoding_t::MyENCODING_GB18030,
            Encoding::Gbk =>
                ffi::myencoding_t::MyENCODING_GBK,
            Encoding::Ibm866 =>
                ffi::myencoding_t::MyENCODING_IBM866,
            Encoding::Iso2022Jp =>
                ffi::myencoding_t::MyENCODING_ISO_2022_JP,
            Encoding::Iso885910 =>
                ffi::myencoding_t::MyENCODING_ISO_8859_10,
            Encoding::Iso885913 =>
                ffi::myencoding_t::MyENCODING_ISO_8859_13,
            Encoding::Iso885914 =>
                ffi::myencoding_t::MyENCODING_ISO_8859_14,
            Encoding::Iso885915 =>
                ffi::myencoding_t::MyENCODING_ISO_8859_15,
            Encoding::Iso885916 =>
                ffi::myencoding_t::MyENCODING_ISO_8859_16,
            Encoding::Iso88592 =>
                ffi::myencoding_t::MyENCODING_ISO_8859_2,
            Encoding::Iso88593 =>
                ffi::myencoding_t::MyENCODING_ISO_8859_3,
            Encoding::Iso88594 =>
                ffi::myencoding_t::MyENCODING_ISO_8859_4,
            Encoding::Iso88595 =>
                ffi::myencoding_t::MyENCODING_ISO_8859_5,
            Encoding::Iso88596 =>
                ffi::myencoding_t::MyENCODING_ISO_8859_6,
            Encoding::Iso88597 =>
                ffi::myencoding_t::MyENCODING_ISO_8859_7,
            Encoding::Iso88598 =>
                ffi::myencoding_t::MyENCODING_ISO_8859_8,
            Encoding::Iso88598I =>
                ffi::myencoding_t::MyENCODING_ISO_8859_8_I,
            Encoding::Koi8R =>
                ffi::myencoding_t::MyENCODING_KOI8_R,
            Encoding::Koi8U =>
                ffi::myencoding_t::MyENCODING_KOI8_U,
            Encoding::Macintosh =>
                ffi::myencoding_t::MyENCODING_MACINTOSH,
            Encoding::ShiftJis =>
                ffi::myencoding_t::MyENCODING_SHIFT_JIS,
            Encoding::Windows1250 =>
                ffi::myencoding_t::MyENCODING_WINDOWS_1250,
            Encoding::Windows1251 =>
                ffi::myencoding_t::MyENCODING_WINDOWS_1251,
            Encoding::Windows1252 =>
                ffi::myencoding_t::MyENCODING_WINDOWS_1252,
            Encoding::Windows1253 =>
                ffi::myencoding_t::MyENCODING_WINDOWS_1253,
            Encoding::Windows1254 =>
                ffi::myencoding_t::MyENCODING_WINDOWS_1254,
            Encoding::Windows1255 =>
                ffi::myencoding_t::MyENCODING_WINDOWS_1255,
            Encoding::Windows1256 =>
                ffi::myencoding_t::MyENCODING_WINDOWS_1256,
            Encoding::Windows1257 =>
                ffi::myencoding_t::MyENCODING_WINDOWS_1257,
            Encoding::Windows1258 =>
                ffi::myencoding_t::MyENCODING_WINDOWS_1258,
            Encoding::Windows874 =>
                ffi::myencoding_t::MyENCODING_WINDOWS_874,
            Encoding::XMacCyrillic =>
                ffi::myencoding_t::MyENCODING_X_MAC_CYRILLIC,
        }
    }
}

impl Default for Encoding {
    fn default() -> Encoding {
        Encoding::Default
    }
}
