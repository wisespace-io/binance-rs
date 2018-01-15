use std;
use reqwest;
use url;

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
    }

}
