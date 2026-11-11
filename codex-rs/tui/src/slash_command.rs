// Modified by the unofficial codex-ar fork (2026): Arabic localization. See CODEX-AR.md.
use strum::IntoEnumIterator;
use strum_macros::AsRefStr;
use strum_macros::EnumIter;
use strum_macros::EnumString;
use strum_macros::IntoStaticStr;

/// Commands that can be invoked by starting a message with a leading slash.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, EnumString, EnumIter, AsRefStr, IntoStaticStr,
)]
#[strum(serialize_all = "kebab-case")]
pub enum SlashCommand {
    // DO NOT ALPHA-SORT! Enum order is presentation order in the popup, so
    // more frequently used commands should be listed first.
    Model,
    Presets,
    Daybreak,
    Ide,
    Permissions,
    Keymap,
    Vim,
    #[strum(serialize = "setup-default-sandbox")]
    ElevateSandbox,
    Experimental,
    #[strum(to_string = "approve")]
    AutoReview,
    Memories,
    Skills,
    Import,
    Hooks,
    Review,
    Rename,
    New,
    Archive,
    Delete,
    Resume,
    Fork,
    Worktree,
    App,
    Init,
    Compact,
    Config,
    Recap,
    Plan,
    Voice,
    Goal,
    Agents,
    Side,
    Btw,
    Copy,
    Export,
    Raw,
    Tui,
    Diff,
    Mention,
    Status,
    Daemon,
    Warnings,
    Cd,
    #[strum(to_string = "pwd", serialize = "cwd")]
    Pwd,
    Usage,
    DebugConfig,
    Title,
    Statusline,
    Theme,
    #[strum(to_string = "pets", serialize = "pet")]
    Pets,
    Mcp,
    Apps,
    Plugins,
    Logout,
    Quit,
    Exit,
    Feedback,
    Rollout,
    Ps,
    #[strum(to_string = "stop", serialize = "clean")]
    Stop,
    Clear,
    TestApproval,
    #[strum(serialize = "subagents")]
    MultiAgents,
    // Debugging commands.
    #[strum(serialize = "debug-m-drop")]
    MemoryDrop,
    #[strum(serialize = "debug-m-update")]
    MemoryUpdate,
}

