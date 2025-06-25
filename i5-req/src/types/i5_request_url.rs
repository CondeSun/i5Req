pub struct I5RequestUrl {
    scenario: String,
    tenant: String,
    hostname: String,
    port: i32,
}

impl I5RequestUrl {
    pub fn new(
        hostname: impl Into<String>,
        port: i32,
        scenario: impl Into<String>,
        tenant: impl Into<String>,
    ) -> I5RequestUrl {
        I5RequestUrl {
            scenario: scenario.into(),
            tenant: tenant.into(),
            hostname: hostname.into(),
            port,
        }
    }

    pub fn to_url(&self) -> String {
        format!(
            "https://{}:{}/api/v1/Input/{}/{}/Batches",
            self.hostname, self.port, self.tenant, self.scenario
        )
    }
}
