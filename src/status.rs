/// taken from https://www.rfc-editor.org/rfc/rfc9110.html#name-overview-of-status-codes
#[repr(u16)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Status {
    Continue = 100,
    SwitchingProtocols,

    Ok = 200,
    Created,
    Accepted,
    NonAuthoritativeInformation,
    NoContent,
    ResetContent,
    PartialContent,

    MultipleChoices = 300,
    MovedPermanently,
    Found,
    SeeOther,
    NotModified,
    UseProxy,
    // 306 is reserved but unused
    TemporaryRedirect = 307,
    PermanentRedirect,

    BadRequest = 400,
    Unauthorized,
    PaymentRequired,
    Forbidden,
    NotFound,
    MethodNotAllowed,
    NotAcceptable,
    ProxyAuthenticationRequired,
    RequestTimeout,
    Conflict,
    Gone,
    LengthRequired,
    PreconditionFailed,
    ContentTooLarge,
    UriTooLong,
    UnsupportedMediaType,
    RangeNotSatisfiable,
    ExpectionFailed,
    // 418 is reserved because its a joke
    MisdirectedRequest,
    UnprocesableContent,
    UpgradeRequired,

    InternalServerError = 500,
    NotImplemented,
    BadGateway,
    ServiceUnavailable,
    GatewayTimeout,
    HttpVersionNotSupported,
}

impl Status {
    pub fn code(&self) -> usize {
        *self as usize
    }

    pub fn text(&self) -> &str {
        match *self {
            Status::Continue => "100 Continue",
            Status::SwitchingProtocols => "101 Switching Protocols",

            Status::Ok => "200 OK",
            Status::Created => "201 Created",
            Status::Accepted => "202 Accepted",
            Status::NonAuthoritativeInformation => "203 Non-Authoritative Information",
            Status::NoContent => "204 No Content",
            Status::ResetContent => "205 Reset Content",
            Status::PartialContent => "206 Partial Content",

            Status::MultipleChoices => "300 Multiple Choices",
            Status::MovedPermanently => "301 Moved Permanently",
            Status::Found => "302 Found",
            Status::SeeOther => "303 See Other",
            Status::NotModified => "304 Not Modified",
            Status::UseProxy => "305 Use Proxy",
            Status::TemporaryRedirect => "307 Temporary Redirect",
            Status::PermanentRedirect => "308 Permanent Redirect",

            Status::BadRequest => "400 Bad Request",
            Status::Unauthorized => "401 Unauthorized",
            Status::PaymentRequired => "402 Payment Required",
            Status::Forbidden => "403 Forbidden",
            Status::NotFound => "404 Not Found",
            Status::MethodNotAllowed => "405 Method Not Allowed",
            Status::NotAcceptable => "406 Not Acceptable",
            Status::ProxyAuthenticationRequired => "407 Proxy Authentication Required",
            Status::RequestTimeout => "408 Request Timeout",
            Status::Conflict => "409 Conflict",
            Status::Gone => "410 Gone",
            Status::LengthRequired => "411 Length Required",
            Status::PreconditionFailed => "412 Precondition Failed",
            Status::ContentTooLarge => "413 Content Too Large",
            Status::UriTooLong => "414 URI Too Long",
            Status::UnsupportedMediaType => "415 Unsupported Media Type",
            Status::RangeNotSatisfiable => "416 Range Not Satisfiable",
            Status::ExpectionFailed => "417 Expectation Failed",
            Status::MisdirectedRequest => "421 Misdirected Request",
            Status::UnprocesableContent => "422 Unprocessable Content",
            Status::UpgradeRequired => "426 Upgrade Required",

            Status::InternalServerError => "500 Internal Server Error",
            Status::NotImplemented => "501 Not Implemented",
            Status::BadGateway => "502 Bad Gateway",
            Status::ServiceUnavailable => "503 Service Unavailable",
            Status::GatewayTimeout => "504 Gateway Timeout",
            Status::HttpVersionNotSupported => "505 HTTP Version Not Supported",
        }
    }

    pub fn as_bytes(&self) -> &[u8] {
        self.text().as_bytes()
    }
}

// PERF: is this is too slow, replace it with a lookup table
impl TryFrom<u16> for Status {
    type Error = &'static str;

    fn try_from(code: u16) -> Result<Self, Self::Error> {
        Ok(match code {
            100 => Status::Continue,
            101 => Status::SwitchingProtocols,
            200 => Status::Ok,
            201 => Status::Created,
            202 => Status::Accepted,
            203 => Status::NonAuthoritativeInformation,
            204 => Status::NoContent,
            205 => Status::ResetContent,
            206 => Status::PartialContent,
            300 => Status::MultipleChoices,
            301 => Status::MovedPermanently,
            302 => Status::Found,
            303 => Status::SeeOther,
            304 => Status::NotModified,
            305 => Status::UseProxy,
            307 => Status::TemporaryRedirect,
            308 => Status::PermanentRedirect,
            400 => Status::BadRequest,
            401 => Status::Unauthorized,
            402 => Status::PaymentRequired,
            403 => Status::Forbidden,
            404 => Status::NotFound,
            405 => Status::MethodNotAllowed,
            406 => Status::NotAcceptable,
            407 => Status::ProxyAuthenticationRequired,
            408 => Status::RequestTimeout,
            409 => Status::Conflict,
            410 => Status::Gone,
            411 => Status::LengthRequired,
            412 => Status::PreconditionFailed,
            413 => Status::ContentTooLarge,
            414 => Status::UriTooLong,
            415 => Status::UnsupportedMediaType,
            416 => Status::RangeNotSatisfiable,
            417 => Status::ExpectionFailed,
            421 => Status::MisdirectedRequest,
            422 => Status::UnprocesableContent,
            426 => Status::UpgradeRequired,
            500 => Status::InternalServerError,
            501 => Status::NotImplemented,
            502 => Status::BadGateway,
            503 => Status::ServiceUnavailable,
            504 => Status::GatewayTimeout,
            505 => Status::HttpVersionNotSupported,
            _ => return Err("Invalid status code code"),
        })
    }
}