impl SlashCommand {
    /// User-visible description shown in the popup.
    pub fn description(self) -> &'static str {
        match self {
            SlashCommand::Config => "ضبط تفضيلات Codex",
            SlashCommand::Feedback => "إرسال السجلات للمطورين أو حفظها كملف ZIP",
            SlashCommand::New => "بدء محادثة جديدة أثناء المحادثة الحالية",
            SlashCommand::Init => "إنشاء ملف AGENTS.md يحوي تعليمات لـ Codex",
            SlashCommand::Compact => "تلخيص المحادثة لتجنب بلوغ حد السياق",
            SlashCommand::Recap => "تلخيص المحادثة الحالية الآن",
            SlashCommand::Review => "مراجعة تغييراتي الحالية والبحث عن المشاكل",
            SlashCommand::Rename => "إعادة تسمية المحادثة الحالية",
            SlashCommand::Resume => "استئناف محادثة محفوظة",
            SlashCommand::Archive => "أرشفة هذه الجلسة",
            SlashCommand::Delete => "حذف هذه الجلسة نهائياً",
            SlashCommand::Clear => "مسح الشاشة وبدء محادثة جديدة",
            SlashCommand::Fork => "تفريع المحادثة الحالية",
            SlashCommand::Worktree => "بدء أو متابعة محادثة في شجرة عمل (worktree) جديدة",
            SlashCommand::App => "متابعة هذه الجلسة في تطبيق سطح المكتب",
            SlashCommand::Quit | SlashCommand::Exit => "الخروج من Codex",
            SlashCommand::Copy => "نسخ آخر رد أو جزء منه",
            SlashCommand::Export => "تصدير المحادثة بصيغة Markdown",
            SlashCommand::Raw => "تبديل وضع العرض الخام لتسهيل تحديد النص ونسخه",
            SlashCommand::Tui => "اختيار وضع الواجهة للتشغيل القادم",
            SlashCommand::Diff => "عرض فروقات git (بما فيها الملفات غير المتتبَّعة)",
            SlashCommand::Mention => "الإشارة إلى ملف",
            SlashCommand::Skills => "استخدام المهارات لتحسين أداء Codex في مهام محددة",
            SlashCommand::Import => "استيراد الإعدادات وهذا المشروع والمحادثات الأخيرة من Claude Code",
            SlashCommand::Hooks => "عرض وإدارة الخطافات (hooks)",
            SlashCommand::Daemon => "إدارة الخادم المحلي العامل في الخلفية",
            SlashCommand::Warnings => "عرض التحذيرات المحفوظة وتفاصيل التشخيص",
            SlashCommand::Status => "عرض إعدادات الجلسة الحالية واستهلاك الرموز (tokens)",
            SlashCommand::Cd => "تغيير مجلد العمل الحالي",
            SlashCommand::Pwd => "عرض مجلد العمل الحالي",
            SlashCommand::Usage => "عرض استهلاك الحساب أو استخدام إعادة ضبط حد الاستهلاك",
            SlashCommand::DebugConfig => "عرض طبقات الإعدادات ومصادر المتطلبات لتتبع الأخطاء",
            SlashCommand::Title => "اختيار العناصر الظاهرة في عنوان الطرفية",
            SlashCommand::Statusline => "اختيار العناصر الظاهرة في شريط الحالة",
            SlashCommand::Theme => "اختيار سمة تلوين الكود",
            SlashCommand::Pets => "اختيار أو إخفاء رفيق الطرفية",
            SlashCommand::Ps => "عرض الطرفيات العاملة في الخلفية",
            SlashCommand::Stop => "إيقاف كل الطرفيات العاملة في الخلفية",
            SlashCommand::MemoryDrop => "لا تستخدمه",
            SlashCommand::MemoryUpdate => "لا تستخدمه",
            SlashCommand::Model => "اختيار النموذج ومستوى التفكير",
            SlashCommand::Presets => "تشغيل أو حفظ طلبات وأوامر قابلة لإعادة الاستخدام",
            SlashCommand::Daybreak => "تشغيل أو إيقاف Daybreak",
            SlashCommand::Ide => {
                "تضمين التحديد الحالي والملفات المفتوحة وسياق آخر من محررك (IDE)"
            }
            SlashCommand::Plan => "التبديل إلى وضع التخطيط",
            SlashCommand::Voice => "تشغيل أو إيقاف الصوت؛ استخدم /voice settings لاختيار صوت",
            SlashCommand::Goal => "تحديد أو عرض هدف مهمة طويلة",
            SlashCommand::Agents => "فتح مركز التحكم بالوكلاء",
            SlashCommand::MultiAgents => "التنقل بين الوكلاء الفرعيين لهذه الجلسة",
            SlashCommand::Side | SlashCommand::Btw => "بدء محادثة جانبية",
            SlashCommand::Permissions => "تحديد ما يُسمح لـ Codex بفعله",
            SlashCommand::Keymap => "تغيير اختصارات لوحة المفاتيح",
            SlashCommand::Vim => "تبديل وضع Vim في حقل الكتابة",
            SlashCommand::ElevateSandbox => "إعداد بيئة العزل (sandbox) بصلاحيات مرتفعة",
            SlashCommand::Experimental => "تبديل الميزات التجريبية",
            SlashCommand::AutoReview => "الموافقة على إعادة محاولة واحدة لطلب رفضته المراجعة الآلية",
            SlashCommand::Memories => "ضبط استخدام الذاكرة وتوليدها",
            SlashCommand::Mcp => "عرض أدوات MCP؛ استخدم /mcp verbose أو /mcp login <name>",
            SlashCommand::Apps => "إدارة التطبيقات",
            SlashCommand::Plugins => "تصفح الإضافات",
            SlashCommand::Logout => "تسجيل الخروج من Codex",
            SlashCommand::Rollout => "عرض مسار ملف السجل (rollout)",
            SlashCommand::TestApproval => "اختبار طلب الموافقة",
        }
    }

    /// Command string without the leading '/'. Provided for compatibility with
    /// existing code that expects a method named `command()`.
    pub fn command(self) -> &'static str {
        self.into()
    }

    /// Whether this command supports inline args (for example `/review ...`).
    pub fn supports_inline_args(self) -> bool {
        matches!(
            self,
            SlashCommand::Review
                | SlashCommand::Model
                | SlashCommand::Permissions
                | SlashCommand::Title
                | SlashCommand::Statusline
                | SlashCommand::Rename
                | SlashCommand::New
                | SlashCommand::Clear
                | SlashCommand::Fork
                | SlashCommand::Plan
                | SlashCommand::Goal
                | SlashCommand::Voice
                | SlashCommand::Ide
                | SlashCommand::Keymap
                | SlashCommand::Mcp
                | SlashCommand::Export
                | SlashCommand::Raw
                | SlashCommand::Cd
                | SlashCommand::Pwd
                | SlashCommand::Usage
                | SlashCommand::Pets
                | SlashCommand::Side
                | SlashCommand::Btw
                | SlashCommand::Resume
        )
    }

    /// Whether this command remains available inside an active side conversation.
    pub fn available_in_side_conversation(self) -> bool {
        matches!(
            self,
            SlashCommand::Copy
                | SlashCommand::Agents
                | SlashCommand::Export
                | SlashCommand::Raw
                | SlashCommand::Diff
                | SlashCommand::Mention
                | SlashCommand::Status
                | SlashCommand::Daemon
                | SlashCommand::Warnings
                | SlashCommand::Pwd
                | SlashCommand::Usage
                | SlashCommand::Ide
                | SlashCommand::Rename
        )
    }

    /// Whether dispatch needs thread state to validate this command before consuming its draft.
    /// The composer must defer busy-state rejection and draft clearing for these commands.
    pub(crate) fn requires_dispatch_validation(self) -> bool {
        matches!(self, SlashCommand::Fork | SlashCommand::Review)
    }

    /// Commands that do not require a writable current thread. The server must still be connected.
    pub(crate) fn available_when_thread_unavailable(self) -> bool {
        matches!(
            self,
            SlashCommand::New
                | SlashCommand::Clear
                | SlashCommand::Resume
                | SlashCommand::Agents
                | SlashCommand::MultiAgents
                | SlashCommand::Quit
                | SlashCommand::Exit
                | SlashCommand::Status
                | SlashCommand::Warnings
                | SlashCommand::DebugConfig
                | SlashCommand::Pwd
                | SlashCommand::Rollout
                | SlashCommand::Copy
                | SlashCommand::Raw
        )
    }

    /// Whether this command can be run while a task is in progress.
    pub fn available_during_task(self) -> bool {
        match self {
            SlashCommand::New
            | SlashCommand::Delete
            | SlashCommand::Fork
            | SlashCommand::Worktree
            | SlashCommand::Init
            | SlashCommand::Compact
            | SlashCommand::Recap
            | SlashCommand::Export
            | SlashCommand::Keymap
            | SlashCommand::Tui
            | SlashCommand::Vim
            | SlashCommand::ElevateSandbox
            | SlashCommand::Experimental
            | SlashCommand::Memories
            | SlashCommand::Import
            | SlashCommand::Review
            | SlashCommand::Plan
            | SlashCommand::Cd
            | SlashCommand::Clear
            | SlashCommand::Logout
            | SlashCommand::MemoryDrop
            | SlashCommand::MemoryUpdate => false,
            SlashCommand::Diff
            | SlashCommand::Archive
            | SlashCommand::Resume
            | SlashCommand::Model
            | SlashCommand::Presets
            | SlashCommand::Daybreak
            | SlashCommand::Permissions
            | SlashCommand::Copy
            | SlashCommand::Raw
            | SlashCommand::Rename
            | SlashCommand::Mention
            | SlashCommand::Skills
            | SlashCommand::Hooks
            | SlashCommand::Status
            | SlashCommand::Daemon
            | SlashCommand::Warnings
            | SlashCommand::Pwd
            | SlashCommand::Usage
            | SlashCommand::DebugConfig
            | SlashCommand::Ps
            | SlashCommand::Stop
            | SlashCommand::App
            | SlashCommand::Goal
            | SlashCommand::Voice
            | SlashCommand::Mcp
            | SlashCommand::Apps
            | SlashCommand::Plugins
            | SlashCommand::Title
            | SlashCommand::Statusline
            | SlashCommand::AutoReview
            | SlashCommand::Feedback
            | SlashCommand::Ide
            | SlashCommand::Quit
            | SlashCommand::Exit
            | SlashCommand::Side
            | SlashCommand::Btw => true,
            SlashCommand::Rollout => true,
            SlashCommand::TestApproval => true,
            SlashCommand::Agents | SlashCommand::MultiAgents => true,
            SlashCommand::Config | SlashCommand::Theme | SlashCommand::Pets => false,
        }
    }

    fn is_visible(self) -> bool {
        match self {
            SlashCommand::Copy => !cfg!(target_os = "android"),
            SlashCommand::App => cfg!(any(target_os = "macos", target_os = "windows")),
            SlashCommand::Voice => true,
            SlashCommand::Rollout | SlashCommand::TestApproval => cfg!(debug_assertions),
            _ => true,
        }
    }
}

