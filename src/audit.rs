use tracing::info;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum AuditEvent {
    LoginStarted,
    LoginSucceeded,
    LoginFailed,
    LoginRejected,
    LoginUnavailable,
    Logout,
    ProtectedAccessDenied,
    ExternalSourceUnavailable,
}

impl AuditEvent {
    const fn code(self) -> &'static str {
        match self {
            Self::LoginStarted => "auth.login_started",
            Self::LoginSucceeded => "auth.login_succeeded",
            Self::LoginFailed => "auth.login_failed",
            Self::LoginRejected => "auth.login_rejected",
            Self::LoginUnavailable => "auth.login_unavailable",
            Self::Logout => "auth.logout",
            Self::ProtectedAccessDenied => "auth.access_denied",
            Self::ExternalSourceUnavailable => "integration.source_unavailable",
        }
    }
}

pub(crate) fn record(event: AuditEvent, subject: Option<&str>) {
    match subject {
        Some(subject) => info!(target: "audit", event = event.code(), subject = subject),
        None => info!(target: "audit", event = event.code()),
    }
}

#[cfg(test)]
mod tests {
    use super::AuditEvent;

    #[test]
    fn event_codes_are_stable_and_non_sensitive() {
        assert_eq!(AuditEvent::LoginStarted.code(), "auth.login_started");
        assert_eq!(AuditEvent::LoginRejected.code(), "auth.login_rejected");
        assert_eq!(
            AuditEvent::LoginUnavailable.code(),
            "auth.login_unavailable"
        );
        assert_eq!(
            AuditEvent::ExternalSourceUnavailable.code(),
            "integration.source_unavailable"
        );
    }
}
