use super::*;
use serde_json::json;

const BASE: i64 = 1_800_000_000;

fn at(seconds: i64) -> Instant {
    Instant::from_unix_seconds(BASE + seconds).unwrap()
}

fn notes(value: &str) -> Notes {
    value.to_owned().try_into().unwrap()
}

fn reference() -> ArtifactReference {
    ArtifactReference::Text(notes("synthetic-result.tex"))
}

fn key(item: &str, dimension: Dimension) -> ScopeKey {
    ScopeKey {
        item: item.to_owned().try_into().unwrap(),
        dimension,
    }
}

fn plan() -> ActionPlan {
    ActionPlan {
        instruction: notes("Construct one original proof"),
        expected_artifact: Some(notes("A proof in the existing source file")),
        verification_method: Some(notes("Check each implication independently")),
        start_reference: Some(reference()),
    }
}

fn schedule(d: i64, e: i64, b: u32) -> Schedule {
    ScheduleInput {
        deadline: Some(TimeFact::Instant(at(d))),
        earliest_finish: Some(TimeFact::Instant(at(e))),
        buffer_minutes: Some(b.try_into().unwrap()),
    }
    .try_into()
    .unwrap()
}

fn prepared(keys: Vec<ScopeKey>) -> Readiness {
    let mut readiness = Readiness::unknown(at(0));
    readiness.identify(keys.clone(), at(0)).unwrap();
    for key in &keys {
        readiness.map(key, reference(), at(0)).unwrap();
        readiness.deploy(key, reference(), at(0)).unwrap();
    }
    readiness
}

fn evidence(
    readiness: &Readiness,
    key: &ScopeKey,
    stage: EvidenceStage,
    outcome: Outcome,
    at: Instant,
) -> Evidence {
    Evidence::new(
        EvidenceInput {
            key: key.clone(),
            stage,
            method: notes("Independent synthetic test"),
            artifact: reference(),
            outcome,
        },
        "synthetic-actor".to_owned().try_into().unwrap(),
        readiness.revision(),
        at,
    )
}

fn pass(readiness: &mut Readiness, key: &ScopeKey) {
    let test = evidence(
        readiness,
        key,
        EvidenceStage::StressTest,
        Outcome::Pass,
        at(0),
    );
    readiness.record(test).unwrap();
    let verification = evidence(
        readiness,
        key,
        EvidenceStage::Verification,
        Outcome::Pass,
        at(0),
    );
    readiness.record(verification).unwrap();
}

#[test]
fn text_and_identifier_bounds_are_enforced_in_json() {
    for value in ["", "  ", "bad\u{0000}text", "\u{001b}control"] {
        assert!(serde_json::from_value::<Title>(json!(value)).is_err());
    }
    assert!(serde_json::from_value::<Title>(json!("a".repeat(160))).is_ok());
    assert!(serde_json::from_value::<Title>(json!("a".repeat(161))).is_err());
    assert!(serde_json::from_value::<Title>(json!("é".repeat(81))).is_err());
    assert_eq!(
        serde_json::from_value::<Title>(json!("  synthetic  "))
            .unwrap()
            .as_str(),
        "synthetic"
    );
    for value in ["", "../private", "bad id", "a".repeat(65).as_str()] {
        assert!(serde_json::from_value::<Identifier>(json!(value)).is_err());
    }
    assert!(serde_json::from_value::<Identifier>(json!("scope-1_written")).is_ok());
}

#[test]
fn web_references_reject_executable_and_token_locations() {
    for value in [
        "javascript:alert(1)",
        "data:text/html,hello",
        "file:///tmp/private",
        "https://user:password@example.invalid/path",
        "https://example.invalid/?token=private",
        "https://example.invalid/#private",
        "https://example.invalid/white space",
    ] {
        assert!(serde_json::from_value::<WebReference>(json!(value)).is_err());
    }
    assert!(
        serde_json::from_value::<WebReference>(json!("https://example.invalid/source")).is_ok()
    );
    // Text is deliberately non-clickable, not an executable URL type.
    assert!(
        serde_json::from_value::<ArtifactReference>(
            json!({"kind":"text","value":"Obsidian note: synthetic"})
        )
        .is_ok()
    );
}