/// Return all built-in commands in a Vec paired with their command string.
pub fn built_in_slash_commands() -> Vec<(&'static str, SlashCommand)> {
    SlashCommand::iter()
        .filter(|command| command.is_visible())
        .map(|c| (c.command(), c))
        .collect()
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use std::str::FromStr;

    use super::SlashCommand;

    #[test]
    fn stop_command_is_canonical_name() {
        assert_eq!(SlashCommand::Stop.command(), "stop");
    }

    #[test]
    fn clean_alias_parses_to_stop_command() {
        assert_eq!(SlashCommand::from_str("clean"), Ok(SlashCommand::Stop));
    }

    #[test]
    fn pet_alias_parses_to_pets_command() {
        assert_eq!(SlashCommand::Pets.command(), "pets");
        assert_eq!(SlashCommand::from_str("pet"), Ok(SlashCommand::Pets));
    }

    #[test]
    fn certain_commands_are_available_during_task() {
        assert!(SlashCommand::Goal.available_during_task());
        assert!(SlashCommand::Ide.available_during_task());
        assert!(SlashCommand::Title.available_during_task());
        assert!(SlashCommand::Statusline.available_during_task());
        assert!(SlashCommand::Raw.available_during_task());
        assert!(SlashCommand::Raw.available_in_side_conversation());
        assert!(SlashCommand::Raw.supports_inline_args());
        assert!(SlashCommand::App.available_during_task());
    }

    #[test]
    fn auto_review_command_is_approve() {
        assert_eq!(SlashCommand::AutoReview.command(), "approve");
        assert_eq!(
            SlashCommand::from_str("approve"),
            Ok(SlashCommand::AutoReview)
        );
    }
}
