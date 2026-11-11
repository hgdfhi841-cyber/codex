// Modified by the unofficial codex-ar fork (2026): Arabic localization. See CODEX-AR.md.
use codex_app_server_protocol::ConsumeAccountRateLimitResetCreditOutcome;
use codex_app_server_protocol::ConsumeAccountRateLimitResetCreditResponse;
use codex_app_server_protocol::RateLimitResetCreditsSummary;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use uuid::Uuid;

use super::reset_credits::reset_credit_options;
use super::*;

const USAGE_MENU_VIEW_ID: &str = "usage-menu";
const RATE_LIMIT_RESET_VIEW_ID: &str = "rate-limit-reset";
const RATE_LIMIT_RESET_CONFIRMATION_VIEW_ID: &str = "rate-limit-reset-confirmation";

impl ChatWidget {
    pub(super) fn open_usage_menu(&mut self) {
        self.clear_pending_rate_limit_reset_hint();
        let should_refresh_reset_availability = self.available_rate_limit_reset_credits == Some(0);
        self.bottom_pane
            .show_selection_view(self.usage_menu_params());
        if should_refresh_reset_availability {
            let request_id = self.take_next_rate_limit_reset_request_id();
            self.pending_usage_menu_rate_limit_request_id = Some(request_id);
            self.app_event_tx.send(AppEvent::RefreshRateLimits {
                origin: RateLimitRefreshOrigin::UsageMenu { request_id },
            });
        }
        self.request_redraw();
    }

    fn usage_menu_params(&self) -> SelectionViewParams {
        let reset_eligible = self.has_chatgpt_account;
        let (reset_action_enabled, reset_description) =
            match (reset_eligible, self.available_rate_limit_reset_credits) {
                (true, Some(available_count)) if available_count > 0 => {
                    (true, format!("المتاح: {available_count}"))
                }
                (true, None) => (true, "التحقق من التوفر".to_string()),
                (true, Some(_)) | (false, _) => (false, "لا شيء متاح".to_string()),
            };

        SelectionViewParams {
            view_id: Some(USAGE_MENU_VIEW_ID),
            title: Some("الاستهلاك".to_string()),
            subtitle: Some("استهلاك الحساب وإعادات الضبط.".to_string()),
            footer_hint: Some(usage_hint_line(&self.bottom_pane.list_keymap(), "open")),
            items: vec![
                SelectionItem {
                    name: "عرض التحليلات".to_string(),
                    description: Some("سجل الاستهلاك".to_string()),
                    actions: vec![Box::new(|tx| {
                        tx.send(AppEvent::OpenAnalytics { view: None });
                    })],
                    dismiss_on_select: true,
                    ..Default::default()
                },
                SelectionItem {
                    name: "استخدام إعادة ضبط".to_string(),
                    description: Some(reset_description),
                    is_disabled: !reset_action_enabled,
                    actions: vec![Box::new(|tx| {
                        tx.send(AppEvent::OpenRateLimitResetCredits);
                    })],
                    dismiss_on_select: true,
                    ..Default::default()
                },
            ],
            ..SelectionViewParams::picker()
        }
    }

    pub(crate) fn finish_usage_menu_rate_limit_refresh(
        &mut self,
        request_id: u64,
        snapshots: Vec<RateLimitSnapshot>,
        result: Result<RateLimitResetCreditsSummary, String>,
    ) {
        if self.pending_usage_menu_rate_limit_request_id != Some(request_id) {
            return;
        }
        self.pending_usage_menu_rate_limit_request_id = None;
        for snapshot in snapshots {
            self.on_rate_limit_snapshot(Some(snapshot));
        }
        if let Ok(response) = result {
            self.available_rate_limit_reset_credits = Some(response.available_count);
        }
        let params = self.usage_menu_params();
        if self
            .bottom_pane
            .replace_selection_view_if_present(USAGE_MENU_VIEW_ID, params)
        {
            self.request_redraw();
        }
    }

    pub(crate) fn show_rate_limit_reset_loading_popup(&mut self) -> u64 {
        self.clear_pending_rate_limit_reset_hint();
        self.pending_rate_limit_reset_idempotency_key = None;
        self.rate_limit_reset_picker_request_id = None;
        let request_id = self.take_next_rate_limit_reset_request_id();
        self.pending_rate_limit_reset_request_id = Some(request_id);
        self.bottom_pane.show_selection_view(SelectionViewParams {
            view_id: Some(RATE_LIMIT_RESET_VIEW_ID),
            title: Some("إعادات ضبط حد الاستخدام".to_string()),
            subtitle: Some("جارٍ التحقق من إعادات الضبط المتاحة لك...".to_string()),
            items: vec![SelectionItem {
                name: "جارٍ التحميل...".to_string(),
                is_disabled: true,
                ..Default::default()
            }],
            ..SelectionViewParams::picker()
        });
        self.request_redraw();
        request_id
    }

