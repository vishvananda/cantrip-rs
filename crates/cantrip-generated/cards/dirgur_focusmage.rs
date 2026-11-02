// Dirgur Focusmage // Braingeyser
// status: ready
// Instant and sorcery spells you cast cost {1} less to cast.
// Whenever you cast an instant or sorcery spell with mana value 5 or greater from your hand, this creature becomes prepared.
// Target player draws X cards.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_0_rule(q: &dyn Query, source: ObjectId, player: PlayerId, costed: Costed, targets: &[Target]) -> CostDelta { let asker = &Asker::new(source, q.controller(source)); let Costed::Spell(spell) = costed else { return CostDelta::NONE; }; if player == asker.controller && (q.chars(spell).is(CardType::Instant) || q.chars(spell).is(CardType::Sorcery)) { CostDelta::less((1i64) as u8) } else { CostDelta::NONE } }
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::SpellCast { spell, player, .. } = event { let id = *spell; let subject = Target::Object(id); let data = [u32::from(player.0), u32::MAX]; if Some(asker.controller) == Some(*player) && (((q.chars(id).is(CardType::Instant) || q.chars(id).is(CardType::Sorcery)) && (i64::from(q.mana_value(id))) >= (5i64)) && cast_at(q, event, id).is_some_and(|cast| cast.from == ZoneKind::Hand && q.state().obj(cast.spell).owner == cast.player)) { return Some(Fired { subject, data }); } }
 None }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
if let Some(object) = ({ let q: &dyn Query = g; (Some(asker.source).into_iter().collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }).first().copied() { g.become_prepared(object); }
g.end_sentence();
 Ok(()) }
fn cast_of<'a>(q: &'a dyn Query, id: ObjectId) -> Option<&'a Cast> { let records = q.history().all(); let spell = records.iter().rev().find_map(|r| match r.event { Event::ZoneChange { old, new, reason: ZoneReason::Resolve, .. } if new == id => Some(old), _ => None }).unwrap_or(id); records.iter().rev().find_map(|r| match &r.event { Event::Cast(cast) if cast.spell == spell => Some(cast), _ => None }) }
#[allow(dead_code)] fn cast_at<'a>(q: &'a dyn Query, event: &Event, id: ObjectId) -> Option<&'a Cast> { match event { Event::ZoneChange { old, new, reason: ZoneReason::Resolve, .. } if *new == id => cast_of(q, *old), _ => cast_of(q, id) } }

pub static CARD: CardDef = CardDef { name: "Dirgur Focusmage", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 1, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Djinn,Subtype::Monk]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(1), toughness: Some(4), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::SPELL_CAST.0), ability_1_event), ability_1, "Whenever you cast an instant or sorcery spell with mana value 5 or greater from your hand, this creature becomes prepared.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, cost_changes: &[ability_0_0_rule], prepare: Some(&prepared::CARD), ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[&prepared::CARD];
pub mod prepared {
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Player(p) if true) }
static ABILITY_0_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_0_target_0)];
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let amount_0 = { let q: &dyn Query = g; (i64::from(item.x)) as u32 }; let players_0 = { let q: &dyn Query = g; t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.player()).into_iter().collect::<Vec<PlayerId>>() };
for player in players_0 { g.draw(player, amount_0); }
g.end_sentence();
 Ok(()) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }

pub static CARD: CardDef = CardDef { name: "Braingeyser", token_words: &[], oracle: false, cost: ManaCost { generic: 0, x: 1, symbols: [0, 2, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Sorcery]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(ABILITY_0_TARGETS, ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
}
