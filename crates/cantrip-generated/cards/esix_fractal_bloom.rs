// Esix, Fractal Bloom
// status: planned
// requires: event change in the program of another event
// requires: replacement event FirstEachTurn
// requires: token recipe Copy
// Flying
// The first time you would create one or more tokens during each of your turns, you may instead choose a creature other than Esix and create that many tokens that are copies of that creature.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_0_applies(q: &dyn Query, asker: &Asker, proposal: &Proposal) -> bool { if let Proposal::TokenCreation(t) = proposal { let subject = Target::Player(t.controller); let data = [t.count, u32::MAX]; if t.count > 0 && true && Some(asker.controller) == Some(t.controller) && Some(asker.controller) == Some(q.state().turn.active) { return true; } }
 false }
#[allow(unused_variables, unused_mut)] fn ability_1_0_apply(q: &dyn Query, asker: &Asker, proposal: &mut Proposal) { let seen = *proposal; if let Proposal::TokenCreation(seen) = &seen { let t = seen; let subject = Target::Player(t.controller); let data = [t.count, u32::MAX]; if let Proposal::TokenCreation(t) = proposal {  } } }
planned_item!("event-change-in-the-program-of-another-event" => #[allow(unused_variables)] fn event_edit_0(q: &dyn Query, asker: &Asker, affected: ObjectId, subject: Target, data: [u32; 2], proposal: &mut Proposal) -> () { if let Proposal::TokenCreation(t) = proposal { t.count = t.count.saturating_mul(1); } });
#[allow(unused_variables, unused_mut)] fn ability_1_0_then_steps(g: &mut dyn Game, asker: &Asker, subject: Target, data: [u32; 2]) -> Result<(), Suspend> { let mut paid_0 = false; {
let selected_1_player = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let selected_1_candidates = { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| (q.chars(id).is(CardType::Creature) && !(Some(asker.source) == Some(id)))).collect::<Vec<ObjectId>>() }; let selected_1_count = { let q: &dyn Query = g; cantrip_mechanics::select::Count::UpTo((1i64).max(0) as u32) }; let selected_1 = { cantrip_mechanics::choose::select(g, selected_1_player, asker.source, Purpose::ChooseAsEnters, selected_1_candidates, cantrip_mechanics::select::Selection::of(selected_1_count, &[]))? };
if !selected_1.is_empty() { paid_0 = true;
}
}
if { let q: &dyn Query = g; ((paid_0 as i64) != 0) } {
planned!("token-recipe-copy", ModifyEvent, "change" = "{\"ReplaceTokens\":{\"copies\":0,\"tokens\":[{\"amount\":{\"kind\":\"EventAmount\"},\"recipe\":{\"Copy\":{\"adjustments\":[{\"args\":{\"family\":\"TokenCopyEffects\",\"form\":\"NoTokenCopyEffects\",\"operands\":[]},\"kind\":\"Term\"}],\"object\":{\"args\":{\"index\":0,\"phase\":\"After\",\"result\":1048576},\"kind\":\"Result\"}}},\"words\":\"CreatingEffect\"}]}}");
} else {
planned!("event-change-in-the-program-of-another-event" => g.change_event(event_edit_0););
}
 Ok(()) }
#[allow(unused_variables)] fn ability_1_0_then(g: &mut dyn Game, asker: &Asker, before: &Proposal, _: &Proposal) { if let Proposal::TokenCreation(t) = before { let q: &dyn Query = g; let subject = Target::Player(t.controller); let data = [t.count, u32::MAX]; let _ = ability_1_0_then_steps(g, asker, subject, data); } }
planned_item!("replacement-event-first-each-turn" => replacements: replacement(ReplacementKind::TokenCreation, ability_1_0_applies, ability_1_0_apply, "The first time you would create one or more tokens during each of your turns, you may instead choose a creature other than Esix and create that many tokens that are copies of that creature.").then(ability_1_0_then).first_each_turn());

pub static CARD: CardDef = CardDef { name: "Esix, Fractal Bloom", token_words: &[], oracle: true, cost: ManaCost { generic: 4, x: 0, symbols: [0, 1, 0, 0, 1, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Fractal]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(4), toughness: Some(4), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Flying]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