    pub(crate) fn finish_rate_limit_reset_credits_refresh(
        &mut self,
        request_id: u64,
        snapshots: Vec<RateLimitSnapshot>,
        result: Result<RateLimitResetCreditsSummary, String>,
    ) -> bool {
        if self.pending_rate_limit_reset_request_id != Some(request_id) {
            return false;
        }
        self.pending_rate_limit_reset_request_id = None;
        for snapshot in snapshots {
            self.on_rate_limit_snapshot(Some(snapshot));
        }

        let mut shows_picker = false;
        let params = match result {
            Ok(response) => {
                let available_count = response.available_count;
                let params = if available_count > 0 {
                    shows_picker = true;
                    self.rate_limit_reset_picker_params(request_id, &response)
                } else {
                    Self::rate_limit_reset_message_params(
                        "لا توجد لديك إعادات ضبط متاحة لحد الاستخدام.",
                    )
                };
                self.available_rate_limit_reset_credits = Some(available_count);
                params
            }
            Err(_) => {
                Self::reset_refresh_params("تعذّر تحميل إعادات ضبط حد الاستخدام. حاول مجدداً.")
            }
        };
        let replaced = self
            .bottom_pane
            .replace_selection_view_if_present(RATE_LIMIT_RESET_VIEW_ID, params);
        self.rate_limit_reset_picker_request_id = (replaced && shows_picker).then_some(request_id);
        if replaced {
            self.request_redraw();
        }
        replaced
    }

    fn rate_limit_reset_picker_params(
        &self,
        picker_request_id: u64,
        reset_credits: &RateLimitResetCreditsSummary,
    ) -> SelectionViewParams {
        let confirmation_gate = Arc::new(AtomicBool::new(true));
        let mut items = reset_credit_options(reset_credits, self.clock_format)
            .into_iter()
            .map(|option| {
                let confirmation_gate = confirmation_gate.clone();
                let credit_id = option.credit_id;
                let reset_title = option.name.clone();
                let reset_detail = option.detail;
                let reset_description = option.description;
                let picker_description = reset_detail
                    .clone()
                    .unwrap_or_else(|| reset_description.clone());
                SelectionItem {
                    name: option.name,
                    description: Some(picker_description),
                    actions: vec![Box::new(move |tx| {
                        if confirmation_gate.swap(false, Ordering::AcqRel) {
                            tx.send(AppEvent::OpenRateLimitResetConfirmation {
                                picker_request_id,
                                confirmation_gate: confirmation_gate.clone(),
                                credit_id: credit_id.clone(),
                                reset_title: reset_title.clone(),
                                reset_detail: reset_detail.clone(),
                                reset_description: reset_description.clone(),
                            });
                        }
                    })],
                    ..Default::default()
                }
            })
            .collect::<Vec<_>>();
        items.push(SelectionItem {
            name: "إلغاء".to_string(),
            dismiss_on_select: true,
            ..Default::default()
        });
        SelectionViewParams {
            view_id: Some(RATE_LIMIT_RESET_VIEW_ID),
            title: Some("إعادات ضبط حد الاستخدام".to_string()),
            subtitle: Some(format!(
                "{} {} available.",
                reset_credits.available_count,
                reset_label(reset_credits.available_count)
            )),
            footer_hint: Some(usage_hint_line(&self.bottom_pane.list_keymap(), "choose")),
            items,
            initial_selected_idx: Some(0),
            ..SelectionViewParams::picker()
        }
    }