#[test]
fn capture_can_leave_details_unknown_but_rejects_forged_fields() {
    assert!(
        serde_json::from_value::<ResolutionSpec>(json!({"title":"Synthetic resolution"})).is_ok()
    );
    assert!(
        serde_json::from_value::<CommitmentSpec>(json!({"title":"Incoming","kind":"unclassified"}))
            .is_ok()
    );
    assert!(
        serde_json::from_value::<ResolutionSpec>(
            json!({"title":"Synthetic", "owner":"another-member"})
        )
        .is_err()
    );
    assert!(
        serde_json::from_value::<EvidenceInput>(json!({"key":{"item":"i","dimension":"oral"},
        "stage":"verification","method":"test","artifact":{"kind":"text","value":"output"},
        "outcome":"pass","actor":"forged"}))
        .is_err()
    );
}

#[test]
fn instants_normalize_offsets_without_guessing_dates() {
    let a: Instant = "2026-10-08T12:30:00+02:00".to_owned().try_into().unwrap();
    let b: Instant = "2026-10-08T10:30:00Z".to_owned().try_into().unwrap();
    assert_eq!(a, b);
    assert_eq!(String::from(a), "2026-10-08T10:30:00Z");
    for value in [
        "2026-10-08",
        "2026-10-08T10:30:00",
        "2026-10-08T10:30:00.5Z",
        "2026-02-30T00:00:00Z",
    ] {
        assert!(Instant::try_from(value.to_owned()).is_err());
    }
    for value in [
        "2026-02-29",
        "2026-13-01",
        "0000-01-01",
        "2026-1-01",
        "😀-01-01",
    ] {
        assert!(DateOnly::try_from(value.to_owned()).is_err());
    }
    let day = DateOnly::try_from("2028-02-29".to_owned()).unwrap();
    assert_eq!(String::from(day), "2028-02-29");
    let input = ScheduleInput {
        deadline: Some(TimeFact::DateOnly(day)),
        ..Default::default()
    };
    let schedule = Schedule::try_from(input).unwrap();
    assert_eq!(schedule.snapshot(at(0)).state, BufferState::TargetUnknown);
    assert_eq!(
        ScheduleInput::from(schedule).deadline,
        Some(TimeFact::DateOnly(day))
    );
}

#[test]
fn buffer_boundaries_are_exact_and_disjoint() {
    let schedule = schedule(600, 300, 2); // protected window begins at 480
    for (t, state) in [
        (299, BufferState::Healthy),
        (300, BufferState::Shrinking),
        (479, BufferState::Shrinking),
        (480, BufferState::Exhausted),
        (599, BufferState::Exhausted),
        (600, BufferState::Late),
        (601, BufferState::Late),
    ] {
        let snapshot = schedule.snapshot(at(t));
        assert_eq!(snapshot.state, state);
        assert_eq!(snapshot.deadline_reached, t >= 600);
        assert_eq!(snapshot.seconds_until_deadline, Some(600 - t));
    }
}

#[test]
fn zero_and_equal_target_buffers_do_not_invent_intervals() {
    let equal = schedule(600, 480, 2);
    assert_eq!(equal.snapshot(at(479)).state, BufferState::Healthy);
    assert_eq!(equal.snapshot(at(480)).state, BufferState::Exhausted);
    let zero = schedule(600, 300, 0);
    assert_eq!(zero.snapshot(at(599)).state, BufferState::Shrinking);
    assert_eq!(zero.snapshot(at(600)).state, BufferState::Late);
    let all_equal = schedule(600, 600, 0);
    assert_eq!(all_equal.snapshot(at(599)).state, BufferState::Healthy);
    assert_eq!(all_equal.snapshot(at(600)).state, BufferState::Late);
}

#[test]
fn every_incomplete_control_combination_remains_unknown() {
    for mask in 0..7 {
        let input = ScheduleInput {
            deadline: (mask & 1 != 0).then_some(TimeFact::Instant(at(600))),
            earliest_finish: (mask & 2 != 0).then_some(TimeFact::Instant(at(300))),
            buffer_minutes: (mask & 4 != 0).then_some(2.try_into().unwrap()),
        };
        let snapshot = Schedule::try_from(input).unwrap().snapshot(at(700));
        assert_eq!(snapshot.state, BufferState::TargetUnknown);
        assert_eq!(snapshot.deadline_reached, mask & 1 != 0);
    }
}

