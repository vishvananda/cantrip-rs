// Ghired's Belligerence
// status: ready
// Ghired's Belligerence deals X damage divided as you choose among any number of target creatures. Whenever a creature dealt damage this way dies this turn, populate. (Create a token that's a copy of a creature token you control.)
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if cantrip_mechanics::targets::permanent(q, id) && (q.chars(id).is(CardType::Creature))) }
static ABILITY_0_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_0_target_0).between(1, TargetSpec::COUNTED)];
#[allow(unused_variables)] fn watch_3_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ZoneChange { old, new, from, to, .. } = event { let id = *old; let subject = Target::Object(id); let data = [0u32, u32::MAX]; if to.is(ZoneKind::Graveyard) && from.is(ZoneKind::Battlefield) && (q.chars(id).is(CardType::Creature) && cantrip_mechanics::history::permanents_dealt_damage_by(q.history(), asker.source, 0).contains(&id)) { return Some(Fired { subject, data }); } }
 None }
fn watch_3(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let mut populated_4: Vec<ObjectId> = Vec::new();
{ let player = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let tokens: Vec<ObjectId> = g.permanents_of(player).into_iter().filter(|&o| g.obj(o).token && g.chars(o).is(CardType::Creature)).collect(); if let Some(token) = cantrip_mechanics::choose::card(g, player, asker.source, Purpose::CopyWhich, tokens)? { let values = g.copiable_values(token); populated_4.extend(g.create_token_copies_tapped(values, player, 1, false)?); }; }
g.end_sentence();
 Ok(()) }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
cantrip_mechanics::damage_divided(g, item);
g.end_sentence();
if let Some(source) = Some(item.object) { g.delayed_trigger(DelayedWhen::ThisTurn, source, item.controller, AbilityRef::delayed(item.ability.card, 0), Target::None); }
g.end_sentence();
 Ok(()) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }

pub static CARD: CardDef = CardDef { name: "Ghired's Belligerence", token_words: &[], oracle: true, cost: ManaCost { generic: 0, x: 1, symbols: [0, 0, 0, 2, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Sorcery]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(ABILITY_0_TARGETS, ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[triggered(when_asked(EventMask(EventMask::ZONE_CHANGE.0), watch_3_event), watch_3, "Whenever a creature dealt damage this way dies this turn, populate.").targets(&[]).divide(Divide { total: TargetSpec::COUNTED, of_x: None })], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