    pub(crate) fn show_rate_limit_reset_confirmation(
        &mut self,
        picker_request_id: u64,
        confirmation_gate: Arc<AtomicBool>,
        credit_id: Option<String>,
        reset_title: String,
        reset_detail: Option<String>,
        reset_description: String,
    ) -> bool {
        if self.rate_limit_reset_picker_request_id != Some(picker_request_id)
            || self
                .bottom_pane
                .selected_index_for_active_view(RATE_LIMIT_RESET_VIEW_ID)
                .is_none()
        {
            confirmation_gate.store(true, Ordering::Release);
            return false;
        }
        let idempotency_key = Uuid::new_v4().to_string();
        self.pending_rate_limit_reset_idempotency_key = Some(idempotency_key.clone());
        let no_confirmation_gate = confirmation_gate.clone();
        let subtitle = reset_detail.map_or_else(
            || reset_title.clone(),
            |reset_detail| format!("{reset_title} · {reset_detail}"),
        );
        self.bottom_pane.show_selection_view(SelectionViewParams {
            view_id: Some(RATE_LIMIT_RESET_CONFIRMATION_VIEW_ID),
            title: Some("استخدام إعادة الضبط هذه؟".to_string()),
            subtitle: Some(subtitle),
            footer_hint: Some(usage_hint_line(&self.bottom_pane.list_keymap(), "confirm")),
            items: vec![
                SelectionItem {
                    name: "نعم، استخدمها".to_string(),
                    description: Some(reset_description),
                    actions: vec![Box::new(move |tx| {
                        tx.send(AppEvent::ConsumeRateLimitResetCredit {
                            idempotency_key: idempotency_key.clone(),
                            credit_id: credit_id.clone(),
                        });
                    })],
                    dismiss_on_select: true,
                    ..Default::default()
                },
                SelectionItem {
                    name: "لا، ارجع".to_string(),
                    description: Some("اختيار إعادة ضبط أخرى".to_string()),
                    actions: vec![Box::new(move |_| {
                        no_confirmation_gate.store(true, Ordering::Release);
                    })],
                    dismiss_on_select: true,
                    ..Default::default()
                },
            ],
            initial_selected_idx: Some(1),
            on_cancel: Some(Box::new(move |_| {
                confirmation_gate.store(true, Ordering::Release);
            })),
            ..SelectionViewParams::confirmation()
        });
        true
    }

    pub(crate) fn start_rate_limit_reset_consumption(
        &mut self,
        idempotency_key: &str,
    ) -> Option<u64> {
        if self.pending_rate_limit_reset_idempotency_key.as_deref() != Some(idempotency_key) {
            return None;
        }
        Some(self.show_rate_limit_reset_consuming_popup())
    }

    fn rate_limit_reset_message_params(message: &str) -> SelectionViewParams {
        SelectionViewParams {
            view_id: Some(RATE_LIMIT_RESET_VIEW_ID),
            title: Some("إعادات ضبط حد الاستخدام".to_string()),
            subtitle: Some(message.to_string()),
            items: vec![SelectionItem {
                name: "إغلاق".to_string(),
                dismiss_on_select: true,
                ..Default::default()
            }],
            ..SelectionViewParams::picker()
        }
    }

    fn reset_refresh_params(message: &str) -> SelectionViewParams {
        SelectionViewParams {
            view_id: Some(RATE_LIMIT_RESET_VIEW_ID),
            title: Some("إعادات ضبط حد الاستخدام".to_string()),
            subtitle: Some(message.to_string()),
            items: vec![
                SelectionItem {
                    name: "حاول مجدداً".to_string(),
                    actions: vec![Box::new(|tx| {
                        tx.send(AppEvent::OpenRateLimitResetCredits);
                    })],
                    dismiss_on_select: true,
                    ..Default::default()
                },
                SelectionItem {
                    name: "إغلاق".to_string(),
                    dismiss_on_select: true,
                    ..Default::default()
                },
            ],
            ..SelectionViewParams::picker()
        }
    }

    pub(crate) fn show_rate_limit_reset_consuming_popup(&mut self) -> u64 {
        self.clear_pending_rate_limit_reset_hint();
        self.pending_rate_limit_reset_idempotency_key = None;
        self.rate_limit_reset_picker_request_id = None;
        let request_id = self.take_next_rate_limit_reset_request_id();
        self.pending_rate_limit_reset_request_id = Some(request_id);
        self.bottom_pane
            .dismiss_view_by_id(RATE_LIMIT_RESET_CONFIRMATION_VIEW_ID);
        self.bottom_pane
            .dismiss_view_by_id(RATE_LIMIT_RESET_VIEW_ID);
        self.bottom_pane.show_selection_view(SelectionViewParams {
            view_id: Some(RATE_LIMIT_RESET_VIEW_ID),
            title: Some("إعادات ضبط حد الاستخدام".to_string()),
            subtitle: Some("جارٍ إعادة ضبط استهلاكك...".to_string()),
            items: vec![SelectionItem {
                name: "جارٍ استخدام إعادة ضبط...".to_string(),
                is_disabled: true,
                ..Default::default()
            }],
            allow_cancel: false,
            ..SelectionViewParams::picker()
        });
        self.request_redraw();
        request_id
    }

