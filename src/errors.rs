use std;
use reqwest;
use url;
use serde_json;

error_chain! {
    types {
        Error, ErrorKind, ResultExt, Result;
    }

    errors { FooError }

    foreign_links {
        ReqError(reqwest::Error);
        IoError(std::io::Error);
        ParseFloatError(std::num::ParseFloatError);
        UrlParserError(url::ParseError);
        Json(serde_json::Error);
        TimestampError(std::time::SystemTimeError);
    }

}