#[test]
fn malformed_and_inconsistent_controls_fail_before_storage() {
    for (deadline, earliest, buffer) in [(600, 601, 0), (600, 481, 2)] {
        assert_eq!(
            Schedule::try_from(ScheduleInput {
                deadline: Some(TimeFact::Instant(at(deadline))),
                earliest_finish: Some(TimeFact::Instant(at(earliest))),
                buffer_minutes: Some(buffer.try_into().unwrap()),
            }),
            Err(DomainError::InvalidSchedule)
        );
    }
    for value in [json!(-1), json!(525_601), json!(1.5)] {
        assert!(serde_json::from_value::<BufferMinutes>(value).is_err());
    }
    assert!(
        serde_json::from_value::<Schedule>(json!({"deadline":null,"invented_estimate":1})).is_err()
    );
    let oldest: Instant = "0001-01-01T00:00:00Z".to_owned().try_into().unwrap();
    assert_eq!(
        Schedule::try_from(ScheduleInput {
            deadline: Some(TimeFact::Instant(oldest)),
            buffer_minutes: Some(1.try_into().unwrap()),
            ..Default::default()
        }),
        Err(DomainError::InvalidSchedule)
    );
}

#[test]
fn buffer_formula_holds_over_many_valid_controls_and_times() {
    for e in [0, 120, 300, 600] {
        for b in 0..=(600 - e) / 60 {
            let schedule = schedule(600, e, b as u32);
            let p = 600 - b * 60;
            for t in -1..=601 {
                let expected = if t < e {
                    BufferState::Healthy
                } else if t < p {
                    BufferState::Shrinking
                } else if t < 600 {
                    BufferState::Exhausted
                } else {
                    BufferState::Late
                };
                assert_eq!(schedule.snapshot(at(t)).state, expected);
            }
        }
    }
}

#[test]
fn incomplete_actions_can_be_captured_then_made_executable() {
    let mut incomplete = plan();
    incomplete.expected_artifact = None;
    let mut action = Action::new(incomplete, at(0));
    assert_eq!(action.ready(at(1)), Err(DomainError::IncompleteAction));
    assert_eq!(action.start(at(1)), Err(DomainError::InvalidTransition));
    assert_eq!(action.status(), ActionStatus::New);
    action.update_plan(plan(), at(2)).unwrap();
    action.ready(at(2)).unwrap();
    action.start(at(3)).unwrap();
    action.complete(reference(), at(4)).unwrap();
    assert_eq!(action.status(), ActionStatus::Completed);
    assert_eq!(
        serde_json::to_value(&action).unwrap()["artifact"]["kind"],
        "text"
    );
    // Completion has no connection to overall readiness or verified finish.
    assert!(Readiness::unknown(at(4)).verified_finish().is_none());
}

#[test]
fn invalid_action_transitions_and_backwards_clocks_are_atomic() {
    let mut action = Action::new(plan(), at(10));
    let original = action.clone();
    assert_eq!(
        action.complete(reference(), at(11)),
        Err(DomainError::InvalidTransition)
    );
    assert_eq!(action.ready(at(9)), Err(DomainError::ClockWentBackwards));
    assert_eq!(action, original);
    action.ready(at(11)).unwrap();
    action.start(at(12)).unwrap();
    assert_eq!(
        action.update_plan(plan(), at(13)),
        Err(DomainError::InvalidTransition)
    );
    action.block(at(13)).unwrap();
    action.ready(at(14)).unwrap();
    action.start(at(15)).unwrap();
    action.complete(reference(), at(16)).unwrap();
    let completed = action.clone();
    assert_eq!(action.block(at(17)), Err(DomainError::InvalidTransition));
    assert_eq!(action.cancel(at(17)), Err(DomainError::InvalidTransition));
    assert_eq!(action, completed);
    action.reopen(at(17)).unwrap();
    let restored = serde_json::to_value(&action).unwrap();
    assert_eq!(restored["status"], "new");
    assert!(restored["completed_at"].is_null());
    action.cancel(at(18)).unwrap();
    assert_eq!(action.start(at(19)), Err(DomainError::InvalidTransition));
    action.reopen(at(19)).unwrap();
}

