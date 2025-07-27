//! URL validation and security checks
//!
//! This module provides comprehensive URL validation to ensure
//! downloads are safe and comply with security policies.

// use crate::{
//   config::Config,
//   error::{Error, Result}
// };
use crate::*;
use std::{
  collections::HashSet,
  net::{IpAddr, Ipv4Addr, Ipv6Addr},
  path::{Path, PathBuf}
};

/// A validated URL with extracted metadata
#[derive(Debug, Clone)]
pub struct Url {
  pub original: String,
  pub parsed: reqwest::Url,
  pub filename: String,
  pub target_path: PathBuf,
  pub exists: bool,
  pub size_hint: Option<u64>
}

impl Url {
  /// Validates and prepares all URLs for downloading.
  pub async fn new(
    urls: Vec<String>,
    target_dir: &Path,
    filename_strategy: &filename::Strategy,
    validator: UrlValidator
  ) -> Result<Vec<Self>> {
    let mut validated = Vec::new();
    let url_count = urls.len();
    trace!("Validating {} URLs", url_count);

    for (index, url_str) in urls.iter().enumerate() {
      let idx_str = index + 1;
      // Basic URL parsing and validation
      let parsed_url = match validator.validate(url_str) {
        Ok(url) => {
          debug!("Validated URL {idx_str} of {url_count}: {url_str}");
          url
        }
        Err(e) => {
          warn!("Invalid URL {idx_str}: {url_str} - {e}");
          continue;
        }
      };

      // Extract filename using configured strategy
      let filename = filename_strategy
        .extract_filename(&parsed_url, index)
        .map_err(|e| Error::Filename {
          message: format!("Failed to extract filename: {e}")
        })?;

      let target_path = target_dir.join(&filename);
      let exists = target_path.exists();

      validated.push(Self {
        original: url_str.clone(),
        parsed: parsed_url,
        filename,
        target_path,
        exists,
        size_hint: None
      });
    }

    Ok(validated)
  }
}

/// URL validator with configurable security policies.
///
/// The validator checks URLs against various security and policy
/// constraints before allowing downloads to proceed.
#[derive(Debug, Clone)]
pub struct UrlValidator {
  /// Allowed URL schemes (protocols)
  allowed_schemes: HashSet<String>,

  /// Blocked domains (exact matches)
  blocked_domains: HashSet<String>,

  /// Allowed domains (if set, only these domains are allowed)
  allowed_domains: Option<HashSet<String>>,

  /// Whether to block private/internal IP addresses
  block_private_ips: bool,

  /// Whether to block localhost addresses
  block_localhost: bool,

  /// Maximum URL length
  max_url_length: Option<usize>,

  /// Whether to validate that URLs are reachable
  validate_reachability: bool
}

impl UrlValidator {
  /// Creates a new URL validator from configuration.
  pub fn new(config: &Config) -> Self {
    Self::default()
  }

  /// Creates a URL validator with default security settings.
  pub fn secure() -> Self {
    let mut allowed_schemes = HashSet::new();
    allowed_schemes.insert("https".to_string());

    Self {
      allowed_schemes,
      blocked_domains: HashSet::new(),
      allowed_domains: None,
      block_private_ips: true,
      block_localhost: true,
      max_url_length: Some(2048),
      validate_reachability: false
    }
  }

  /// Creates a permissive URL validator that allows most URLs.
  pub fn permissive() -> Self {
    let mut allowed_schemes = HashSet::new();
    allowed_schemes.insert("http".to_string());
    allowed_schemes.insert("https".to_string());
    allowed_schemes.insert("ftp".to_string());

    Self {
      allowed_schemes,
      blocked_domains: HashSet::new(),
      allowed_domains: None,
      block_private_ips: false,
      block_localhost: false,
      max_url_length: None,
      validate_reachability: false
    }
  }

  /// Validates a URL string and returns a parsed URL if valid.
  ///
  /// # Arguments
  ///
  /// * `url_str` - The URL string to validate
  ///
  /// # Returns
  ///
  /// Returns `Ok(reqwest::Url)` if the URL is valid and passes all checks,
  /// or an error describing why validation failed.
  pub fn validate(&self, url_str: &str) -> Result<reqwest::Url> {
    // Check URL length
    if let Some(max_len) = self.max_url_length
      && url_str.len() > max_len
    {
      error!("URL too long: {}", url_str);
      return Err(Error::validation_error(format!(
        "URL too long: {} characters (max: {})",
        url_str.len(),
        max_len
      )));
    }

    // Parse the URL
    let url = reqwest::Url::parse(url_str).map_err(|e| Error::InvalidUrl {
      url: url_str.to_string(),
      reason: format!("Parse error: {e}")
    })?;

    // Validate scheme
    self.validate_scheme(&url)?;

    // Validate domain/host
    self.validate_host(&url)?;

    // Additional security checks
    self.validate_security(&url)?;

    trace!("URL validation passed: {url_str}");
    Ok(url)
  }

