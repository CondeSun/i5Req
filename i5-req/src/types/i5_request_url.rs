/// Represents the target URL configuration for an Interface5 API request.
///
/// This struct helps build a fully qualified API endpoint URL for sending requests
/// to an Interface5 WebServiceInput.
///
/// # Fields
///
/// - `hostname`: The hostname or IP address of the target Interface5 instance.
/// - `port`: The network port where Interface5 is running.
/// - `scenario`: The Interface5 scenario name.
/// - `tenant`: The Interface5 tenant identifier.
///
/// # Example
///
/// ```rust
/// use your_crate_name::I5RequestUrl;
///
/// let url = I5RequestUrl::new("localhost", 43001, "Processor", "Default");
/// let full_url = url.to_url();
///
/// assert_eq!(
///     full_url,
///     "https://localhost:43001/api/v1/Input/Processor/Default/Batches"
/// );
/// ```
pub struct I5RequestUrl {
    scenario: String,
    tenant: String,
    hostname: String,
    port: i32,
}

/// Creates a new [`I5RequestUrl`] instance.
///
/// # Arguments
///
/// * `hostname`: The hostname or IP address of the target Interface5 instance.
/// * `port`: The network port where Interface5 is running.
/// * `scenario`: The Interface5 scenario name.
/// * `tenant`: The Interface5 tenant identifier.
///
/// # Example
///
/// ```rust
/// use your_crate_name::I5RequestUrl;
///
/// let url = I5RequestUrl::new("localhost", 43001, "Processor", "Default");
/// ```
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

    /// Converts the [`I5RequestUrl`] into a fully qualified URL string.
    ///
    /// The generated URL follows this pattern:
    ///
    /// `https://{hostname}:{port}/api/v1/Input/{tenant}/{scenario}/Batches`
    ///
    /// # Example
    ///
    /// ```rust
    /// use your_crate_name::I5RequestUrl;
    ///
    /// let url = I5RequestUrl::new("localhost", 43001, "Processor", "Default");
    /// assert_eq!(
    ///     full_url,
    ///     "https://localhost:43001/api/v1/Input/Processor/Default/Batches"
    /// );
    pub fn to_url(&self) -> String {
        format!(
            "https://{}:{}/api/v1/Input/{}/{}/Batches",
            self.hostname, self.port, self.tenant, self.scenario
        )
    }
}