#[test]
fn scope_requires_bounded_unique_applicable_items_and_no_empty_readiness() {
    let k = key("item", Dimension::Written);
    let mut readiness = Readiness::unknown(at(0));
    assert_eq!(readiness.confirm(at(0)), Err(DomainError::NotReady));
    assert_eq!(
        readiness.identify(Vec::new(), at(0)),
        Err(DomainError::InvalidScope)
    );
    assert_eq!(
        readiness.identify(vec![k.clone(), k.clone()], at(0)),
        Err(DomainError::InvalidScope)
    );
    let oversized = (0..=MAX_SCOPE_ITEMS)
        .map(|i| key(&format!("item-{i}"), Dimension::Written))
        .collect();
    assert_eq!(
        readiness.identify(oversized, at(0)),
        Err(DomainError::InvalidScope)
    );
    readiness
        .identify(vec![k.clone(), key("item", Dimension::Oral)], at(0))
        .unwrap();
    assert_eq!(readiness.coverage(&k), Ok(CoverageStage::Identified));
    assert_eq!(
        readiness.deploy(&k, reference(), at(0)),
        Err(DomainError::MissingPrerequisite)
    );
    assert!(!readiness.eligible_for_confirmation());
}

#[test]
fn deployment_is_not_testing_and_testing_is_not_verification() {
    let k = key("item", Dimension::Written);
    let mut readiness = prepared(vec![k.clone()]);
    assert_eq!(readiness.coverage(&k), Ok(CoverageStage::Deployed));
    let premature = evidence(
        &readiness,
        &k,
        EvidenceStage::Verification,
        Outcome::Pass,
        at(0),
    );
    let original = readiness.clone();
    assert_eq!(
        readiness.record(premature),
        Err(DomainError::MissingPrerequisite)
    );
    assert_eq!(readiness, original);
    let test = evidence(
        &readiness,
        &k,
        EvidenceStage::StressTest,
        Outcome::Pass,
        at(0),
    );
    readiness.record(test).unwrap();
    assert_eq!(readiness.coverage(&k), Ok(CoverageStage::StressTested));
    assert_eq!(readiness.confirm(at(0)), Err(DomainError::NotReady));
    let verification = evidence(
        &readiness,
        &k,
        EvidenceStage::Verification,
        Outcome::Pass,
        at(0),
    );
    readiness.record(verification).unwrap();
    assert!(readiness.eligible_for_confirmation());
    assert!(readiness.verified_finish().is_none());
    readiness.confirm(at(1)).unwrap();
    assert_eq!(readiness.confirm(at(2)), Err(DomainError::NotReady));
}

#[test]
fn a_single_example_cannot_verify_other_scope_items_or_dimensions() {
    let written = key("item", Dimension::Written);
    let oral = key("item", Dimension::Oral);
    let mut readiness = prepared(vec![written.clone(), oral.clone()]);
    pass(&mut readiness, &written);
    assert_eq!(readiness.coverage(&written), Ok(CoverageStage::Verified));
    assert_eq!(readiness.coverage(&oral), Ok(CoverageStage::Deployed));
    assert_eq!(readiness.confirm(at(0)), Err(DomainError::NotReady));
    pass(&mut readiness, &oral);
    readiness.confirm(at(0)).unwrap();
}

#[test]
fn later_failures_and_new_tests_invalidate_the_verified_finish() {
    for stage in [EvidenceStage::StressTest, EvidenceStage::Verification] {
        let k = key("item", Dimension::Written);
        let mut readiness = prepared(vec![k.clone()]);
        pass(&mut readiness, &k);
        readiness.confirm(at(0)).unwrap();
        let failure = evidence(&readiness, &k, stage, Outcome::Fail, at(1));
        readiness.record(failure).unwrap();
        assert!(readiness.verified_finish().is_none());
        assert!(!readiness.eligible_for_confirmation());
        let expected = if stage == EvidenceStage::StressTest {
            CoverageStage::StressTestFailed
        } else {
            CoverageStage::VerificationFailed
        };
        assert_eq!(readiness.coverage(&k), Ok(expected));
    }
    let k = key("item", Dimension::Written);
    let mut readiness = prepared(vec![k.clone()]);
    pass(&mut readiness, &k);
    readiness.confirm(at(0)).unwrap();
    let test = evidence(
        &readiness,
        &k,
        EvidenceStage::StressTest,
        Outcome::Pass,
        at(1),
    );
    readiness.record(test).unwrap();
    assert_eq!(readiness.coverage(&k), Ok(CoverageStage::StressTested));
    assert!(readiness.verified_finish().is_none());
}

