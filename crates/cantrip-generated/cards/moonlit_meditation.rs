// Moonlit Meditation
// status: planned
// requires: event change in the program of another event
// requires: instruction ChooseOne
// requires: replacement event FirstEachTurn
// Enchant artifact or creature you control
// The first time you would create one or more tokens each turn, you may instead create that many tokens that are copies of enchanted permanent.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_enchant(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if cantrip_mechanics::targets::permanent(q, id) && (((q.chars(id).is(CardType::Artifact) || q.chars(id).is(CardType::Creature)) && Some(asker.controller) == Some(q.controller(id))))) }
#[allow(unused_variables)] fn ability_1_applies(q: &dyn Query, asker: &Asker, proposal: &Proposal) -> bool { if let Proposal::TokenCreation(t) = proposal { let subject = Target::Player(t.controller); let data = [t.count, u32::MAX]; if t.count > 0 && true && Some(asker.controller) == Some(t.controller) { return true; } }
 false }
#[allow(unused_variables, unused_mut)] fn ability_1_apply(q: &dyn Query, asker: &Asker, proposal: &mut Proposal) { let seen = *proposal; if let Proposal::TokenCreation(seen) = &seen { let t = seen; let subject = Target::Player(t.controller); let data = [t.count, u32::MAX]; if let Proposal::TokenCreation(t) = proposal {  } } }
#[allow(unused_variables, unused_mut)] fn ability_1_then_steps(g: &mut dyn Game, asker: &Asker, subject: Target, data: [u32; 2]) -> Result<(), Suspend> { let decider_1 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") };
match planned_value!("instruction ChooseOne", 0u8 => cantrip_mechanics::choose::one_of(g, decider_1, asker.source, vec![Label::Card(card_1, 0), Label::Card(card_1, 1)])?) {
0 => {
planned!("event-change-in-the-program-of-another-event" => g.change_event(|q, proposal| { if let Proposal::TokenCreation(t) = proposal { if let Some(object) = Some(asker.source).and_then(|o: ObjectId| q.state().obj(o).attached_to) { let values = q.copiable_values(object); t.token = values.card; t.copy = Some(values); }  } }););
}
_ => {
planned!("event-change-in-the-program-of-another-event" => g.change_event(|q, proposal| { if let Proposal::TokenCreation(t) = proposal { t.count = t.count.saturating_mul(1); } }););
}
}
 Ok(()) }
#[allow(unused_variables)] fn ability_1_then(g: &mut dyn Game, asker: &Asker, before: &Proposal, _: &Proposal) { if let Proposal::TokenCreation(t) = before { let q: &dyn Query = g; let subject = Target::Player(t.controller); let data = [t.count, u32::MAX]; let _ = ability_1_then_steps(g, asker, subject, data); } }
planned_item!("replacement-event-first-each-turn" => replacements: replacement(ReplacementKind::TokenCreation, ability_1_applies, ability_1_apply, "The first time you would create one or more tokens each turn, you may instead create that many tokens that are copies of enchanted permanent.").then(ability_1_then).first_each_turn());

pub static CARD: CardDef = CardDef { name: "Moonlit Meditation", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 1, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Enchantment]), subtypes: SubtypeSet::of(&[Subtype::Aura]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: Some(target(ability_0_enchant)), ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
