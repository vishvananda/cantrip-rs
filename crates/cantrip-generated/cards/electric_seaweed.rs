// Electric Seaweed
// status: ready
// Defender, haste
// When this creature enters, until end of turn, whenever another creature dies, this creature deals 1 damage to each non-Wall creature.
// {T}: This creature deals 1 damage to any target.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_2_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ZoneChange { old, new, from, to, .. } = event { let id = *new; let subject = Target::Object(id); let data = [0u32, u32::MAX]; if to.is(ZoneKind::Battlefield) && Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
#[allow(unused_variables)] fn watch_2_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ZoneChange { old, new, from, to, .. } = event { let id = *old; let subject = Target::Object(id); let data = [0u32, u32::MAX]; if to.is(ZoneKind::Graveyard) && from.is(ZoneKind::Battlefield) && (!(Some(asker.source) == Some(id)) && q.chars(id).is(CardType::Creature)) { return Some(Fired { subject, data }); } }
 None }
fn watch_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let source_3 = { let q: &dyn Query = g; Some(asker.source) }; let amount_3 = { let q: &dyn Query = g; (1i64).max(0) as u16 }; let recipients_3 = { let q: &dyn Query = g; (q.state().battlefield().iter().copied().filter(|&id| (!(q.chars(id).has_subtype(q.words(asker.source).subtype(Subtype::Wall))) && q.chars(id).is(CardType::Creature))).collect::<Vec<ObjectId>>()).into_iter().map(Target::Object).collect::<Vec<Target>>() }; if let Some(source) = source_3 { let batch: Vec<(ObjectId, Target, u16)> = recipients_3.iter().map(|t| (source, *t, amount_3)).collect(); g.damage_together(&batch); }
g.end_sentence();
 Ok(()) }
fn ability_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
if let Some(source) = Some(item.object) { g.delayed_trigger(DelayedWhen::ThisTurn, source, item.controller, AbilityRef::delayed(item.ability.card, 0), Target::None); }
g.end_sentence();
 Ok(()) }
static ABILITY_3_TARGETS: &[cantrip_api::TargetSpec] = &[cantrip_mechanics::targets::ANY];
fn ability_3(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let source_4 = { let q: &dyn Query = g; Some(asker.source) }; let amount_4 = { let q: &dyn Query = g; (1i64).max(0) as u16 }; let recipients_4 = { let q: &dyn Query = g; t_0.clone() }; if let Some(source) = source_4 { let batch: Vec<(ObjectId, Target, u16)> = recipients_4.iter().map(|t| (source, *t, amount_4)).collect(); g.damage_together(&batch); }
g.end_sentence();
 Ok(()) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }

pub static CARD: CardDef = CardDef { name: "Electric Seaweed", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 0, 0, 2, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Plant,Subtype::Wall]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(0), toughness: Some(4), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Defender,Keyword::Haste]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ZONE_CHANGE.0), ability_2_event), ability_2, "When this creature enters, until end of turn, whenever another creature dies, this creature deals 1 damage to each non-Wall creature.").targets(&[]),ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(Cost { tap: true, ..Cost::NONE }), ability_3, "{T}: This creature deals 1 damage to any target.").targets(ABILITY_3_TARGETS)], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[triggered(when_asked(EventMask(EventMask::ZONE_CHANGE.0), watch_2_event), watch_2, "When this creature enters, until end of turn, whenever another creature dies, this creature deals 1 damage to each non-Wall creature.").targets(&[])], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
