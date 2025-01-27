pub struct Data {
    pub redirect_url: RedirectUrl,
}

pub struct RedirectUrl(String);

pub struct Pkce {
    pub redirect_url: RedirectUrl,
    pub csrf_token: CsrfToken,
    pub code_verifier: CodeVerifier,
}

pub struct CsrfToken(String);

pub struct CodeVerifier(String);

impl Pkce {
    pub const fn new(redirect_url: String, csrf_token: String, code_verifier: String) -> Self {
        Self {
            redirect_url: RedirectUrl(redirect_url),
            csrf_token: CsrfToken(csrf_token),
            code_verifier: CodeVerifier(code_verifier),
        }
    }
}

impl RedirectUrl {
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

impl CodeVerifier {
    #[inline]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