  /// Validates multiple URLs and returns results for each.
  pub fn validate_batch(&self, urls: &[String]) -> Vec<Result<reqwest::Url>> {
    urls.iter().map(|url_str| self.validate(url_str)).collect()
  }

  /// Validates the URL scheme (protocol).
  fn validate_scheme(&self, url: &reqwest::Url) -> Result<()> {
    let scheme = url.scheme();

    if !self.allowed_schemes.contains(scheme) {
      return Err(Error::validation_error(format!(
        "Unsupported scheme '{}'. Allowed schemes: {:?}",
        scheme, self.allowed_schemes
      )));
    }

    Ok(())
  }

  /// Validates the URL host/domain.
  fn validate_host(&self, url: &reqwest::Url) -> Result<()> {
    let host = url
      .host_str()
      .ok_or_else(|| Error::validation_error("URL must have a host"))?;

    // Check blocked domains
    if self.blocked_domains.contains(host) {
      return Err(Error::validation_error(format!(
        "Domain '{host}' is blocked"
      )));
    }

    // Check allowed domains (if allowlist is configured)
    if let Some(ref allowed) = self.allowed_domains
      && !allowed.contains(host)
    {
      return Err(Error::validation_error(format!(
        "Domain '{host}' is not in the allowed list"
      )));
    }
    // IP address validation
    if let Ok(ip) = host.parse::<IpAddr>() {
      self.validate_ip_address(&ip)?;
    }

    Ok(())
  }

  /// Validates IP addresses against security policies.
  fn validate_ip_address(&self, ip: &IpAddr) -> Result<()> {
    match ip {
      IpAddr::V4(ipv4) => self.validate_ipv4(ipv4),
      IpAddr::V6(ipv6) => self.validate_ipv6(ipv6)
    }
  }

  /// Validates IPv4 addresses.
  fn validate_ipv4(&self, ip: &Ipv4Addr) -> Result<()> {
    if self.block_localhost && ip.is_loopback() {
      return Err(Error::validation_error("Localhost addresses are blocked"));
    }

    if self.block_private_ips && self.is_private_ipv4(ip) {
      return Err(Error::validation_error("Private IP addresses are blocked"));
    }

    Ok(())
  }

  /// Validates IPv6 addresses.
  fn validate_ipv6(&self, ip: &Ipv6Addr) -> Result<()> {
    if self.block_localhost && ip.is_loopback() {
      return Err(Error::validation_error("Localhost addresses are blocked"));
    }

    if self.block_private_ips && self.is_private_ipv6(ip) {
      return Err(Error::validation_error("Private IP addresses are blocked"));
    }

    Ok(())
  }

  /// Checks if an IPv4 address is private.
  fn is_private_ipv4(&self, ip: &Ipv4Addr) -> bool {
    ip.is_private()
      || ip.is_link_local()
      || ip.is_multicast()
      || ip.is_broadcast()
      || ip.octets()[0] == 0 // "This network"
  }

  /// Checks if an IPv6 address is private.
  fn is_private_ipv6(&self, ip: &Ipv6Addr) -> bool {
    // Check for various private/special IPv6 ranges
    let segments = ip.segments();

    // Link-local (fe80::/10)
    if segments[0] & 0xffc0 == 0xfe80 {
      return true;
    }

    // Unique local (fc00::/7)
    if segments[0] & 0xfe00 == 0xfc00 {
      return true;
    }

    // Multicast (ff00::/8)
    if segments[0] & 0xff00 == 0xff00 {
      return true;
    }

    false
  }

  /// Performs additional security validation.
  fn validate_security(&self, url: &reqwest::Url) -> Result<()> {
    // Check for suspicious URL patterns
    let url_str = url.as_str();

    // Check for URL redirection attempts
    if url_str.contains("redirect") || url_str.contains("goto") {
      warn!("URL contains potential redirection: {}", url_str);
    }

    // Check for data URLs (which could contain malicious content)
    if url.scheme() == "data" {
      return Err(Error::validation_error(
        "Data URLs are not supported for security reasons"
      ));
    }

    // Check for file URLs
    if url.scheme() == "file" {
      return Err(Error::validation_error("File URLs are not supported"));
    }

    // Check for JavaScript URLs
    if url.scheme() == "javascript" {
      return Err(Error::validation_error(
        "JavaScript URLs are not supported for security reasons"
      ));
    }

    Ok(())
  }

