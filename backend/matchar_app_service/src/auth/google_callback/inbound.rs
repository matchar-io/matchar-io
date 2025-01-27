pub struct Data {
    pub(crate) code: Code,
    pub(crate) csrf_token: CsrfToken,
}

pub struct Code(String);

pub struct CsrfToken(String);

impl Data {
    pub fn new(code: &str, csrf_token: &str) -> Self {
        Self {
            code: Code(code.to_owned()),
            csrf_token: CsrfToken(csrf_token.to_owned()),
        }
    }
}

impl Code {
    #[inline]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl CsrfToken {
    #[inline]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
