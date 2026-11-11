// Modified by the unofficial codex-ar fork (2026): Arabic localization. See CODEX-AR.md.
//! Approval, denial, and review-status transcript cells.

use super::*;
use crate::style::accent_color;

fn truncate_exec_snippet(full_cmd: &str) -> String {
    let mut snippet = match full_cmd.split_once('\n') {
        Some((first, _)) => format!("{first} ..."),
        None => full_cmd.to_string(),
    };
    snippet = truncate_text(&snippet, /*max_graphemes*/ 80);
    snippet
}

fn exec_snippet(command: &[String]) -> String {
    let full_cmd = strip_bash_lc_and_escape(command);
    truncate_exec_snippet(&full_cmd)
}

fn non_empty_exec_snippet(command: &[String]) -> Option<String> {
    let snippet = exec_snippet(command);
    (!snippet.is_empty()).then_some(snippet)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum ReviewDecision {
    Approved,
    ApprovedExecpolicyAmendment {
        proposed_execpolicy_amendment: ExecPolicyAmendment,
    },
    ApprovedForSession,
    NetworkPolicyAmendment {
        network_policy_amendment: NetworkPolicyAmendment,
    },
    Denied,
    TimedOut,
    Abort,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum ApprovalDecisionSubject {
    Command(Vec<String>),
    NetworkAccess { target: String },
}

pub fn new_approval_decision_cell(
    subject: ApprovalDecisionSubject,
    decision: ReviewDecision,
    actor: ApprovalDecisionActor,
) -> Box<dyn HistoryCell> {
    use ReviewDecision::*;
    use codex_protocol::approvals::NetworkPolicyRuleAction;

    let (symbol, summary): (Span<'static>, Vec<Span<'static>>) = match decision {
        Approved => match subject {
            ApprovalDecisionSubject::Command(command) => {
                let summary = if let Some(snippet) = non_empty_exec_snippet(&command) {
                    vec![
                        actor.subject().into(),
                        "تمت الموافقة على".bold(),
                        " تشغيل Codex للأمر ".into(),
                        Span::from(snippet).dim(),
                        " هذه المرة".bold(),
                    ]
                } else {
                    vec![
                        actor.subject().into(),
                        "تمت الموافقة على".bold(),
                        " هذا الطلب".into(),
                        " هذه المرة".bold(),
                    ]
                };
                ("✔ ".green(), summary)
            }
            ApprovalDecisionSubject::NetworkAccess { target } => (
                "✔ ".green(),
                vec![
                    actor.subject().into(),
                    "تمت الموافقة على".bold(),
                    " وصول Codex عبر الشبكة إلى ".into(),
                    Span::from(target).dim(),
                    " هذه المرة".bold(),
                ],
            ),
        },
        ApprovedExecpolicyAmendment {
            proposed_execpolicy_amendment,
        } => {
            let snippet = Span::from(exec_snippet(&proposed_execpolicy_amendment.command)).dim();
            (
                "✔ ".green(),
                vec![
                    actor.subject().into(),
                    "تمت الموافقة على".bold(),
                    " تشغيل Codex دائماً للأوامر التي تبدأ بـ ".into(),
                    snippet,
                ],
            )
        }
        ApprovedForSession => match subject {
            ApprovalDecisionSubject::Command(command) => {
                let summary = if let Some(snippet) = non_empty_exec_snippet(&command) {
                    vec![
                        actor.subject().into(),
                        "تمت الموافقة على".bold(),
                        " تشغيل Codex للأمر ".into(),
                        Span::from(snippet).dim(),
                        " طوال هذه الجلسة".bold(),
                    ]
                } else {
                    vec![
                        actor.subject().into(),
                        "تمت الموافقة على".bold(),
                        " هذا الطلب".into(),
                        " طوال هذه الجلسة".bold(),
                    ]
                };
                ("✔ ".green(), summary)
            }
            ApprovalDecisionSubject::NetworkAccess { target } => (
                "✔ ".green(),
                vec![
                    actor.subject().into(),
                    "تمت الموافقة على".bold(),
                    " وصول Codex عبر الشبكة إلى ".into(),
                    Span::from(target).dim(),
                    " طوال هذه الجلسة".bold(),
                ],
            ),
        },
        NetworkPolicyAmendment {
            network_policy_amendment,
        } => {
            let target = match subject {
                ApprovalDecisionSubject::NetworkAccess { target } => target,
                ApprovalDecisionSubject::Command(_) => network_policy_amendment.host,
            };
            match network_policy_amendment.action {
                NetworkPolicyRuleAction::Allow => (
                    "✔ ".green(),
                    vec![
                        actor.subject().into(),
                        "تم حفظ السماح بـ".bold(),
                        " وصول Codex عبر الشبكة إلى ".into(),
                        Span::from(target).dim(),
                    ],
                ),
                NetworkPolicyRuleAction::Deny => (
                    "✗ ".red(),
                    vec![
                        actor.subject().into(),
                        "رُفض".bold(),
                        " وصول Codex عبر الشبكة إلى ".into(),
                        Span::from(target).dim(),
                        " وحُفظت هذه القاعدة".into(),
                    ],
                ),
            }
        }
        Denied => match subject {
            ApprovalDecisionSubject::Command(command) => {
                let summary = if let Some(snippet) = non_empty_exec_snippet(&command) {
                    let snippet = Span::from(snippet).dim();
                    match actor {
                        ApprovalDecisionActor::User => vec![
                            actor.subject().into(),
                            "لم تتم الموافقة على".bold(),
                            " تشغيل Codex للأمر ".into(),
                            snippet,
                        ],
                        ApprovalDecisionActor::Guardian => vec![
                            "الطلب: ".into(),
                            "رُفض".bold(),
                            " تشغيل Codex للأمر ".into(),
                            snippet,
                        ],
                    }
                } else {
                    match actor {
                        ApprovalDecisionActor::User => vec![
                            actor.subject().into(),
                            "لم تتم الموافقة على".bold(),
                            " هذا الطلب".into(),
                        ],
                        ApprovalDecisionActor::Guardian => {
                            vec!["الطلب: ".into(), "رُفض".bold()]
                        }
                    }
                };
                ("✗ ".red(), summary)
            }
            ApprovalDecisionSubject::NetworkAccess { target } => (
                "✗ ".red(),
                vec![
                    actor.subject().into(),
                    "لم تتم الموافقة على".bold(),
                    " وصول Codex عبر الشبكة إلى ".into(),
                    Span::from(target).dim(),
                ],
            ),
        },
        TimedOut => match subject {
            ApprovalDecisionSubject::Command(command) => {
                let summary = if let Some(snippet) = non_empty_exec_snippet(&command) {
                    vec![
                        "المراجعة: ".into(),
                        "انتهت مهلتها".bold(),
                        " قبل أن يشغّل Codex الأمر ".into(),
                        Span::from(snippet).dim(),
                    ]
                } else {
                    vec![
                        "المراجعة: ".into(),
                        "انتهت مهلتها".bold(),
                        " قبل الموافقة على هذا الطلب".into(),
                    ]
                };
                ("✗ ".red(), summary)
            }
            ApprovalDecisionSubject::NetworkAccess { target } => (
                "✗ ".red(),
                vec![
                    "المراجعة: ".into(),
                    "انتهت مهلتها".bold(),
                    " قبل أن يصل Codex إلى ".into(),
                    Span::from(target).dim(),
                ],
            ),
        },
        Abort => match subject {
            ApprovalDecisionSubject::Command(command) => {
                let summary = if let Some(snippet) = non_empty_exec_snippet(&command) {
                    vec![
                        actor.subject().into(),
                        "أُلغي".bold(),
                        " طلب تشغيل ".into(),
                        Span::from(snippet).dim(),
                    ]
                } else {
                    vec![
                        actor.subject().into(),
                        "أُلغي".bold(),
                        " هذا الطلب".into(),
                    ]
                };
                ("✗ ".red(), summary)
            }
            ApprovalDecisionSubject::NetworkAccess { target } => (
                "✗ ".red(),
                vec![
                    actor.subject().into(),
                    "أُلغي".bold(),
                    " طلب وصول Codex عبر الشبكة إلى ".into(),
                    Span::from(target).dim(),
                ],
            ),
        },
    };

    Box::new(PrefixedWrappedHistoryCell::new(
        Line::from(summary),
        symbol,
        "  ",
    ))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ApprovalDecisionActor {
    User,
    Guardian,
}

impl ApprovalDecisionActor {
    fn subject(self) -> &'static str {
        match self {
            Self::User => "أنت: ",
            Self::Guardian => "المراجع الآلي: ",
        }
    }
}

pub fn new_guardian_denied_patch_request(files: Vec<String>) -> Box<dyn HistoryCell> {
    let mut summary = vec![
        "الطلب: ".into(),
        "رُفض".bold(),
        " تطبيق Codex ".into(),
    ];
    if files.len() == 1 {
        summary.push("تعديلاً يمسّ ".into());
        summary.push(Span::from(files[0].clone()).dim());
    } else {
        summary.push("تعديلاً يمسّ ".into());
        summary.push(Span::from(files.len().to_string()).dim());
        summary.push(" ملفات".into());
    }

    Box::new(PrefixedWrappedHistoryCell::new(
        Line::from(summary),
        "✗ ".red(),
        "  ",
    ))
}

pub fn new_guardian_denied_action_request(summary: String) -> Box<dyn HistoryCell> {
    let line = Line::from(vec![
        "الطلب: ".into(),
        "رُفض".bold(),
        " بخصوص: ".into(),
        Span::from(summary).dim(),
    ]);
    Box::new(PrefixedWrappedHistoryCell::new(line, "✗ ".red(), "  "))
}

pub fn new_guardian_timed_out_patch_request(files: Vec<String>) -> Box<dyn HistoryCell> {
    let mut summary = vec![
        "المراجعة: ".into(),
        "انتهت مهلتها".bold(),
        " قبل أن يطبّق Codex ".into(),
    ];
    if files.len() == 1 {
        summary.push("تعديلاً يمسّ ".into());
        summary.push(Span::from(files[0].clone()).dim());
    } else {
        summary.push("تعديلاً يمسّ ".into());
        summary.push(Span::from(files.len().to_string()).dim());
        summary.push(" ملفات".into());
    }

    Box::new(PrefixedWrappedHistoryCell::new(
        Line::from(summary),
        "✗ ".red(),
        "  ",
    ))
}

pub fn new_guardian_timed_out_action_request(summary: String) -> Box<dyn HistoryCell> {
    let line = Line::from(vec![
        "المراجعة: ".into(),
        "انتهت مهلتها".bold(),
        " قبل: ".into(),
        Span::from(summary).dim(),
    ]);
    Box::new(PrefixedWrappedHistoryCell::new(line, "✗ ".red(), "  "))
}

/// Accented history cell line showing the current review status.
pub(crate) fn new_review_status_line(message: String) -> PlainHistoryCell {
    PlainHistoryCell {
        lines: vec![Line::from(message.fg(accent_color()))],
    }
}
