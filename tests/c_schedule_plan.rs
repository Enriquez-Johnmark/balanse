//! C1, C3–C6: which reminder step, if any, a Debtor is due today. The
//! planner is pure: the scheduler gives it the facts and acts on its answer.

#![allow(clippy::unwrap_used, reason = "test code: a failed unwrap is a failed test")]

use balanse::balanse::schedule::{Next, Plan, Step, plan};
use time::macros::date;

/// A Debt due Thursday 2026-10-15 with no history and no heads-up.
fn fresh() -> Plan {
    Plan {
        today: date!(2026 - 10 - 16),
        cycle_start: date!(2026 - 10 - 15),
        due: date!(2026 - 10 - 15),
        heads_up: false,
        done: Vec::new(),
        waiting: false,
        sent_on: Vec::new(),
    }
}

#[test]
fn c1_the_first_reminder_is_due_the_day_after_the_due_date() {
    assert_eq!(plan(&fresh()), Next::Draft(Step::First));
}

#[test]
fn c1_nothing_is_due_on_the_due_date_itself() {
    let on_due = Plan {
        today: date!(2026 - 10 - 15),
        ..fresh()
    };

    assert_eq!(plan(&on_due), Next::Nothing);
}

#[test]
fn c1_the_second_reminder_is_due_seven_days_after_the_due_date() {
    let day_7 = Plan {
        today: date!(2026 - 10 - 22),
        done: vec![Step::First],
        sent_on: vec![date!(2026 - 10 - 16)],
        ..fresh()
    };

    assert_eq!(plan(&day_7), Next::Draft(Step::Second));
}

#[test]
fn c1_the_third_reminder_is_due_fourteen_days_after_the_due_date() {
    let day_14 = Plan {
        today: date!(2026 - 10 - 29),
        done: vec![Step::First, Step::Second],
        sent_on: vec![date!(2026 - 10 - 16), date!(2026 - 10 - 22)],
        ..fresh()
    };

    assert_eq!(plan(&day_14), Next::Draft(Step::Third));
}

#[test]
fn c1_a_step_waits_while_an_earlier_draft_is_still_waiting() {
    let waiting = Plan {
        today: date!(2026 - 10 - 22),
        done: vec![Step::First],
        waiting: true,
        ..fresh()
    };

    assert_eq!(plan(&waiting), Next::Nothing);
}

#[test]
fn c3_after_three_reminders_the_debtor_needs_the_owners_call() {
    let after_third = Plan {
        today: date!(2026 - 11 - 05),
        done: vec![Step::First, Step::Second, Step::Third],
        ..fresh()
    };

    assert_eq!(plan(&after_third), Next::NeedsCall);
}

#[test]
fn c3_the_debtor_isnt_marked_while_the_third_draft_still_waits() {
    let third_waiting = Plan {
        today: date!(2026 - 10 - 30),
        done: vec![Step::First, Step::Second, Step::Third],
        waiting: true,
        ..fresh()
    };

    assert_eq!(plan(&third_waiting), Next::Nothing);
}

#[test]
fn c4_a_step_waits_while_a_message_went_out_in_the_last_three_days() {
    let sent_yesterday = Plan {
        sent_on: vec![date!(2026 - 10 - 15)],
        ..fresh()
    };

    assert_eq!(plan(&sent_yesterday), Next::Wait);
}

#[test]
fn c4_a_step_goes_ahead_once_three_days_have_passed() {
    let sent_three_days_ago = Plan {
        sent_on: vec![date!(2026 - 10 - 13)],
        ..fresh()
    };

    assert_eq!(plan(&sent_three_days_ago), Next::Draft(Step::First));
}

#[test]
fn c4_a_step_waits_after_two_messages_in_seven_days() {
    let two_this_week = Plan {
        today: date!(2026 - 10 - 22),
        done: vec![Step::First],
        sent_on: vec![date!(2026 - 10 - 16), date!(2026 - 10 - 19)],
        ..fresh()
    };

    assert_eq!(plan(&two_this_week), Next::Wait);
}

#[test]
fn c5_a_step_falling_on_a_saturday_moves_to_monday() {
    // Due Friday 2026-10-16: the first reminder falls on Saturday the 17th.
    let saturday = Plan {
        today: date!(2026 - 10 - 17),
        cycle_start: date!(2026 - 10 - 16),
        due: date!(2026 - 10 - 16),
        ..fresh()
    };
    let monday = Plan {
        today: date!(2026 - 10 - 19),
        ..saturday.clone()
    };

    assert_eq!(
        (plan(&saturday), plan(&monday)),
        (Next::Nothing, Next::Draft(Step::First))
    );
}

#[test]
fn c6_the_heads_up_is_off_by_default() {
    let three_days_before = Plan {
        today: date!(2026 - 10 - 12),
        ..fresh()
    };

    assert_eq!(plan(&three_days_before), Next::Nothing);
}

#[test]
fn c6_when_turned_on_the_heads_up_is_due_three_days_before() {
    let three_days_before = Plan {
        today: date!(2026 - 10 - 12),
        heads_up: true,
        ..fresh()
    };

    assert_eq!(plan(&three_days_before), Next::Draft(Step::HeadsUp));
}

#[test]
fn c6_a_debt_added_after_its_heads_up_day_skips_the_heads_up() {
    // The cycle starts later than the due date: the Debt was entered overdue.
    let late = Plan {
        today: date!(2026 - 10 - 21),
        cycle_start: date!(2026 - 10 - 20),
        heads_up: true,
        ..fresh()
    };

    assert_eq!(plan(&late), Next::Draft(Step::First));
}