  /// Adds a domain to the blocked list.
  pub fn block_domain<S: Into<String>>(&mut self, domain: S) {
    self.blocked_domains.insert(domain.into());
  }

  /// Adds multiple domains to the blocked list.
  pub fn block_domains<I, S>(&mut self, domains: I)
  where
    I: IntoIterator<Item = S>,
    S: Into<String>
  {
    for domain in domains {
      self.blocked_domains.insert(domain.into());
    }
  }

  /// Sets the allowed domains list (replaces any existing list).
  pub fn set_allowed_domains<I, S>(&mut self, domains: I)
  where
    I: IntoIterator<Item = S>,
    S: Into<String>
  {
    let allowed = domains.into_iter().map(|s| s.into()).collect();
    self.allowed_domains = Some(allowed);
  }

  /// Clears the allowed domains list (allows all domains except blocked ones).
  pub fn clear_allowed_domains(&mut self) {
    self.allowed_domains = None;
  }

  /// Adds a scheme to the allowed list.
  pub fn allow_scheme<S: Into<String>>(&mut self, scheme: S) {
    self.allowed_schemes.insert(scheme.into());
  }

  /// Sets whether to block private IP addresses.
  pub fn set_block_private_ips(&mut self, block: bool) {
    self.block_private_ips = block;
  }

  /// Sets whether to block localhost addresses.
  pub fn set_block_localhost(&mut self, block: bool) {
    self.block_localhost = block;
  }

  /// Sets the maximum allowed URL length.
  pub fn set_max_url_length(&mut self, length: Option<usize>) {
    self.max_url_length = length;
  }
}

impl Default for UrlValidator {
  fn default() -> Self {
    let mut allowed_schemes = HashSet::new();
    allowed_schemes.insert("http".to_string());
    allowed_schemes.insert("https".to_string());

    Self {
      allowed_schemes,
      blocked_domains: HashSet::new(),
      allowed_domains: None,
      block_private_ips: false,
      block_localhost: false,
      max_url_length: Some(4096),
      validate_reachability: false
    }
  }
}

/// Builder for creating URL validators with specific configurations.
#[derive(Debug, Default)]
pub struct UrlValidatorBuilder {
  validator: UrlValidator
}

impl UrlValidatorBuilder {
  /// Creates a new validator builder.
  pub fn new() -> Self {
    Self::default()
  }

  /// Starts with secure defaults.
  pub fn secure() -> Self {
    Self {
      validator: UrlValidator::secure()
    }
  }

  /// Starts with permissive defaults.
  pub fn permissive() -> Self {
    Self {
      validator: UrlValidator::permissive()
    }
  }

  /// Allows a specific scheme.
  pub fn allow_scheme<S: Into<String>>(mut self, scheme: S) -> Self {
    self.validator.allowed_schemes.insert(scheme.into());
    self
  }

  /// Allows multiple schemes.
  pub fn allow_schemes<I, S>(mut self, schemes: I) -> Self
  where
    I: IntoIterator<Item = S>,
    S: Into<String>
  {
    for scheme in schemes {
      self.validator.allowed_schemes.insert(scheme.into());
    }
    self
  }

  /// Blocks a specific domain.
  pub fn block_domain<S: Into<String>>(mut self, domain: S) -> Self {
    self.validator.blocked_domains.insert(domain.into());
    self
  }

  /// Sets allowed domains (allowlist).
  pub fn allowed_domains<I, S>(mut self, domains: I) -> Self
  where
    I: IntoIterator<Item = S>,
    S: Into<String>
  {
    let allowed = domains.into_iter().map(|s| s.into()).collect();
    self.validator.allowed_domains = Some(allowed);
    self
  }

  /// Sets whether to block private IP addresses.
  pub fn block_private_ips(mut self, block: bool) -> Self {
    self.validator.block_private_ips = block;
    self
  }

  /// Sets whether to block localhost.
  pub fn block_localhost(mut self, block: bool) -> Self {
    self.validator.block_localhost = block;
    self
  }

  /// Sets maximum URL length.
  pub fn max_url_length(mut self, length: usize) -> Self {
    self.validator.max_url_length = Some(length);
    self
  }

  /// Builds the validator.
  pub fn build(self) -> UrlValidator {
    self.validator
  }
}
