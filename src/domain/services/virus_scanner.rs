//! Virus Scanner Service
//!
//! Provides file scanning capabilities to detect malicious content.
//! Blocks dangerous file types and detects executable binaries.

use std::path::Path;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::domain::entity::ThreatLevel;

/// Result of a virus scan
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VirusScanResult {
    pub threat_level: ThreatLevel,
    pub threats: Vec<String>,
    pub scan_details: serde_json::Value,
}

impl VirusScanResult {
    /// Create a safe result
    pub fn safe() -> Self {
        Self {
            threat_level: ThreatLevel::Safe,
            threats: vec![],
            scan_details: json!({"scan_method": "basic", "status": "clean"}),
        }
    }

    /// Create a blocked result for dangerous files
    pub fn blocked(reason: String) -> Self {
        Self {
            threat_level: ThreatLevel::Critical,
            threats: vec![reason.clone()],
            scan_details: json!({"scan_method": "extension_check", "blocked_reason": reason}),
        }
    }

    /// Create a high-threat result
    pub fn high_threat(reason: String) -> Self {
        Self {
            threat_level: ThreatLevel::High,
            threats: vec![reason.clone()],
            scan_details: json!({"scan_method": "magic_bytes", "threat_reason": reason}),
        }
    }
}

/// Virus Scanner Service
///
/// Scans files for potential threats using:
/// 1. Extension-based blocking (executables, scripts)
/// 2. Magic byte detection (PE, ELF binaries)
/// 3. (Future) ClamAV integration for signature-based detection
#[derive(Debug, Clone, Default)]
pub struct VirusScannerService {
    /// List of blocked file extensions
    blocked_extensions: Vec<&'static str>,
}

impl VirusScannerService {
    /// Create a new virus scanner service
    pub fn new() -> Self {
        Self {
            blocked_extensions: vec![
                // Windows executables
                "exe", "bat", "cmd", "com", "scr", "pif", "msi", "msp",
                // Scripts
                "vbs", "vbe", "js", "jse", "ws", "wsf", "wsc", "wsh",
                "ps1", "ps1xml", "ps2", "ps2xml", "psc1", "psc2",
                // Libraries
                "dll", "sys", "drv", "ocx", "cpl",
                // Shortcuts
                "lnk", "url",
                // Archives that can hide malware
                "hta", "reg",
            ],
        }
    }

    /// Scan content for viruses/malware
    pub fn scan(&self, content: &[u8], filename: &str) -> VirusScanResult {
        // Phase 1: Check blocked extensions
        if let Some(result) = self.check_extension(filename) {
            return result;
        }

        // Phase 2: Check magic bytes for executables
        if let Some(result) = self.check_magic_bytes(content) {
            return result;
        }

        // Phase 3: Additional heuristics
        if let Some(result) = self.check_heuristics(content, filename) {
            return result;
        }

        // File passed all checks
        VirusScanResult::safe()
    }

    /// Check file extension against blocked list
    fn check_extension(&self, filename: &str) -> Option<VirusScanResult> {
        let ext = Path::new(filename)
            .extension()
            .and_then(|e| e.to_str())
            .map(|e| e.to_lowercase())
            .unwrap_or_default();

        if self.blocked_extensions.contains(&ext.as_str()) {
            return Some(VirusScanResult::blocked(
                format!("Blocked extension: .{}", ext),
            ));
        }

        None
    }

    /// Check for executable magic bytes
    fn check_magic_bytes(&self, content: &[u8]) -> Option<VirusScanResult> {
        if content.len() < 4 {
            return None;
        }

        // Windows PE executable (MZ header)
        if content.starts_with(b"MZ") {
            return Some(VirusScanResult::high_threat(
                "Windows PE executable detected".to_string(),
            ));
        }

        // Linux ELF executable
        if content.starts_with(b"\x7fELF") {
            return Some(VirusScanResult::high_threat(
                "Linux ELF executable detected".to_string(),
            ));
        }

        // Mach-O (macOS) executable
        let macho_magic = [0xfe, 0xed, 0xfa, 0xce]; // 32-bit
        let macho_magic_64 = [0xfe, 0xed, 0xfa, 0xcf]; // 64-bit
        let macho_magic_rev = [0xce, 0xfa, 0xed, 0xfe]; // reversed
        let macho_magic_64_rev = [0xcf, 0xfa, 0xed, 0xfe]; // 64-bit reversed

        if content.starts_with(&macho_magic)
            || content.starts_with(&macho_magic_64)
            || content.starts_with(&macho_magic_rev)
            || content.starts_with(&macho_magic_64_rev)
        {
            return Some(VirusScanResult::high_threat(
                "macOS Mach-O executable detected".to_string(),
            ));
        }

        // Java class file
        if content.starts_with(b"\xca\xfe\xba\xbe") {
            return Some(VirusScanResult {
                threat_level: ThreatLevel::Medium,
                threats: vec!["Java class file detected".to_string()],
                scan_details: json!({"scan_method": "magic_bytes", "file_type": "java_class"}),
            });
        }

        None
    }

