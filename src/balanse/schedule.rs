//! The reminder schedule (C1–C6), as a pure function of the facts about one
//! Debtor. The scheduler gathers the facts, asks [`plan`], and acts.
//!
//! - A cycle starts on its *cycle start*: normally the earliest open Debt's
//!   due date. The first reminder is due the day after, the second 7 days
//!   after and the third 14 days after (C1).
//! - The optional heads-up is due 3 days before the due date (C6). It only
//!   belongs to a cycle that starts on the due date, and is skipped once the
//!   due date arrives.
//! - A step falling on a Saturday or Sunday moves to the Monday (C5).
//! - Nothing new is drafted while a Draft still waits for approval or
//!   sending; once all three reminders are done, the Debtor needs the
//!   Owner's call (C3).
//! - At most one message in any 3 days and two in any 7 days, counting
//!   messages sent by hand; a step that would break that waits (C4).

use time::{Date, Duration, Weekday};

/// One step of a reminder cycle.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Step {
    /// A courtesy note before the due date.
    HeadsUp,
    /// The day after the cycle starts.
    First,
    /// 7 days after.
    Second,
    /// 14 days after.
    Third,
}

impl Step {
    /// The stored name.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::HeadsUp => "heads_up",
            Self::First => "first",
            Self::Second => "second",
            Self::Third => "third",
        }
    }

    /// The step with this stored name.
    pub fn parse(text: &str) -> Option<Self> {
        match text {
            "heads_up" => Some(Self::HeadsUp),
            "first" => Some(Self::First),
            "second" => Some(Self::Second),
            "third" => Some(Self::Third),
            _ => None,
        }
    }
}

/// The facts about one active Debtor with open Debts.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Plan {
    /// Today, in Manila.
    pub today: Date,
    /// When the current cycle started.
    pub cycle_start: Date,
    /// The earliest open Debt's due date.
    pub due: Date,
    /// Whether the Business sends heads-ups.
    pub heads_up: bool,
    /// The steps of this cycle already drafted, whatever became of them.
    pub done: Vec<Step>,
    /// Whether one of the Debtor's Drafts still waits to be approved or sent.
    pub waiting: bool,
    /// The Manila dates of every message sent to the Debtor in the last 7
    /// days, by the engine or by hand.
    pub sent_on: Vec<Date>,
}

/// What to do for the Debtor today.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Next {
    /// Draft this step now.
    Draft(Step),
    /// A step is due, but the cap says not today.
    Wait,
    /// All three reminders are done: stop and ask the Owner.
    NeedsCall,
    /// Nothing is due.
    Nothing,
}

/// The most messages to a Debtor in any 3 days.
const PER_3_DAYS: usize = 1;
/// The most messages to a Debtor in any 7 days.
const PER_7_DAYS: usize = 2;

/// What to do for the Debtor described by `facts`.
pub fn plan(facts: &Plan) -> Next {
    let mut steps = Vec::with_capacity(4);
    if facts.heads_up && facts.cycle_start == facts.due && facts.today < facts.due {
        steps.push((Step::HeadsUp, facts.due - Duration::days(3)));
    }
    steps.extend([
        (Step::First, facts.cycle_start + Duration::days(1)),
        (Step::Second, facts.cycle_start + Duration::days(7)),
        (Step::Third, facts.cycle_start + Duration::days(14)),
    ]);
    let Some((step, day)) = steps.into_iter().find(|(step, _)| !facts.done.contains(step)) else {
        return if facts.waiting { Next::Nothing } else { Next::NeedsCall };
    };
    if facts.waiting || facts.today < weekday_on_or_after(day) {
        return Next::Nothing;
    }
    if !cap_allows(facts.today, &facts.sent_on) {
        return Next::Wait;
    }
    Next::Draft(step)
}

/// Whether another message may go to a Debtor on `today`, given the dates
/// its earlier messages went out (C4).
pub fn cap_allows(today: Date, sent_on: &[Date]) -> bool {
    let since = |days: i64| {
        sent_on
            .iter()
            .filter(|&&d| d > today - Duration::days(days) && d <= today)
            .count()
    };
    since(3) < PER_3_DAYS && since(7) < PER_7_DAYS
}

/// `day`, or the Monday after if it falls on a weekend (C5).
pub fn weekday_on_or_after(day: Date) -> Date {
    match day.weekday() {
        Weekday::Saturday => day + Duration::days(2),
        Weekday::Sunday => day + Duration::days(1),
        _ => day,
    }
}