    pub(crate) fn finish_rate_limit_reset_consume(
        &mut self,
        request_id: u64,
        idempotency_key: String,
        credit_id: Option<String>,
        result: Result<ConsumeAccountRateLimitResetCreditResponse, String>,
    ) -> bool {
        if self.pending_rate_limit_reset_request_id != Some(request_id) {
            return false;
        }

        match result {
            Ok(response)
                if matches!(
                    response.outcome,
                    ConsumeAccountRateLimitResetCreditOutcome::Reset
                        | ConsumeAccountRateLimitResetCreditOutcome::AlreadyRedeemed
                ) =>
            {
                self.available_rate_limit_reset_credits = None;
                self.replace_rate_limit_reset_popup(Self::rate_limit_reset_success_loading_params());
                true
            }
            Ok(response) => {
                self.pending_rate_limit_reset_request_id = None;
                let message = match response.outcome {
                    ConsumeAccountRateLimitResetCreditOutcome::NothingToReset => {
                        "استهلاكك لا يحتاج إعادة ضبط الآن."
                    }
                    ConsumeAccountRateLimitResetCreditOutcome::NoCredit if credit_id.is_some() => {
                        self.available_rate_limit_reset_credits = None;
                        self.replace_rate_limit_reset_popup(Self::reset_refresh_params(
                            "إعادة الضبط هذه لم تعد متاحة. حدّث لرؤية المتاح حالياً.",
                        ));
                        return false;
                    }
                    ConsumeAccountRateLimitResetCreditOutcome::NoCredit => {
                        self.available_rate_limit_reset_credits = Some(0);
                        "لا توجد إعادات ضبط متاحة لحد الاستخدام."
                    }
                    ConsumeAccountRateLimitResetCreditOutcome::Reset
                    | ConsumeAccountRateLimitResetCreditOutcome::AlreadyRedeemed => unreachable!(),
                };
                self.replace_rate_limit_reset_popup(Self::rate_limit_reset_message_params(message));
                false
            }
            Err(_) => {
                self.pending_rate_limit_reset_request_id = None;
                self.pending_rate_limit_reset_idempotency_key = Some(idempotency_key.clone());
                self.replace_rate_limit_reset_popup(SelectionViewParams {
                    view_id: Some(RATE_LIMIT_RESET_VIEW_ID),
                    title: Some("إعادات ضبط حد الاستخدام".to_string()),
                    subtitle: Some("تعذّرت إعادة ضبط الاستهلاك. حاول مجدداً.".to_string()),
                    items: vec![
                        SelectionItem {
                            name: "حاول مجدداً".to_string(),
                            actions: vec![Box::new(move |tx| {
                                tx.send(AppEvent::ConsumeRateLimitResetCredit {
                                    idempotency_key: idempotency_key.clone(),
                                    credit_id: credit_id.clone(),
                                });
                            })],
                            dismiss_on_select: true,
                            ..Default::default()
                        },
                        SelectionItem {
                            name: "إغلاق".to_string(),
                            dismiss_on_select: true,
                            ..Default::default()
                        },
                    ],
                    ..SelectionViewParams::picker()
                });
                false
            }
        }
    }

    pub(crate) fn finish_post_consume_reset_credits_refresh(
        &mut self,
        request_id: u64,
        snapshots: Vec<RateLimitSnapshot>,
        result: Result<RateLimitResetCreditsSummary, String>,
    ) -> bool {
        if self.pending_rate_limit_reset_request_id != Some(request_id) {
            return false;
        }
        self.pending_rate_limit_reset_request_id = None;
        for snapshot in snapshots {
            self.on_rate_limit_snapshot(Some(snapshot));
        }

        let message = match result {
            Ok(response) => {
                let available_count = response.available_count;
                self.available_rate_limit_reset_credits = Some(available_count);
                format!(
                    "أُعيد ضبط الاستهلاك. بقي لديك {available_count} {}.",
                    reset_label(available_count)
                )
            }
            Err(_) => "أُعيد ضبط الاستهلاك.".to_string(),
        };
        self.replace_rate_limit_reset_popup(Self::rate_limit_reset_message_params(&message));
        true
    }

    fn rate_limit_reset_success_loading_params() -> SelectionViewParams {
        SelectionViewParams {
            view_id: Some(RATE_LIMIT_RESET_VIEW_ID),
            title: Some("إعادات ضبط حد الاستخدام".to_string()),
            subtitle: Some("أُعيد ضبط الاستهلاك. جارٍ التحقق مما بقي لك...".to_string()),
            items: vec![SelectionItem {
                name: "جارٍ التحديث...".to_string(),
                is_disabled: true,
                ..Default::default()
            }],
            allow_cancel: false,
            ..SelectionViewParams::picker()
        }
    }