    /// Additional heuristics for suspicious content
    fn check_heuristics(&self, content: &[u8], filename: &str) -> Option<VirusScanResult> {
        // Check for double extensions (e.g., document.pdf.exe)
        let lower_name = filename.to_lowercase();
        let suspicious_double_ext = [
            ".pdf.exe", ".doc.exe", ".xls.exe", ".jpg.exe", ".png.exe",
            ".pdf.scr", ".doc.scr", ".xls.scr", ".jpg.scr", ".png.scr",
        ];

        for ext in &suspicious_double_ext {
            if lower_name.ends_with(ext) {
                return Some(VirusScanResult::blocked(
                    format!("Suspicious double extension: {}", ext),
                ));
            }
        }

        // Check for embedded scripts in HTML/SVG (XSS vectors)
        if lower_name.ends_with(".svg") || lower_name.ends_with(".html") || lower_name.ends_with(".htm") {
            if let Ok(text) = std::str::from_utf8(content) {
                let lower_text = text.to_lowercase();
                if lower_text.contains("<script") || lower_text.contains("javascript:") {
                    return Some(VirusScanResult {
                        threat_level: ThreatLevel::Medium,
                        threats: vec!["Embedded script detected".to_string()],
                        scan_details: json!({"scan_method": "heuristics", "issue": "embedded_script"}),
                    });
                }
            }
        }

        None
    }

    /// Check if the scan result should block the file
    pub fn should_block(&self, result: &VirusScanResult) -> bool {
        matches!(result.threat_level, ThreatLevel::Critical | ThreatLevel::High)
    }

    /// Check if the file should be quarantined (not blocked but flagged)
    pub fn should_quarantine(&self, result: &VirusScanResult) -> bool {
        matches!(result.threat_level, ThreatLevel::Medium)
    }

    /// Add a custom blocked extension
    pub fn add_blocked_extension(&mut self, ext: &'static str) {
        if !self.blocked_extensions.contains(&ext) {
            self.blocked_extensions.push(ext);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blocks_exe_extension() {
        let scanner = VirusScannerService::new();
        let result = scanner.scan(b"some content", "malware.exe");
        assert!(scanner.should_block(&result));
        assert_eq!(result.threat_level, ThreatLevel::Critical);
    }

    #[test]
    fn test_blocks_bat_extension() {
        let scanner = VirusScannerService::new();
        let result = scanner.scan(b"@echo off", "script.bat");
        assert!(scanner.should_block(&result));
    }

    #[test]
    fn test_detects_pe_magic_bytes() {
        let scanner = VirusScannerService::new();
        let pe_content = b"MZ\x90\x00\x03\x00\x00\x00";
        let result = scanner.scan(pe_content, "program.bin");
        assert!(scanner.should_block(&result));
        assert_eq!(result.threat_level, ThreatLevel::High);
    }

    #[test]
    fn test_detects_elf_magic_bytes() {
        let scanner = VirusScannerService::new();
        let elf_content = b"\x7fELF\x02\x01\x01\x00";
        let result = scanner.scan(elf_content, "program");
        assert!(scanner.should_block(&result));
    }

    #[test]
    fn test_allows_safe_files() {
        let scanner = VirusScannerService::new();
        let result = scanner.scan(b"Hello, World!", "document.txt");
        assert!(!scanner.should_block(&result));
        assert!(!scanner.should_quarantine(&result));
        assert_eq!(result.threat_level, ThreatLevel::Safe);
    }

    #[test]
    fn test_detects_double_extension() {
        let scanner = VirusScannerService::new();
        let result = scanner.scan(b"fake pdf content", "document.pdf.exe");
        assert!(scanner.should_block(&result));
    }

    #[test]
    fn test_flags_svg_with_script() {
        let scanner = VirusScannerService::new();
        let svg = b"<svg><script>alert('xss')</script></svg>";
        let result = scanner.scan(svg, "image.svg");
        assert!(scanner.should_quarantine(&result));
        assert_eq!(result.threat_level, ThreatLevel::Medium);
    }
}