#[test]
fn scope_and_material_changes_make_old_attestations_stale() {
    let k = key("item", Dimension::Written);
    let mut readiness = prepared(vec![k.clone()]);
    pass(&mut readiness, &k);
    let old = evidence(
        &readiness,
        &k,
        EvidenceStage::Verification,
        Outcome::Pass,
        at(10),
    );
    readiness.confirm(at(0)).unwrap();
    readiness
        .identify(vec![k.clone(), key("new-item", Dimension::Oral)], at(1))
        .unwrap();
    assert_eq!(readiness.coverage(&k), Ok(CoverageStage::Stale));
    assert!(readiness.verified_finish().is_none());
    assert_eq!(readiness.record(old), Err(DomainError::StaleEvidence));
    let original = readiness.clone();
    assert_eq!(
        readiness.map(&key("absent", Dimension::Written), reference(), at(2)),
        Err(DomainError::UnknownScopeItem)
    );
    assert_eq!(readiness, original);
    readiness.map(&k, reference(), at(2)).unwrap();
    let test = evidence(
        &readiness,
        &k,
        EvidenceStage::StressTest,
        Outcome::Pass,
        at(2),
    );
    assert_eq!(
        readiness.record(test),
        Err(DomainError::MissingPrerequisite)
    );
    readiness.mark_scope_unknown(at(3)).unwrap();
    assert!(!readiness.eligible_for_confirmation());
}

#[test]
fn blockers_and_reopening_require_fresh_evidence() {
    let k = key("item", Dimension::Written);
    let mut readiness = prepared(vec![k.clone()]);
    pass(&mut readiness, &k);
    readiness.confirm(at(0)).unwrap();
    let original = readiness.clone();
    assert_eq!(
        readiness.set_blocking_threats(129, at(1)),
        Err(DomainError::InvalidScope)
    );
    assert_eq!(readiness, original);
    readiness.set_blocking_threats(1, at(1)).unwrap();
    assert!(readiness.verified_finish().is_none());
    readiness.set_blocking_threats(0, at(2)).unwrap();
    assert!(!readiness.eligible_for_confirmation());
    // Fresh evidence against the current revision, then a deliberate reopen.
    for stage in [EvidenceStage::StressTest, EvidenceStage::Verification] {
        let proof = evidence(&readiness, &k, stage, Outcome::Pass, at(3));
        readiness.record(proof).unwrap();
    }
    readiness.confirm(at(4)).unwrap();
    readiness.reopen(at(5)).unwrap();
    assert_eq!(readiness.coverage(&k), Ok(CoverageStage::Stale));
    assert!(readiness.verified_finish().is_none());
}

#[test]
fn finish_outcomes_are_frozen_and_deadline_equality_is_not_late() {
    for (finished, buffer, preserved, late) in [
        (479, 121, true, false),
        (480, 120, true, false),
        (481, 119, false, false),
        (600, 0, false, false),
        (601, -1, false, true),
    ] {
        let k = key("item", Dimension::Written);
        let mut readiness = prepared(vec![k.clone()]);
        pass(&mut readiness, &k);
        let finish = readiness.confirm(at(finished)).unwrap();
        let schedule = schedule(600, 300, 2);
        let outcome = schedule.finish_outcome(finish);
        assert_eq!(outcome.actual_buffer_seconds, Some(buffer));
        assert_eq!(outcome.target_preserved, Some(preserved));
        assert_eq!(outcome.late, Some(late));
        assert_eq!(schedule.finish_outcome(finish), outcome);
        assert_eq!(schedule.snapshot(at(999)).state, BufferState::Late);
        let unknown = Schedule::try_from(ScheduleInput::default())
            .unwrap()
            .finish_outcome(finish);
        assert!(unknown.actual_buffer_seconds.is_none());
        assert!(unknown.target_preserved.is_none());
        assert!(unknown.late.is_none());
    }
}

#[test]
fn rejected_evidence_and_backwards_readiness_edits_preserve_state() {
    let k = key("item", Dimension::Written);
    let mut readiness = prepared(vec![k.clone()]);
    readiness.set_blocking_threats(0, at(10)).unwrap();
    let original = readiness.clone();
    let past = evidence(
        &readiness,
        &k,
        EvidenceStage::StressTest,
        Outcome::Pass,
        at(9),
    );
    assert_eq!(readiness.record(past), Err(DomainError::ClockWentBackwards));
    assert_eq!(
        readiness.identify(vec![k.clone()], at(9)),
        Err(DomainError::ClockWentBackwards)
    );
    assert_eq!(readiness, original);
    let wrong_key = evidence(
        &readiness,
        &key("absent", Dimension::Oral),
        EvidenceStage::StressTest,
        Outcome::Pass,
        at(10),
    );
    assert_eq!(
        readiness.record(wrong_key),
        Err(DomainError::UnknownScopeItem)
    );
    assert_eq!(readiness, original);
}
