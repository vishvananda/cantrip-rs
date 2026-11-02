// Mirrormind Crown
// status: planned
// requires: event change in the program of another event
// requires: replacement event FirstEachTurn
// As long as this Equipment is attached to a creature, the first time you would create one or more tokens each turn, you may instead create that many tokens that are copies of equipped creature.
// Equip {2}
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_0_applies(q: &dyn Query, asker: &Asker, proposal: &Proposal) -> bool { if let Proposal::TokenCreation(t) = proposal { let subject = Target::Player(t.controller); let data = [t.count, u32::MAX]; if t.count > 0 && true && Some(asker.controller) == Some(t.controller) && Some(asker.source).is_some_and(|candidate: ObjectId| q.state().obj(candidate).attached_to.is_some_and(|host| (q.state().battlefield().iter().copied().filter(|&id| q.chars(id).is(CardType::Creature)).collect::<Vec<ObjectId>>()).contains(&host))) { return true; } }
 false }
#[allow(unused_variables, unused_mut)] fn ability_0_0_apply(q: &dyn Query, asker: &Asker, proposal: &mut Proposal) { let seen = *proposal; if let Proposal::TokenCreation(seen) = &seen { let t = seen; let subject = Target::Player(t.controller); let data = [t.count, u32::MAX]; if let Proposal::TokenCreation(t) = proposal {  } } }
#[allow(unused_variables, unused_mut)] fn ability_0_0_then_steps(g: &mut dyn Game, asker: &Asker, subject: Target, data: [u32; 2]) -> Result<(), Suspend> { let decider_1 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") };
match planned_value!("depends on an instruction with a gap", 0u8 => cantrip_mechanics::choose::one_of(g, decider_1, asker.source, vec![Label::Card(card_1, 0), Label::Card(card_1, 1)])?) {
0 => {
planned!("event-change-in-the-program-of-another-event" => g.change_event(|q, proposal| { if let Proposal::TokenCreation(t) = proposal { if let Some(object) = Some(asker.source).and_then(|o: ObjectId| q.state().obj(o).attached_to) { let values = q.copiable_values(object); t.token = values.card; t.copy = Some(values); }  } }););
}
_ => {
planned!("event-change-in-the-program-of-another-event" => g.change_event(|q, proposal| { if let Proposal::TokenCreation(t) = proposal { t.count = t.count.saturating_mul(1); } }););
}
}
 Ok(()) }
#[allow(unused_variables)] fn ability_0_0_then(g: &mut dyn Game, asker: &Asker, before: &Proposal, _: &Proposal) { if let Proposal::TokenCreation(t) = before { let q: &dyn Query = g; let subject = Target::Player(t.controller); let data = [t.count, u32::MAX]; let _ = ability_0_0_then_steps(g, asker, subject, data); } }
#[allow(unused_variables)] fn ability_1_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if cantrip_mechanics::targets::permanent(q, id) && ((q.chars(id).is(CardType::Creature) && Some(asker.controller) == Some(q.controller(id))))) }
static ABILITY_1_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_1_target_0)];
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let attachments_4 = { let q: &dyn Query = g; (Some(asker.source).into_iter().collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let host_4 = { let q: &dyn Query = g; t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.object()) }; if let Some(host) = host_4 { cantrip_api::instruction(g, |g| { for attachment in attachments_4 { g.attach(attachment, host); } }); }
g.end_sentence();
 Ok(()) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }
planned_item!("replacement-event-first-each-turn" => replacements: replacement(ReplacementKind::TokenCreation, ability_0_0_applies, ability_0_0_apply, "As long as this Equipment is attached to a creature, the first time you would create one or more tokens each turn, you may instead create that many tokens that are copies of equipped creature.").then(ability_0_0_then).first_each_turn());

pub static CARD: CardDef = CardDef { name: "Mirrormind Crown", token_words: &[], oracle: true, cost: ManaCost { generic: 4, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Artifact]), subtypes: SubtypeSet::of(&[Subtype::Equipment]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 2, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })), ability_1, "Equip {2}").targets(ABILITY_1_TARGETS).equip().sorcery_speed()], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