    fn replace_rate_limit_reset_popup(&mut self, params: SelectionViewParams) {
        if self
            .bottom_pane
            .replace_selection_view_if_present(RATE_LIMIT_RESET_VIEW_ID, params)
        {
            self.request_redraw();
        }
    }

    pub(crate) fn start_rate_limit_reset_startup_check(&mut self) -> u64 {
        self.clear_pending_rate_limit_reset_hint();
        let request_id = self.take_next_rate_limit_reset_request_id();
        self.pending_rate_limit_reset_hint_request_id = Some(request_id);
        request_id
    }

    pub(crate) fn finish_rate_limit_reset_hint_refresh(
        &mut self,
        request_id: u64,
        snapshots: Vec<RateLimitSnapshot>,
        result: Result<RateLimitResetCreditsSummary, String>,
    ) -> bool {
        if self.pending_rate_limit_reset_hint_request_id != Some(request_id) {
            return false;
        }
        self.pending_rate_limit_reset_hint_request_id = None;
        for snapshot in snapshots {
            self.on_rate_limit_snapshot(Some(snapshot));
        }
        if !self.has_codex_backend_auth {
            return false;
        }
        if let Ok(response) = result {
            let available_count = response.available_count;
            self.available_rate_limit_reset_credits = Some(available_count);
            self.set_rate_limit_reset_available_hint(available_count);
        }
        true
    }

    pub(crate) fn clear_pending_rate_limit_reset_requests(&mut self) {
        self.pending_rate_limit_reset_request_id = None;
        self.pending_rate_limit_reset_idempotency_key = None;
        self.rate_limit_reset_picker_request_id = None;
        self.pending_usage_menu_rate_limit_request_id = None;
        self.available_rate_limit_reset_credits = None;
        self.rate_limit_snapshots_by_limit_id.clear();
        self.clear_pending_rate_limit_reset_hint();
        self.bottom_pane.dismiss_view_by_id(USAGE_MENU_VIEW_ID);
        self.bottom_pane
            .dismiss_view_by_id(RATE_LIMIT_RESET_VIEW_ID);
        self.bottom_pane
            .dismiss_view_by_id(RATE_LIMIT_RESET_CONFIRMATION_VIEW_ID);
    }

    pub(crate) fn clear_pending_rate_limit_reset_hint(&mut self) {
        self.pending_rate_limit_reset_hint_request_id = None;
        let cleared_hint = self.pending_rate_limit_reset_hint.take().is_some();
        if cleared_hint {
            self.bump_active_cell_revision();
            self.request_redraw();
        }
    }

    pub(super) fn pending_rate_limit_reset_hint(&self) -> Option<&PlainHistoryCell> {
        self.pending_rate_limit_reset_hint.as_ref()
    }

    pub(crate) fn take_pending_rate_limit_reset_hint(&mut self) -> Option<PlainHistoryCell> {
        let hint = self.pending_rate_limit_reset_hint.take()?;
        self.bump_active_cell_revision();
        Some(hint)
    }

    fn set_rate_limit_reset_available_hint(&mut self, available_count: i64) {
        if available_count <= 0 {
            return;
        }
        self.pending_rate_limit_reset_hint = Some(history_cell::new_info_event(
            format!(
                "لديك {available_count} {} متاحة. شغّل /usage لاستخدام واحدة.",
                reset_label(available_count)
            ),
            /*hint*/ None,
        ));
        self.bump_active_cell_revision();
        self.request_redraw();
    }

    fn take_next_rate_limit_reset_request_id(&mut self) -> u64 {
        let request_id = self.next_rate_limit_reset_request_id;
        self.next_rate_limit_reset_request_id = self
            .next_rate_limit_reset_request_id
            .wrapping_add(/*rhs*/ 1);
        request_id
    }
}

/// Keep usage actions readable on narrow terminals and honor customized list bindings.
fn usage_hint_line(
    keymap: &crate::keymap::ListKeymap,
    accept_label: &'static str,
) -> Line<'static> {
    let mut spans = Vec::new();
    for (action, label) in [
        (crate::keymap::ListAction::Accept, accept_label),
        (crate::keymap::ListAction::Cancel, "back"),
    ] {
        if let Some(hint) = keymap.primary_hint(action) {
            if !spans.is_empty() {
                spans.push(" · ".into());
            }
            spans.extend(hint.spans());
            spans.push(format!(" {label}").into());
        }
    }
    Line::from(spans)
}

fn reset_label(count: i64) -> &'static str {
    if count == 1 {
        "إعادة ضبط للحد"
    } else {
        "إعادات ضبط للحد"
    }
}
