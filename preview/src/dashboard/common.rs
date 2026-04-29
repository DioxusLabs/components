use dioxus::prelude::*;
use dioxus_primitives::icon::Icon;

#[derive(Clone, Copy, PartialEq)]
pub struct AvatarProfile {
    pub name: &'static str,
    pub initials: &'static str,
    pub src: &'static str,
}

pub const AVATAR_PROFILE_OPTIONS: &[AvatarProfile] = &[
    AvatarProfile {
        name: "ealmloff",
        initials: "EA",
        src: "https://avatars.githubusercontent.com/u/66571940?s=96&v=4",
    },
    AvatarProfile {
        name: "jkelleyrtp",
        initials: "JK",
        src: "https://github.com/jkelleyrtp.png",
    },
    AvatarProfile {
        name: "DioxusLabs",
        initials: "DX",
        src: "https://github.com/DioxusLabs.png",
    },
];

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum FolderId {
    Inbox,
    Starred,
    Sent,
    Drafts,
    Archive,
    Trash,
}

impl FolderId {
    pub const fn as_str(self) -> &'static str {
        match self {
            FolderId::Inbox => "inbox",
            FolderId::Starred => "starred",
            FolderId::Sent => "sent",
            FolderId::Drafts => "drafts",
            FolderId::Archive => "archive",
            FolderId::Trash => "trash",
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum TabId {
    All,
    Unread,
    Flagged,
}

impl TabId {
    pub const fn as_str(self) -> &'static str {
        match self {
            TabId::All => "all",
            TabId::Unread => "unread",
            TabId::Flagged => "flagged",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "all" => Some(TabId::All),
            "unread" => Some(TabId::Unread),
            "flagged" => Some(TabId::Flagged),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Folder {
    pub id: FolderId,
    pub label: &'static str,
    pub icon: IconKind,
    pub count: Option<u32>,
}

#[derive(Clone, Copy, PartialEq)]
pub struct Tab {
    pub id: TabId,
    pub label: &'static str,
    pub count: u32,
}

#[derive(Clone, Copy, PartialEq)]
pub struct MessageProperties {
    pub message_id: &'static str,
    pub folder_id: FolderId,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum MessageTag {
    Alerts,
    Calendar,
    Deploys,
    Design,
    Github,
    Newsletter,
    Receipts,
    Sales,
    Security,
    Urgent,
    Work,
}

impl MessageTag {
    pub const ALL: &[MessageTag] = &[
        MessageTag::Alerts,
        MessageTag::Calendar,
        MessageTag::Deploys,
        MessageTag::Design,
        MessageTag::Github,
        MessageTag::Newsletter,
        MessageTag::Receipts,
        MessageTag::Sales,
        MessageTag::Security,
        MessageTag::Urgent,
        MessageTag::Work,
    ];

    pub const fn label(self) -> &'static str {
        match self {
            MessageTag::Alerts => "alerts",
            MessageTag::Calendar => "calendar",
            MessageTag::Deploys => "deploys",
            MessageTag::Design => "design",
            MessageTag::Github => "github",
            MessageTag::Newsletter => "newsletter",
            MessageTag::Receipts => "receipts",
            MessageTag::Sales => "sales",
            MessageTag::Security => "security",
            MessageTag::Urgent => "urgent",
            MessageTag::Work => "work",
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct Message {
    pub id: &'static str,
    pub day: &'static str,
    pub from: &'static str,
    pub from_addr: &'static str,
    pub initials: &'static str,
    pub subject: &'static str,
    pub snippet: &'static str,
    pub body: &'static str,
    pub time: &'static str,
    pub full_time: &'static str,
    pub unread: bool,
    pub starred: bool,
    pub thread_count: u32,
    pub tags: &'static [MessageTag],
    pub has_attachment: bool,
}

pub const EMAIL_REPEAT_COUNT: usize = 5;

#[derive(Clone, PartialEq)]
pub struct MessageState {
    pub uid: String,
    pub source_id: &'static str,
    pub folder_id: FolderId,
    pub tags: Vec<MessageTag>,
    pub unread: bool,
    pub starred: bool,
    pub flagged: bool,
    pub snoozed: bool,
}

pub fn seed_message_states() -> Vec<MessageState> {
    let mut out = Vec::with_capacity(MESSAGES.len() * EMAIL_REPEAT_COUNT);
    for rep in 0..EMAIL_REPEAT_COUNT {
        for msg in MESSAGES.iter() {
            let folder_id = MESSAGE_PROPERTIES
                .iter()
                .find(|p| p.message_id == msg.id)
                .map(|p| p.folder_id)
                .unwrap_or(DEFAULT_MESSAGE_FOLDER_ID);
            out.push(MessageState {
                uid: format!("{}#{}", msg.id, rep),
                source_id: msg.id,
                folder_id,
                tags: msg.tags.to_vec(),
                unread: msg.unread,
                starred: msg.starred,
                flagged: msg.starred,
                snoozed: false,
            });
        }
    }
    out
}

pub fn lookup_message(source_id: &str) -> &'static Message {
    MESSAGES
        .iter()
        .find(|m| m.id == source_id)
        .unwrap_or(&MESSAGES[0])
}

pub const FOLDERS: &[Folder] = &[
    Folder {
        id: FolderId::Inbox,
        label: "Inbox",
        icon: IconKind::Inbox,
        count: Some(12),
    },
    Folder {
        id: FolderId::Starred,
        label: "Starred",
        icon: IconKind::StarOutline,
        count: None,
    },
    Folder {
        id: FolderId::Sent,
        label: "Sent",
        icon: IconKind::Send,
        count: None,
    },
    Folder {
        id: FolderId::Drafts,
        label: "Drafts",
        icon: IconKind::Pen,
        count: Some(3),
    },
    Folder {
        id: FolderId::Archive,
        label: "Archive",
        icon: IconKind::Archive,
        count: None,
    },
    Folder {
        id: FolderId::Trash,
        label: "Trash",
        icon: IconKind::Trash,
        count: None,
    },
];

pub const TABS: &[Tab] = &[
    Tab {
        id: TabId::All,
        label: "All",
        count: 42,
    },
    Tab {
        id: TabId::Unread,
        label: "Unread",
        count: 12,
    },
    Tab {
        id: TabId::Flagged,
        label: "Flagged",
        count: 5,
    },
];

pub const DEFAULT_MESSAGE_FOLDER_ID: FolderId = FolderId::Inbox;

pub const MESSAGE_PROPERTIES: &[MessageProperties] = &[
    MessageProperties {
        message_id: "m6",
        folder_id: FolderId::Sent,
    },
    MessageProperties {
        message_id: "m23",
        folder_id: FolderId::Sent,
    },
    MessageProperties {
        message_id: "m29",
        folder_id: FolderId::Sent,
    },
    MessageProperties {
        message_id: "m43",
        folder_id: FolderId::Sent,
    },
    MessageProperties {
        message_id: "m51",
        folder_id: FolderId::Sent,
    },
    MessageProperties {
        message_id: "m55",
        folder_id: FolderId::Sent,
    },
    MessageProperties {
        message_id: "m58",
        folder_id: FolderId::Sent,
    },
    MessageProperties {
        message_id: "m3",
        folder_id: FolderId::Drafts,
    },
    MessageProperties {
        message_id: "m8",
        folder_id: FolderId::Drafts,
    },
    MessageProperties {
        message_id: "m37",
        folder_id: FolderId::Drafts,
    },
    MessageProperties {
        message_id: "m20",
        folder_id: FolderId::Archive,
    },
    MessageProperties {
        message_id: "m21",
        folder_id: FolderId::Archive,
    },
    MessageProperties {
        message_id: "m31",
        folder_id: FolderId::Archive,
    },
    MessageProperties {
        message_id: "m39",
        folder_id: FolderId::Archive,
    },
    MessageProperties {
        message_id: "m46",
        folder_id: FolderId::Archive,
    },
    MessageProperties {
        message_id: "m52",
        folder_id: FolderId::Archive,
    },
    MessageProperties {
        message_id: "m4",
        folder_id: FolderId::Trash,
    },
    MessageProperties {
        message_id: "m22",
        folder_id: FolderId::Trash,
    },
    MessageProperties {
        message_id: "m44",
        folder_id: FolderId::Trash,
    },
];

pub const MESSAGES: &[Message] = &[
    Message {
        id: "m1", day: "Today",
        from: "Linear", from_addr: "notifications@linear.app", initials: "LN",
        subject: "ENG-1247 · Reading-pane scroll restoration",
        snippet: "Caroline Wu assigned this issue to you and moved it into Cycle 47. Three sub-issues remain open. Due Friday.",
        body: "Caroline Wu assigned ENG-1247 to you and moved it into Cycle 47.\n\nStatus: In Progress\nPriority: High\nCycle: 47 · ends Fri Apr 30\nSub-issues: 3 open, 2 done\n\nDescription\nThe reading pane should restore scroll position when navigating back to a previously-opened thread. Currently it resets to the top, which is disorienting when you've scrolled deep into a long thread.\n\nAcceptance criteria\n— Position persists across thread switches in the same session\n— Position resets when message is marked unread\n— Works with virtualized lists\n\nOpen in Linear → linear.app/eng/issue/ENG-1247",
        time: "11:42", full_time: "Apr 28, 2026 · 11:42 AM",
        unread: true, starred: false, thread_count: 4,
        tags: &[MessageTag::Work, MessageTag::Urgent, MessageTag::Calendar], has_attachment: false,
    },
    Message {
        id: "m2", day: "Today",
        from: "GitHub", from_addr: "noreply@github.com", initials: "GH",
        subject: "[anthropics/claude-cookbook] PR #482 ready for review",
        snippet: "samir-kapoor opened pull request #482: Add streaming examples for tool use. 14 files changed, +312 −47.",
        body: "samir-kapoor opened pull request #482\n\nAdd streaming examples for tool use\n\n14 files changed · +312 −47\n\nReview required from one of: @you, @marta-l, @dan-k",
        time: "10:18", full_time: "Apr 28, 2026 · 10:18 AM",
        unread: true, starred: true, thread_count: 1,
        tags: &[MessageTag::Github, MessageTag::Work, MessageTag::Urgent], has_attachment: false,
    },
    Message {
        id: "m3", day: "Today",
        from: "Maya Chen", from_addr: "maya@figma-internal.com", initials: "MC",
        subject: "Re: Two-pane mail client — first pass",
        snippet: "These are looking sharp. The hairline rules instead of cards is exactly the right call for this density. One small note about the unread indicator —",
        body: "These are looking sharp. The hairline rules instead of cards is exactly the right call for this density.\n\nOne small note about the unread indicator — the 2px left bar reads beautifully but I wonder if we lose it visually on the selected row since both use the same accent. Maybe try inverting it: filled bar for unread, hollow ring for selected?\n\nAlso the day-headers as monospace eyebrows are a really nice touch. Could we extend that treatment to the reading-pane metadata? Right now the timestamp on the right feels disconnected from the rest of the page rhythm.\n\nI'll be in tomorrow morning if you want to walk through it.\n\n— M",
        time: "09:51", full_time: "Apr 28, 2026 · 9:51 AM",
        unread: true, starred: false, thread_count: 6,
        tags: &[MessageTag::Design, MessageTag::Work], has_attachment: true,
    },
    Message {
        id: "m4", day: "Today",
        from: "Stripe", from_addr: "receipts@stripe.com", initials: "ST",
        subject: "Your April invoice is ready — $2,847.00",
        snippet: "Invoice INV-2026-04-218 for Acme Corp is ready. Payment of $2,847.00 will be charged to Visa •• 4242 on May 1.",
        body: "Invoice INV-2026-04-218\nAcme Corp · April 2026\n\nPayment of $2,847.00 will be charged to Visa •• 4242 on May 1, 2026.\n\nLine items\n— Subscription, Pro tier · $1,500.00\n— Usage overage · $1,247.00\n— Tax · $100.00\n\nView invoice → stripe.com/invoices/INV-2026-04-218",
        time: "08:30", full_time: "Apr 28, 2026 · 8:30 AM",
        unread: false, starred: false, thread_count: 1,
        tags: &[MessageTag::Receipts, MessageTag::Work], has_attachment: true,
    },
    Message {
        id: "m5", day: "Today",
        from: "Calendar", from_addr: "calendar@workspace.app", initials: "CA",
        subject: "Daily agenda · Tue Apr 28",
        snippet: "4 events today. Next: Design review with Maya at 14:00. Travel time considered. No conflicts.",
        body: "Tue Apr 28, 2026\n\n09:00–09:30  Standup (recurring)\n14:00–15:00  Design review · Maya Chen\n15:30–16:00  1:1\n17:00–17:45  Architecture sync\n\nNo conflicts detected.",
        time: "07:00", full_time: "Apr 28, 2026 · 7:00 AM",
        unread: false, starred: false, thread_count: 1,
        tags: &[MessageTag::Calendar, MessageTag::Work], has_attachment: false,
    },
    Message {
        id: "m6", day: "Yesterday",
        from: "You", from_addr: "you@yourcompany.com", initials: "Y",
        subject: "Q2 roadmap — final draft for your eyes",
        snippet: "Attaching the Q2 roadmap doc with all the changes from Friday's offsite. Biggest swap: pushing Mobile to Q3 so we can land Search-v2 properly.",
        body: "Attaching the Q2 roadmap doc with all the changes from Friday's offsite.\n\nBiggest swap: pushing Mobile to Q3 so we can land Search-v2 properly. Reasoning is in the doc on page 4. Marta and I are aligned but I want your read before sending it to leadership Wednesday.\n\nReview by EOD tomorrow if possible?",
        time: "Yesterday", full_time: "Apr 27, 2026 · 5:14 PM",
        unread: false, starred: true, thread_count: 3,
        tags: &[MessageTag::Work, MessageTag::Urgent, MessageTag::Calendar], has_attachment: true,
    },
    Message {
        id: "m7", day: "Yesterday",
        from: "Vercel", from_addr: "deploys@vercel.com", initials: "VC",
        subject: "Production deploy succeeded · web@4f8a2c1",
        snippet: "Deployment to production completed in 2m 14s. 0 errors, 12 warnings. View build logs.",
        body: "Deployment to production completed in 2m 14s.\n\n0 errors · 12 warnings\n\nView build logs → vercel.com/deploys/4f8a2c1",
        time: "Yesterday", full_time: "Apr 27, 2026 · 3:42 PM",
        unread: false, starred: false, thread_count: 1,
        tags: &[MessageTag::Deploys, MessageTag::Work], has_attachment: false,
    },
    Message {
        id: "m8", day: "Yesterday",
        from: "Marta Liu", from_addr: "marta.l@yourcompany.com", initials: "ML",
        subject: "Notes from the customer call (Acme)",
        snippet: "Quick recap from this morning's call with Sarah at Acme. They're expanding the seat count to 240 in May, and asked about SSO timing.",
        body: "Quick recap from this morning's call with Sarah at Acme.\n\nThey're expanding the seat count to 240 in May, and asked about SSO timing. I committed to a Q3 ETA but said we'd confirm specifics by mid-May.\n\nAction items\n— Confirm SSO timeline\n— Send updated pricing for 240 seats\n— Schedule technical kickoff for first week of May",
        time: "Yesterday", full_time: "Apr 27, 2026 · 11:08 AM",
        unread: false, starred: false, thread_count: 2,
        tags: &[MessageTag::Sales, MessageTag::Work], has_attachment: false,
    },
    Message {
        id: "m9", day: "This week",
        from: "1Password", from_addr: "security@1password.com", initials: "1P",
        subject: "New sign-in from MacBook Pro · San Francisco",
        snippet: "We detected a new sign-in to your account. If this was you, no action needed. If not, change your master password immediately.",
        body: "We detected a new sign-in to your account.\n\nDevice: MacBook Pro\nLocation: San Francisco, CA\nTime: Apr 26, 2026 · 8:17 AM\n\nIf this was you, no action needed. If not, change your master password immediately.",
        time: "Mon", full_time: "Apr 26, 2026 · 8:17 AM",
        unread: false, starred: false, thread_count: 1,
        tags: &[MessageTag::Security, MessageTag::Alerts], has_attachment: false,
    },
    Message {
        id: "m10", day: "This week",
        from: "AWS Billing", from_addr: "no-reply@aws.amazon.com", initials: "AW",
        subject: "Your AWS bill for April is now available",
        snippet: "Total estimated charges for April 2026: $1,204.36. View detailed breakdown in the AWS Billing console.",
        body: "Total estimated charges for April 2026: $1,204.36\n\nView detailed breakdown in the AWS Billing console.",
        time: "Sun", full_time: "Apr 25, 2026 · 11:00 PM",
        unread: false, starred: false, thread_count: 1,
        tags: &[MessageTag::Receipts, MessageTag::Work], has_attachment: false,
    },
    Message {
        id: "m11", day: "This week",
        from: "Notion", from_addr: "team@notion.so", initials: "NO",
        subject: "Caroline shared 'Cycle 47 — Eng planning' with you",
        snippet: "Caroline Wu (caroline@yourcompany.com) gave you can-edit access to a page in the Engineering workspace.",
        body: "Caroline Wu (caroline@yourcompany.com) gave you can-edit access to a page in the Engineering workspace.\n\nOpen page → notion.so/cycle-47-eng-planning",
        time: "Sat", full_time: "Apr 24, 2026 · 4:33 PM",
        unread: false, starred: false, thread_count: 1,
        tags: &[MessageTag::Work, MessageTag::Calendar], has_attachment: false,
    },
    Message {
        id: "m12", day: "This week",
        from: "Sentry", from_addr: "alerts@sentry.io", initials: "SE",
        subject: "[web-prod] Spike resolved · TypeError in MailList",
        snippet: "An error spike (47 events / 5 min) opened at 14:02 and auto-resolved at 14:11. No users affected outside the rollout cohort.",
        body: "An error spike (47 events / 5 min) opened at 14:02 and auto-resolved at 14:11.\n\nNo users affected outside the rollout cohort.\n\nView issue → sentry.io/issues/web-prod/TypeError-MailList",
        time: "Fri", full_time: "Apr 23, 2026 · 2:11 PM",
        unread: false, starred: false, thread_count: 1,
        tags: &[MessageTag::Alerts, MessageTag::Work, MessageTag::Deploys], has_attachment: false,
    },
    Message {
        id: "m13", day: "This week",
        from: "Caroline Wu", from_addr: "caroline@yourcompany.com", initials: "CW",
        subject: "Re: Cycle 47 capacity",
        snippet: "I bumped your allocation to 5 issues for the cycle since the search migration is wrapping up. Let me know if that's too aggressive.",
        body: "I bumped your allocation to 5 issues for the cycle since the search migration is wrapping up.\n\nLet me know if that's too aggressive — I can pull one back if you're already underwater.\n\n— C",
        time: "Fri", full_time: "Apr 23, 2026 · 11:02 AM",
        unread: false, starred: false, thread_count: 5,
        tags: &[MessageTag::Work, MessageTag::Calendar], has_attachment: false,
    },
    Message {
        id: "m14", day: "This week",
        from: "Datadog", from_addr: "alerts@datadog.com", initials: "DD",
        subject: "[Triggered] high p95 latency · api.search",
        snippet: "p95 latency on api.search exceeded 850ms for 10 minutes. Threshold: 500ms. View dashboard for breakdown by region.",
        body: "Monitor: high p95 latency · api.search\nState: Triggered\np95: 850ms (threshold 500ms)\nDuration: 10 min\n\nTop region by latency: us-east-1 (1.2s)\n\nDashboard → app.datadoghq.com/dashboard/api-search",
        time: "Thu", full_time: "Apr 22, 2026 · 6:48 PM",
        unread: false, starred: false, thread_count: 1,
        tags: &[MessageTag::Alerts, MessageTag::Work], has_attachment: false,
    },
    Message {
        id: "m15", day: "This week",
        from: "Slack", from_addr: "feedback@slack.com", initials: "SL",
        subject: "5 new mentions in #eng-search",
        snippet: "@dan-k was mentioned 5 times today. Top thread: 'Search v2 cutover plan' from samir-kapoor.",
        body: "5 new mentions in #eng-search:\n\n— samir-kapoor: 'Search v2 cutover plan' (2 replies)\n— marta-l: 'Acme rollout next week?' (8 replies)\n— caroline-w: 'Final QA pass on highlight bug'\n— pedro-c: 'Latency dashboard'\n— eli-r: 'Cache key collision repro'\n\nOpen Slack → app.slack.com/client",
        time: "Thu", full_time: "Apr 22, 2026 · 5:00 PM",
        unread: false, starred: false, thread_count: 1,
        tags: &[MessageTag::Work, MessageTag::Github], has_attachment: false,
    },
    Message {
        id: "m16", day: "This week",
        from: "Figma", from_addr: "no-reply@figma.com", initials: "FG",
        subject: "Maya Chen invited you to 'Mail v2 — exploration'",
        snippet: "You can edit. Two new frames: 'List density study' and 'Reading-pane variants'. Comments enabled for the team.",
        body: "Maya Chen (maya@figma-internal.com) invited you to:\n\nMail v2 — exploration\nRole: can edit\n\nNew this week:\n— List density study (3 variants)\n— Reading-pane variants (5 variants)\n\nOpen → figma.com/file/mail-v2",
        time: "Wed", full_time: "Apr 21, 2026 · 3:18 PM",
        unread: false, starred: false, thread_count: 1,
        tags: &[MessageTag::Design, MessageTag::Work], has_attachment: false,
    },
    Message {
        id: "m17", day: "This week",
        from: "GitHub", from_addr: "noreply@github.com", initials: "GH",
        subject: "[anthropics/claude-cookbook] CI failed on main",
        snippet: "Workflow 'integration-tests' failed on commit 4f8a2c1. 2 of 47 tests failing in test_streaming.py.",
        body: "Workflow: integration-tests\nBranch: main\nCommit: 4f8a2c1 (samir-kapoor)\n\n2 of 47 tests failing:\n— test_streaming_with_tool_use (timeout)\n— test_streaming_partial_json (assertion)\n\nView run → github.com/anthropics/claude-cookbook/actions/runs/8821",
        time: "Wed", full_time: "Apr 21, 2026 · 1:42 PM",
        unread: false, starred: false, thread_count: 2,
        tags: &[MessageTag::Github, MessageTag::Alerts, MessageTag::Work, MessageTag::Deploys], has_attachment: false,
    },
    Message {
        id: "m18", day: "This week",
        from: "Pedro Carvalho", from_addr: "pedro@yourcompany.com", initials: "PC",
        subject: "Loom: walking through the indexer refactor",
        snippet: "Recorded a 12 min walkthrough of the new indexer pipeline before I head to Lisbon. Watch when you have a coffee.",
        body: "Recorded a 12 min Loom walking through the new indexer pipeline before I head to Lisbon Friday.\n\nKey timestamps:\n— 0:00 architecture diagram\n— 3:20 partition strategy\n— 7:15 backpressure handling\n— 10:40 known limitations\n\nWatch → loom.com/share/indexer-refactor",
        time: "Tue", full_time: "Apr 20, 2026 · 11:25 AM",
        unread: false, starred: false, thread_count: 1,
        tags: &[MessageTag::Work, MessageTag::Github], has_attachment: false,
    },
    Message {
        id: "m19", day: "This week",
        from: "1Password", from_addr: "security@1password.com", initials: "1P",
        subject: "Weekly security report",
        snippet: "0 reused passwords, 2 weak passwords, 1 site without 2FA. Watchtower score: 94/100. Improved from 91 last week.",
        body: "Weekly security report\n\nWatchtower score: 94/100 (+3)\n\nReused passwords: 0\nWeak passwords: 2 (down from 3)\nSites without 2FA: 1\nVulnerable sites: 0\n\nReview → 1password.com/watchtower",
        time: "Mon", full_time: "Apr 20, 2026 · 8:00 AM",
        unread: false, starred: false, thread_count: 1,
        tags: &[MessageTag::Security, MessageTag::Newsletter], has_attachment: false,
    },
    Message {
        id: "m20", day: "Last week",
        from: "LinkedIn", from_addr: "messages-noreply@linkedin.com", initials: "LI",
        subject: "Sarah Patel sent you a connection request",
        snippet: "We met briefly at the Anthropic dev meetup. Loved your talk on DX for AI tooling. Would love to stay in touch.",
        body: "Sarah Patel · Staff Engineer at Vercel\n\nWe met briefly at the Anthropic dev meetup. Loved your talk on DX for AI tooling. Would love to stay in touch.\n\nAccept · Ignore",
        time: "Sun", full_time: "Apr 19, 2026 · 6:14 PM",
        unread: false, starred: false, thread_count: 1,
        tags: &[MessageTag::Newsletter], has_attachment: false,
    },
    Message {
        id: "m21", day: "Last week",
        from: "Substack", from_addr: "no-reply@substack.com", initials: "SU",
        subject: "Stratechery · The end of the unbundled inbox",
        snippet: "Ben Thompson on why the consumer email stack is collapsing back into a few super-apps, and what it means for productivity tools.",
        body: "Stratechery by Ben Thompson\n\nThe end of the unbundled inbox\n\nBen Thompson on why the consumer email stack is collapsing back into a few super-apps, and what it means for productivity tools.\n\nRead online → stratechery.com",
        time: "Sun", full_time: "Apr 19, 2026 · 9:00 AM",
        unread: false, starred: false, thread_count: 1,
        tags: &[MessageTag::Newsletter, MessageTag::Sales], has_attachment: false,
    },
    Message {
        id: "m22", day: "Last week",
        from: "Stripe", from_addr: "receipts@stripe.com", initials: "ST",
        subject: "Dispute opened on payment INV-2026-04-184",
        snippet: "A customer at Mossberg Industries opened a dispute for $1,200.00. You have until May 14 to respond with evidence.",
        body: "Dispute opened\nInvoice: INV-2026-04-184\nAmount: $1,200.00\nReason: 'Product not as described'\n\nResponse deadline: May 14, 2026\n\nProvide evidence → stripe.com/disputes/dp_4Hk2",
        time: "Sat", full_time: "Apr 18, 2026 · 4:22 PM",
        unread: false, starred: false, thread_count: 1,
        tags: &[MessageTag::Urgent, MessageTag::Receipts, MessageTag::Sales, MessageTag::Alerts], has_attachment: false,
    },
    Message {
        id: "m23", day: "Last week",
        from: "Eli Rosen", from_addr: "eli.r@yourcompany.com", initials: "ER",
        subject: "Postmortem draft · Apr 11 search outage",
        snippet: "First draft of the postmortem doc. Want your eyes before I share with the broader eng org Tuesday.",
        body: "First draft of the postmortem doc for the Apr 11 search outage.\n\nWant your eyes before I share with the broader eng org Tuesday.\n\nGoogle Doc → docs.google.com/postmortem-apr-11\n\n— Eli",
        time: "Sat", full_time: "Apr 18, 2026 · 10:08 AM",
        unread: false, starred: true, thread_count: 3,
        tags: &[MessageTag::Work, MessageTag::Urgent, MessageTag::Alerts], has_attachment: true,
    },
    Message {
        id: "m24", day: "Last week",
        from: "AWS Cost Explorer", from_addr: "no-reply@aws.amazon.com", initials: "AW",
        subject: "Anomaly detected: EC2 in us-west-2 spend up 217%",
        snippet: "Daily spend on EC2 in us-west-2 jumped from $42 to $134 on Apr 17. Likely cause: new c7g.4xlarge instances launched at 03:14 UTC.",
        body: "Anomaly detected\n\nService: EC2\nRegion: us-west-2\nDaily spend: $42 → $134 (+217%)\nDate: Apr 17, 2026\n\nLikely cause: 4 × c7g.4xlarge instances launched at 03:14 UTC, still running.\n\nInvestigate → console.aws.amazon.com/cost-anomaly",
        time: "Fri", full_time: "Apr 17, 2026 · 7:45 AM",
        unread: false, starred: false, thread_count: 1,
        tags: &[MessageTag::Alerts, MessageTag::Receipts, MessageTag::Security, MessageTag::Work], has_attachment: false,
    },
    Message {
        id: "m25", day: "Last week",
        from: "Notion", from_addr: "team@notion.so", initials: "NO",
        subject: "Marta Liu commented on '240-seat Acme expansion'",
        snippet: "@dan-k can you confirm the SSO timeline by Tuesday so I can update the proposal? Sarah needs final pricing by Wed.",
        body: "Marta Liu commented:\n\n@dan-k can you confirm the SSO timeline by Tuesday so I can update the proposal? Sarah needs final pricing by Wed.\n\nOpen page → notion.so/240-seat-acme-expansion",
        time: "Thu", full_time: "Apr 16, 2026 · 5:50 PM",
        unread: false, starred: false, thread_count: 4,
        tags: &[MessageTag::Sales, MessageTag::Work, MessageTag::Urgent], has_attachment: false,
    },
    Message {
        id: "m26", day: "Last week",
        from: "Linear", from_addr: "notifications@linear.app", initials: "LN",
        subject: "ENG-1198 · Highlight overlap on selected row",
        snippet: "Maya Chen reported a regression: selected row's accent bar overlaps the unread dot in dense mode. Repro attached.",
        body: "ENG-1198 reported by Maya Chen\n\nThe selected row's accent bar overlaps the unread dot in dense mode (≤32px row height). Looks broken in Safari only.\n\nSteps to reproduce attached.\n\nOpen → linear.app/eng/issue/ENG-1198",
        time: "Thu", full_time: "Apr 16, 2026 · 2:14 PM",
        unread: false, starred: false, thread_count: 2,
        tags: &[MessageTag::Work, MessageTag::Design], has_attachment: true,
    },
    Message {
        id: "m27", day: "Last week",
        from: "Vercel", from_addr: "deploys@vercel.com", initials: "VC",
        subject: "Production deploy failed · web@a14b9c2",
        snippet: "Build failed at step 'next build'. TypeScript error in src/mail/MailList.tsx: Property 'aria-selected' does not exist on type 'IntrinsicAttributes'.",
        body: "Build failed at step 'next build'.\n\nError:\nsrc/mail/MailList.tsx:142:14\nProperty 'aria-selected' does not exist on type 'IntrinsicAttributes & ItemProps'.\n\nView build logs → vercel.com/deploys/a14b9c2",
        time: "Wed", full_time: "Apr 15, 2026 · 11:32 AM",
        unread: false, starred: false, thread_count: 1,
        tags: &[MessageTag::Deploys, MessageTag::Alerts, MessageTag::Github, MessageTag::Work], has_attachment: false,
    },
    Message {
        id: "m28", day: "Last week",
        from: "Calendar", from_addr: "calendar@workspace.app", initials: "CA",
        subject: "Declined: 'Quick sync re: budget'",
        snippet: "Marta Liu declined your invitation. Reason: 'travel — let's reschedule for next week when I'm back from NYC'.",
        body: "Marta Liu declined: 'Quick sync re: budget'\nWhen: Apr 15, 2026 · 14:00–14:30\n\nReason: 'travel — let's reschedule for next week when I'm back from NYC'\n\nReschedule → calendar.app/reschedule",
        time: "Tue", full_time: "Apr 14, 2026 · 9:14 AM",
        unread: false, starred: false, thread_count: 1,
        tags: &[MessageTag::Calendar, MessageTag::Work], has_attachment: false,
    },
    Message {
        id: "m29", day: "Last week",
        from: "Samir Kapoor", from_addr: "samir@yourcompany.com", initials: "SK",
        subject: "Quick q on the streaming examples PR",
        snippet: "On the partial-JSON example — should we surface a typed callback or keep the parsed object as `any`? The DX implications are non-trivial.",
        body: "Quick q on the streaming examples PR (#482):\n\nOn the partial-JSON example — should we surface a typed callback or keep the parsed object as `any`? The DX implications are non-trivial:\n\nTyped: stronger guarantees, but users have to declare the schema up front.\nUntyped: looser, but less safe and IDE help is limited.\n\nMy lean is typed-with-fallback. Curious if you've thought through this.",
        time: "Tue", full_time: "Apr 14, 2026 · 8:01 AM",
        unread: false, starred: false, thread_count: 6,
        tags: &[MessageTag::Work, MessageTag::Github, MessageTag::Design], has_attachment: false,
    },
    Message {
        id: "m30", day: "Last week",
        from: "GitHub", from_addr: "noreply@github.com", initials: "GH",
        subject: "[anthropics/claude-cookbook] dependabot · 4 PRs ready",
        snippet: "Dependabot opened 4 update PRs: react@19.2 → 19.3, vite@5 → 6, eslint@8 → 9, typescript@5.4 → 5.5. All checks passing.",
        body: "Dependabot updates ready\n\n— PR #495: react 19.2 → 19.3 (patch)\n— PR #496: vite 5 → 6 (major)\n— PR #497: eslint 8 → 9 (major)\n— PR #498: typescript 5.4 → 5.5 (minor)\n\nAll checks passing.",
        time: "Mon", full_time: "Apr 13, 2026 · 7:15 AM",
        unread: false, starred: false, thread_count: 1,
        tags: &[MessageTag::Github, MessageTag::Security, MessageTag::Work], has_attachment: false,
    },
    Message {
        id: "m31", day: "Earlier in April",
        from: "Apple", from_addr: "no-reply@email.apple.com", initials: "AP",
        subject: "Your receipt from Apple · Apr 11",
        snippet: "Order ID: ML8RT2K. AppleCare+ for MacBook Pro 16-inch · $399.00. Total: $432.92.",
        body: "Your receipt from Apple\nOrder ID: ML8RT2K\n\nAppleCare+ for MacBook Pro 16-inch\n$399.00\n\nSubtotal: $399.00\nTax: $33.92\nTotal: $432.92\n\nView receipt → apple.com/orders/ML8RT2K",
        time: "Apr 11", full_time: "Apr 11, 2026 · 4:48 PM",
        unread: false, starred: false, thread_count: 1,
        tags: &[MessageTag::Receipts, MessageTag::Security], has_attachment: false,
    },
    Message {
        id: "m32", day: "Earlier in April",
        from: "Sentry", from_addr: "alerts@sentry.io", initials: "SE",
        subject: "[web-prod] New issue · UnhandledPromiseRejection",
        snippet: "First seen: Apr 10 14:22. 14 events in 1 hour. Affecting 8 users. Stack trace shows a missing await in MailList.fetchPage.",
        body: "New issue · UnhandledPromiseRejection\n\nFirst seen: Apr 10 14:22\nEvents: 14 (last hour)\nUsers affected: 8\n\nTop frame: MailList.fetchPage (src/mail/MailList.tsx:218)\nLikely cause: missing await on fetchPage()\n\nView issue → sentry.io/issues/web-prod/UnhandledPromiseRejection",
        time: "Apr 10", full_time: "Apr 10, 2026 · 2:24 PM",
        unread: false, starred: false, thread_count: 1,
        tags: &[MessageTag::Alerts, MessageTag::Work, MessageTag::Github], has_attachment: false,
    },
    Message {
        id: "m33", day: "Earlier in April",
        from: "Caroline Wu", from_addr: "caroline@yourcompany.com", initials: "CW",
        subject: "Engineering all-hands · Apr 30",
        snippet: "Reminder: eng all-hands on Apr 30 at 13:00 PT. Agenda: Q1 retrospective, Q2 roadmap, demo of Search v2, open Q&A.",
        body: "Eng all-hands · Apr 30 · 13:00–14:30 PT\n\nAgenda:\n— Q1 retrospective (10 min)\n— Q2 roadmap walkthrough (25 min)\n— Search v2 demo (15 min · Pedro)\n— Demo: new mail client (10 min · Maya)\n— Open Q&A (30 min)\n\nZoom link in calendar invite.",
        time: "Apr 9", full_time: "Apr 9, 2026 · 11:00 AM",
        unread: false, starred: false, thread_count: 1,
        tags: &[MessageTag::Work, MessageTag::Calendar], has_attachment: false,
    },
    Message {
        id: "m34", day: "Earlier in April",
        from: "Hex", from_addr: "no-reply@hex.tech", initials: "HX",
        subject: "Your scheduled report 'Daily DAU' is ready",
        snippet: "April 8 DAU: 38,420 (+2.4% WoW). Session length p50: 4m 12s. Top feature by usage: search.",
        body: "Daily DAU report — Apr 8\n\nDAU: 38,420 (+2.4% WoW)\nWAU: 142,108 (+1.8%)\nMAU: 312,540 (+0.9%)\n\nSession length p50: 4m 12s\nTop feature by usage: search (62% of sessions)\n\nOpen report → hex.tech/dashboards/daily-dau",
        time: "Apr 8", full_time: "Apr 8, 2026 · 8:00 AM",
        unread: false, starred: false, thread_count: 1,
        tags: &[MessageTag::Work, MessageTag::Newsletter], has_attachment: false,
    },
    Message {
        id: "m35", day: "Earlier in April",
        from: "Marta Liu", from_addr: "marta.l@yourcompany.com", initials: "ML",
        subject: "Mossberg lead · they want a demo Wed",
        snippet: "Mossberg Industries reached out cold via the website. 1,200 seats, Fortune 500. Wants a 30-min demo Wed at 15:00 ET. Can you join?",
        body: "Mossberg Industries reached out cold via the website.\n\nProfile:\n— 1,200 seats potential\n— Fortune 500\n— Currently on Outlook\n— Pain: poor search, no shared inboxes\n\nRequested: 30-min demo Wed at 15:00 ET\n\nCan you join? I'll lead but want eng presence for the SSO/SCIM questions.",
        time: "Apr 7", full_time: "Apr 7, 2026 · 4:32 PM",
        unread: false, starred: true, thread_count: 4,
        tags: &[MessageTag::Sales, MessageTag::Urgent, MessageTag::Work, MessageTag::Calendar], has_attachment: false,
    },
    Message {
        id: "m36", day: "Earlier in April",
        from: "Crunchbase Daily", from_addr: "newsletter@crunchbase.com", initials: "CB",
        subject: "AI infra raises $4.2B in Q1 · 47 deals tracked",
        snippet: "Q1 AI infrastructure funding hit $4.2B across 47 disclosed deals — up 38% from Q4. The big movers: model serving, agent platforms, retrieval.",
        body: "Crunchbase Daily · AI infra Q1 wrap\n\nQ1 AI infrastructure funding: $4.2B / 47 disclosed deals (+38% QoQ)\n\nTop categories:\n— Model serving (12 deals · $980M)\n— Agent platforms (9 deals · $1.1B)\n— Retrieval / RAG (8 deals · $620M)\n— Eval & observability (7 deals · $290M)\n\nFull report → crunchbase.com/insights/ai-infra-q1",
        time: "Apr 7", full_time: "Apr 7, 2026 · 7:00 AM",
        unread: false, starred: false, thread_count: 1,
        tags: &[MessageTag::Newsletter, MessageTag::Sales], has_attachment: false,
    },
    Message {
        id: "m37", day: "Earlier in April",
        from: "GitHub", from_addr: "noreply@github.com", initials: "GH",
        subject: "[anthropics/claude-cookbook] You were assigned PR #471",
        snippet: "marta-l requested your review on PR #471: Add SSO/SAML examples for enterprise customers. 8 files changed.",
        body: "marta-l requested your review on:\n\nPR #471 · Add SSO/SAML examples for enterprise customers\n\n8 files changed · +218 −12\n\nView PR → github.com/anthropics/claude-cookbook/pull/471",
        time: "Apr 6", full_time: "Apr 6, 2026 · 3:21 PM",
        unread: false, starred: false, thread_count: 2,
        tags: &[MessageTag::Github, MessageTag::Work, MessageTag::Sales, MessageTag::Security], has_attachment: false,
    },
    Message {
        id: "m38", day: "Earlier in April",
        from: "Eli Rosen", from_addr: "eli.r@yourcompany.com", initials: "ER",
        subject: "Re: index rebuild ETA",
        snippet: "Index rebuild for the search-v2 cutover finished at 02:14. Total time: 6h 22m. Validation queries all green. Ready for the cutover Friday.",
        body: "Index rebuild for the search-v2 cutover finished at 02:14.\n\nTotal time: 6h 22m (estimate was 8h, came in under)\nValidation queries: 100% match against legacy\nDelta indexer: caught up\n\nReady for the cutover Friday.\n\n— Eli",
        time: "Apr 5", full_time: "Apr 5, 2026 · 8:42 AM",
        unread: false, starred: false, thread_count: 7,
        tags: &[MessageTag::Work, MessageTag::Deploys, MessageTag::Calendar], has_attachment: false,
    },
    Message {
        id: "m39", day: "Earlier in April",
        from: "Cloudflare", from_addr: "noreply@cloudflare.com", initials: "CF",
        subject: "Certificate renewed · *.yourcompany.com",
        snippet: "Wildcard cert for *.yourcompany.com renewed automatically. Valid through Jul 4, 2026. No action needed.",
        body: "Certificate renewed\n\nDomain: *.yourcompany.com\nIssued: Apr 4, 2026\nValid through: Jul 4, 2026\nIssuer: Let's Encrypt\n\nNo action needed.",
        time: "Apr 4", full_time: "Apr 4, 2026 · 6:00 AM",
        unread: false, starred: false, thread_count: 1,
        tags: &[MessageTag::Security, MessageTag::Deploys, MessageTag::Work], has_attachment: false,
    },
    Message {
        id: "m40", day: "Earlier in April",
        from: "Stripe", from_addr: "receipts@stripe.com", initials: "ST",
        subject: "March MRR report · $284,210 (+4.2% MoM)",
        snippet: "March MRR closed at $284,210, up 4.2% from February. New: $14,200. Expansion: $8,400. Churn: −$2,210.",
        body: "March MRR report\n\nClosing MRR: $284,210 (+4.2% MoM)\nNew: $14,200\nExpansion: $8,400\nContraction: −$1,290\nChurn: −$2,210\n\nTop expansions:\n— Acme Corp ($1,200 → $2,400)\n— Mossberg pilot ($0 → $1,800)\n\nView report → stripe.com/reports/mrr-march",
        time: "Apr 3", full_time: "Apr 3, 2026 · 9:00 AM",
        unread: false, starred: false, thread_count: 1,
        tags: &[MessageTag::Receipts, MessageTag::Sales, MessageTag::Work], has_attachment: true,
    },
    Message {
        id: "m41", day: "Earlier in April",
        from: "Calendar", from_addr: "calendar@workspace.app", initials: "CA",
        subject: "Weekly summary · Mar 30 – Apr 5",
        snippet: "27 events, 18h 45m total. Most time on: 1:1s (5h 30m), eng standups (2h 30m), customer calls (3h 15m). 4 conflicts auto-resolved.",
        body: "Weekly summary · Mar 30 – Apr 5\n\nEvents: 27\nTotal time in meetings: 18h 45m (37% of work week)\n\nBreakdown:\n— 1:1s · 5h 30m\n— Eng standups · 2h 30m\n— Customer calls · 3h 15m\n— Design reviews · 1h 45m\n— Architecture sync · 2h\n— Other · 3h 45m\n\nConflicts auto-resolved: 4",
        time: "Apr 2", full_time: "Apr 2, 2026 · 8:00 AM",
        unread: false, starred: false, thread_count: 1,
        tags: &[MessageTag::Calendar, MessageTag::Work, MessageTag::Newsletter], has_attachment: false,
    },
    Message {
        id: "m42", day: "Earlier in April",
        from: "Hacker News Daily", from_addr: "digest@hnrss.com", initials: "HN",
        subject: "Top 10 from HN · Apr 1",
        snippet: "1. 'I rewrote my email client in Rust' (842 pts) · 2. SQLite 3.50 release · 3. Why typescript 'satisfies' is everywhere now · …",
        body: "Top 10 from HN · Apr 1\n\n1. I rewrote my email client in Rust (842 pts, 312 comments)\n2. SQLite 3.50 release notes (614 pts)\n3. Why TypeScript 'satisfies' is everywhere now (582 pts)\n4. Vercel acquires Lockfile (471 pts)\n5. The hidden cost of context windows (398 pts)\n6. Show HN: Local-first vector DB in Go (354 pts)\n7. Apple's new memory protection chip (322 pts)\n8. Postgres 17 hash join improvements (301 pts)\n9. The case against monorepos, revisited (288 pts)\n10. A new approach to GUI testing (264 pts)",
        time: "Apr 1", full_time: "Apr 1, 2026 · 7:00 AM",
        unread: false, starred: false, thread_count: 1,
        tags: &[MessageTag::Newsletter, MessageTag::Github], has_attachment: false,
    },
    Message {
        id: "m43", day: "March",
        from: "Caroline Wu", from_addr: "caroline@yourcompany.com", initials: "CW",
        subject: "Q1 perf review · self-eval prompt",
        snippet: "Reminder: Q1 self-eval is due Friday. Three short prompts: 'biggest impact', 'biggest learning', 'where you want to grow'.",
        body: "Q1 self-eval prompt — due Fri Apr 4\n\nThree prompts (200 words each):\n1. Biggest impact this quarter\n2. Biggest learning\n3. Where you want to grow next quarter\n\nNo word salad — short and concrete is better. We'll use these in your 1:1.\n\n— C",
        time: "Mar 30", full_time: "Mar 30, 2026 · 4:14 PM",
        unread: false, starred: false, thread_count: 2,
        tags: &[MessageTag::Work, MessageTag::Urgent, MessageTag::Calendar], has_attachment: false,
    },
    Message {
        id: "m44", day: "March",
        from: "Vercel", from_addr: "deploys@vercel.com", initials: "VC",
        subject: "Production deploy succeeded · web@b73d1f4",
        snippet: "Deployment to production completed in 1m 48s. 0 errors, 4 warnings. Edge functions: 12 deployed.",
        body: "Production deploy · web@b73d1f4\nBuild time: 1m 48s\nErrors: 0\nWarnings: 4\nEdge functions: 12 deployed\n\nView build → vercel.com/deploys/b73d1f4",
        time: "Mar 28", full_time: "Mar 28, 2026 · 11:48 AM",
        unread: false, starred: false, thread_count: 1,
        tags: &[MessageTag::Deploys, MessageTag::Work], has_attachment: false,
    },
    Message {
        id: "m45", day: "March",
        from: "Maya Chen", from_addr: "maya@figma-internal.com", initials: "MC",
        subject: "Initial mood board for mail v2",
        snippet: "Started a mood board pulling from Things, Linear, and a few editorial layouts. Want your gut reactions before I commit to a direction.",
        body: "Started a mood board pulling from Things, Linear, and a few editorial layouts.\n\nThe high-level question: are we trying to feel like a productivity app (Linear/Things) or like a reading experience (NYT/Substack)? Different visual systems entirely.\n\nLink → figma.com/file/mail-v2-moodboard\n\nWant your gut reactions before I commit to a direction.\n— M",
        time: "Mar 26", full_time: "Mar 26, 2026 · 2:18 PM",
        unread: false, starred: true, thread_count: 8,
        tags: &[MessageTag::Design, MessageTag::Work], has_attachment: true,
    },
    Message {
        id: "m46", day: "March",
        from: "GitHub", from_addr: "noreply@github.com", initials: "GH",
        subject: "[anthropics/claude-cookbook] PR #463 merged",
        snippet: "Your PR #463 'Add cookbook for parallel tool calls' was merged into main by samir-kapoor.",
        body: "Your PR was merged.\n\nPR #463 · Add cookbook for parallel tool calls\nMerged by: samir-kapoor\nCommits: 8\n+ 412 lines · − 23 lines\n\nThanks for the contribution!",
        time: "Mar 25", full_time: "Mar 25, 2026 · 5:12 PM",
        unread: false, starred: false, thread_count: 1,
        tags: &[MessageTag::Github, MessageTag::Work, MessageTag::Deploys], has_attachment: false,
    },
    Message {
        id: "m47", day: "March",
        from: "Pedro Carvalho", from_addr: "pedro@yourcompany.com", initials: "PC",
        subject: "Friday team lunch · Mediterranean?",
        snippet: "Thinking Souvla on Divisadero this Friday at 12:30. 8 of us so far. Reply if you're in or have a veto.",
        body: "Thinking Souvla on Divisadero this Friday at 12:30.\n\n8 of us so far: me, Caroline, Marta, Eli, Samir, Maya, you, and one of the new hires.\n\nReply if you're in or have a veto.\n\n— P",
        time: "Mar 24", full_time: "Mar 24, 2026 · 10:45 AM",
        unread: false, starred: false, thread_count: 5,
        tags: &[MessageTag::Calendar], has_attachment: false,
    },
    Message {
        id: "m48", day: "March",
        from: "1Password", from_addr: "security@1password.com", initials: "1P",
        subject: "Password compromised · airbnb.com",
        snippet: "Your password for airbnb.com was found in a recent data breach (LinkedIn-affiliated dataset, 2.7B records). Change it now.",
        body: "Password compromised\n\nSite: airbnb.com\nFound in: LinkedIn-affiliated dataset (2.7B records, surfaced this week)\n\nRecommended action: change password immediately and rotate any shared accounts.\n\nChange now → 1password.com/sites/airbnb",
        time: "Mar 22", full_time: "Mar 22, 2026 · 8:14 AM",
        unread: false, starred: false, thread_count: 1,
        tags: &[MessageTag::Security, MessageTag::Urgent, MessageTag::Alerts], has_attachment: false,
    },
    Message {
        id: "m49", day: "March",
        from: "Linear", from_addr: "notifications@linear.app", initials: "LN",
        subject: "Cycle 46 closed · 14 of 16 issues completed (88%)",
        snippet: "Cycle 46 closed. 14 of 16 issues completed. Carryover: ENG-1184 (search migration), ENG-1192 (highlight bug). Velocity stable at 18 pts.",
        body: "Cycle 46 closed\n\nIssues completed: 14 of 16 (88%)\nVelocity: 18 pts (stable)\nCarryover:\n— ENG-1184 · search migration (4 pts)\n— ENG-1192 · highlight bug Safari (1 pt)\n\nReview → linear.app/eng/cycle/46",
        time: "Mar 20", full_time: "Mar 20, 2026 · 6:00 PM",
        unread: false, starred: false, thread_count: 1,
        tags: &[MessageTag::Work, MessageTag::Calendar, MessageTag::Newsletter], has_attachment: false,
    },
    Message {
        id: "m50", day: "March",
        from: "Notion", from_addr: "team@notion.so", initials: "NO",
        subject: "Eli shared 'Search v2 cutover plan' with you",
        snippet: "Eli Rosen gave you full-access to a page in the Engineering workspace.",
        body: "Eli Rosen (eli.r@yourcompany.com) gave you full-access to:\n\nSearch v2 cutover plan\nWorkspace: Engineering\n\nOpen page → notion.so/search-v2-cutover-plan",
        time: "Mar 18", full_time: "Mar 18, 2026 · 3:42 PM",
        unread: false, starred: false, thread_count: 1,
        tags: &[MessageTag::Work, MessageTag::Calendar], has_attachment: false,
    },
    Message {
        id: "m51", day: "March",
        from: "AWS Billing", from_addr: "no-reply@aws.amazon.com", initials: "AW",
        subject: "March bill is now available · $4,824.18",
        snippet: "Total estimated charges for March 2026: $4,824.18 (down $312 from Feb). Largest line: EC2 ($2,418), S3 ($914), CloudFront ($612).",
        body: "March 2026 bill\n\nTotal estimated charges: $4,824.18 (down $312 from Feb)\n\nTop services:\n— EC2: $2,418.00\n— S3: $914.20\n— CloudFront: $612.40\n— RDS: $462.10\n— Lambda: $214.00\n— Other: $203.48\n\nView detailed breakdown → console.aws.amazon.com/billing",
        time: "Mar 15", full_time: "Mar 15, 2026 · 11:00 PM",
        unread: false, starred: false, thread_count: 1,
        tags: &[MessageTag::Receipts, MessageTag::Work], has_attachment: false,
    },
    Message {
        id: "m52", day: "March",
        from: "Marta Liu", from_addr: "marta.l@yourcompany.com", initials: "ML",
        subject: "Thanks for the demo prep!",
        snippet: "The Acme call went really well — Sarah gave us a verbal yes on the 240 seats. Your SSO/SCIM walkthrough sealed it.",
        body: "The Acme call went really well — Sarah gave us a verbal yes on the 240 seats.\n\nYour SSO/SCIM walkthrough sealed it. She specifically called out how clean our IdP integration story is vs. their current vendor.\n\nLegal will turn the contract around by EOW.\n\nThanks for jumping in on short notice.\n— Marta",
        time: "Mar 12", full_time: "Mar 12, 2026 · 5:48 PM",
        unread: false, starred: true, thread_count: 3,
        tags: &[MessageTag::Sales, MessageTag::Work, MessageTag::Security], has_attachment: false,
    },
    Message {
        id: "m53", day: "March",
        from: "Calendar", from_addr: "calendar@workspace.app", initials: "CA",
        subject: "Out of office: Pedro Carvalho · Mar 20 – Apr 4",
        snippet: "Pedro Carvalho will be out from Mar 20 to Apr 4 (Lisbon trip). Backup contact for indexer/search work: Eli Rosen.",
        body: "Out of office\n\nWho: Pedro Carvalho\nDates: Mar 20 – Apr 4 (Lisbon trip)\nBackup for indexer/search: Eli Rosen\nBackup for general questions: Caroline Wu",
        time: "Mar 10", full_time: "Mar 10, 2026 · 9:30 AM",
        unread: false, starred: false, thread_count: 1,
        tags: &[MessageTag::Calendar, MessageTag::Work], has_attachment: false,
    },
    Message {
        id: "m54", day: "March",
        from: "Sentry", from_addr: "alerts@sentry.io", initials: "SE",
        subject: "[web-prod] Issue resolved · UnhandledPromiseRejection",
        snippet: "The issue you fixed in commit 8a14b29 (Mar 8) hasn't reoccurred in 24h. Marking as resolved.",
        body: "Issue auto-resolved\n\nUnhandledPromiseRejection in MailList.fetchPage\nFix: commit 8a14b29 (Mar 8)\n24h without reoccurrence\n\nView → sentry.io/issues/web-prod/UnhandledPromiseRejection",
        time: "Mar 9", full_time: "Mar 9, 2026 · 2:14 PM",
        unread: false, starred: false, thread_count: 1,
        tags: &[MessageTag::Alerts, MessageTag::Work, MessageTag::Github], has_attachment: false,
    },
    Message {
        id: "m55", day: "March",
        from: "GitHub", from_addr: "noreply@github.com", initials: "GH",
        subject: "[anthropics/claude-cookbook] You triaged 12 issues",
        snippet: "Weekly recap: you triaged 12 issues, opened 2 PRs, reviewed 5 PRs. Top label: 'documentation' (7 issues).",
        body: "Weekly contributor recap\n\nYou this week:\n— Triaged: 12 issues\n— Opened PRs: 2\n— Reviewed PRs: 5\n— Comments: 23\n\nTop label this week: 'documentation' (7 issues)",
        time: "Mar 7", full_time: "Mar 7, 2026 · 8:00 AM",
        unread: false, starred: false, thread_count: 1,
        tags: &[MessageTag::Github, MessageTag::Work, MessageTag::Newsletter], has_attachment: false,
    },
    Message {
        id: "m56", day: "March",
        from: "Substack", from_addr: "no-reply@substack.com", initials: "SU",
        subject: "Read Max · Why every app wants to be email",
        snippet: "Max Read on the slow homogenization of consumer software around the inbox metaphor: notifications, threads, snoozing, archive.",
        body: "Read Max\n\nWhy every app wants to be email\n\nMax Read on the slow homogenization of consumer software around the inbox metaphor: notifications, threads, snoozing, archive. From Slack to Linear to Things, the pattern keeps winning. What that says about how we want to relate to information.\n\nRead → readmax.substack.com/p/email-pattern",
        time: "Mar 4", full_time: "Mar 4, 2026 · 9:00 AM",
        unread: false, starred: false, thread_count: 1,
        tags: &[MessageTag::Newsletter, MessageTag::Design], has_attachment: false,
    },
    Message {
        id: "m57", day: "March",
        from: "Cloudflare", from_addr: "noreply@cloudflare.com", initials: "CF",
        subject: "DDoS attack mitigated · 14M req/s peak",
        snippet: "Cloudflare mitigated a Layer 7 DDoS attack against api.yourcompany.com. Peak: 14M req/s. Duration: 47 min. No customer impact.",
        body: "DDoS attack mitigated\n\nTarget: api.yourcompany.com\nLayer: 7 (HTTPS)\nPeak: 14M req/s\nDuration: 47 min (start 03:14 UTC)\nCustomer impact: none\n\nReport → dash.cloudflare.com/security/ddos",
        time: "Mar 2", full_time: "Mar 2, 2026 · 4:01 AM",
        unread: false, starred: false, thread_count: 1,
        tags: &[MessageTag::Security, MessageTag::Alerts, MessageTag::Urgent], has_attachment: false,
    },
    Message {
        id: "m58", day: "February",
        from: "Caroline Wu", from_addr: "caroline@yourcompany.com", initials: "CW",
        subject: "1:1 notes · Feb 26",
        snippet: "Notes from our 1:1: focus on landing search-v2 cleanly, push the perf review prompts to mid-Q2, talk to Maya about design partnership.",
        body: "Notes from our 1:1 (Feb 26)\n\nFocus areas through end of Q1:\n— Land search-v2 cleanly (tracking ENG-1184)\n— Mentor Eli on the indexer side\n— Push perf review prompts to mid-Q2 (low priority)\n\nLonger term:\n— Pair with Maya on the mail v2 design partnership\n— Q2 OKR: ship one user-facing feature end-to-end\n\nNext 1:1: Mar 5",
        time: "Feb 26", full_time: "Feb 26, 2026 · 11:30 AM",
        unread: false, starred: false, thread_count: 1,
        tags: &[MessageTag::Work, MessageTag::Calendar], has_attachment: false,
    },
    Message {
        id: "m59", day: "February",
        from: "Stripe", from_addr: "receipts@stripe.com", initials: "ST",
        subject: "Subscription renewed · Anthropic Pro · $20.00",
        snippet: "Your Anthropic Pro subscription renewed for $20.00 on Visa •• 4242. Next renewal: Mar 25, 2026.",
        body: "Subscription renewed\n\nAnthropic Pro\n$20.00 on Visa •• 4242\n\nPeriod: Feb 25 – Mar 25, 2026\n\nManage subscription → stripe.com/customers/sub_4Hk2",
        time: "Feb 25", full_time: "Feb 25, 2026 · 8:14 AM",
        unread: false, starred: false, thread_count: 1,
        tags: &[MessageTag::Receipts, MessageTag::Work], has_attachment: false,
    },
    Message {
        id: "m60", day: "February",
        from: "Calendar", from_addr: "calendar@workspace.app", initials: "CA",
        subject: "Birthday reminder · Marta Liu (Mar 8)",
        snippet: "Marta Liu's birthday is in 11 days (Mar 8). Want to add a reminder or schedule a team gesture?",
        body: "Birthday reminder\n\nMarta Liu · Mar 8 (11 days away)\n\nLast year you organized a team lunch. Want to do the same?\n\nOptions:\n— Add a calendar reminder\n— Send a card from the team\n— Schedule a team lunch",
        time: "Feb 25", full_time: "Feb 25, 2026 · 7:00 AM",
        unread: false, starred: false, thread_count: 1,
        tags: &[MessageTag::Calendar, MessageTag::Work], has_attachment: false,
    },
];

#[derive(Clone, Copy, PartialEq)]
pub enum IconKind {
    Inbox,
    Send,
    Pen,
    Archive,
    Trash,
    StarOutline,
    StarFilled,
    Paperclip,
    More,
    Filter,
    ArrowLeft,
    Flag,
    Snooze,
}

#[component]
pub fn LucideIcon(kind: IconKind, #[props(default = 16)] size: u32) -> Element {
    let size_str = format!("{size}px");
    let (fill, paths) = match kind {
        IconKind::Inbox => (
            "none",
            rsx! {
                path { d: "M22 12h-6l-2 3h-4l-2-3H2" }
                path { d: "M5.45 5.11 2 12v6a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2v-6l-3.45-6.89A2 2 0 0 0 16.76 4H7.24a2 2 0 0 0-1.79 1.11Z" }
            },
        ),
        IconKind::Send => (
            "none",
            rsx! {
                path { d: "m22 2-7 20-4-9-9-4Z" }
                path { d: "M22 2 11 13" }
            },
        ),
        IconKind::Pen => (
            "none",
            rsx! {
                path { d: "M12 20h9" }
                path { d: "M16.5 3.5a2.121 2.121 0 1 1 3 3L7 19l-4 1 1-4Z" }
            },
        ),
        IconKind::Archive => (
            "none",
            rsx! {
                rect { x: "2", y: "4", width: "20", height: "5", rx: "2" }
                path { d: "M4 9v9a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V9" }
                path { d: "M10 13h4" }
            },
        ),
        IconKind::Trash => (
            "none",
            rsx! {
                path { d: "M3 6h18" }
                path { d: "M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6" }
                path { d: "M8 6V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2" }
            },
        ),
        IconKind::StarOutline => (
            "none",
            rsx! {
                path { d: "m12 2 3.09 6.26L22 9.27l-5 4.87 1.18 6.88L12 17.77l-6.18 3.25L7 14.14 2 9.27l6.91-1.01L12 2z" }
            },
        ),
        IconKind::StarFilled => (
            "currentColor",
            rsx! {
                path { d: "m12 2 3.09 6.26L22 9.27l-5 4.87 1.18 6.88L12 17.77l-6.18 3.25L7 14.14 2 9.27l6.91-1.01L12 2z" }
            },
        ),
        IconKind::Paperclip => (
            "none",
            rsx! {
                path { d: "m21.44 11.05-9.19 9.19a6 6 0 0 1-8.49-8.49l8.57-8.57A4 4 0 1 1 17.99 8.83l-8.59 8.57a2 2 0 1 1-2.83-2.83l8.49-8.48" }
            },
        ),
        IconKind::More => (
            "none",
            rsx! {
                circle { cx: "12", cy: "12", r: "1" }
                circle { cx: "12", cy: "5", r: "1" }
                circle { cx: "12", cy: "19", r: "1" }
            },
        ),
        IconKind::Filter => (
            "none",
            rsx! {
                polygon { points: "22 3 2 3 10 12.46 10 19 14 21 14 12.46 22 3" }
            },
        ),
        IconKind::ArrowLeft => (
            "none",
            rsx! {
                path { d: "m12 19-7-7 7-7" }
                path { d: "M19 12H5" }
            },
        ),
        IconKind::Flag => (
            "none",
            rsx! {
                path { d: "M4 15s1-1 4-1 5 2 8 2 4-1 4-1V3s-1 1-4 1-5-2-8-2-4 1-4 1z" }
                line { x1: "4", y1: "22", x2: "4", y2: "15" }
            },
        ),
        IconKind::Snooze => (
            "none",
            rsx! {
                circle { cx: "12", cy: "13", r: "8" }
                path { d: "M5 3 2 6" }
                path { d: "m22 6-3-3" }
                path { d: "M12 9v4l2 2" }
            },
        ),
    };

    rsx! {
        Icon {
            width: "{size_str}",
            height: "{size_str}",
            fill,
            stroke_width: 1.75,
            "aria-hidden": "true",
            {paths}
        }
    }
}
