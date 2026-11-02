// Space Beleren
// status: planned
// requires: engine: no SpaceSculptor
// requires: engine: no resolving rule on an open set: Prohibit BeBlocked beyond lasting rules
// requires: keyword action ChooseSector
// requires: object filter InSector
// Space sculptor (Space Beleren divides the battlefield into alpha, beta, and gamma sectors. If a creature isn't assigned to a sector, its controller assigns it to one. Opponents assign first.)
// [+1]: Creatures in each sector can be blocked this turn only by creatures in the same sector.
// [−1]: Put a +1/+1 counter on each creature in the sector of your choice.
// [−5]: Destroy all creatures in the sector of your choice.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn block_rule_0(q: &dyn Query, source: ObjectId, blocker: ObjectId, attacker: ObjectId) -> bool { let asker = &Asker::new(source, q.controller(source)); !(attacker == source && !(planned_value!("object filter InSector", false => q.in_sector(asker, blocker, Some(attacker))))) }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
planned!("engine-no-resolving-rule-on-an-open-set-prohibit-be-blocked-beyond-lasting-rules" => if let Some(source) = { let q: &dyn Query = g; Some(asker.source) } { g.add_open_effect(source, move |q: &dyn Query, asker: &Asker, id: ObjectId| q.chars(id).is(CardType::Creature), EffectKind::BlockRestriction { card: { let o = q.state().obj(asker.source); o.copied_card().unwrap_or(o.card) }, index: 0 }, Duration::EndOfTurn); });
g.end_sentence();
 Ok(()) }
fn ability_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
planned!("keyword-action-choose-sector" => { let player = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; cantrip_mechanics::sectors::choose_sector(g, player, asker.source)?; });
g.end_sentence();
let mut counters_2: Vec<(ObjectId, CounterKind, i16)> = Vec::new();
let amount = { let q: &dyn Query = g; (1i64).max(0) as i16 }; for t in { let q: &dyn Query = g; (q.state().battlefield().iter().copied().filter(|&id| (q.chars(id).is(CardType::Creature) && planned_value!("object filter InSector", false => q.in_sector(asker, id, None)))).collect::<Vec<ObjectId>>()).into_iter().map(Target::Object).collect::<Vec<Target>>() } { if let Target::Object(object) = t { counters_2.push((object, CounterKind::PlusOne, amount)); } }
let had_2: Vec<_> = counters_2.iter().map(|c| g.state().obj(c.0).counters(c.1)).collect(); g.add_counters_together(&counters_2); let applied_2: Vec<ObjectId> = counters_2.iter().zip(had_2).filter(|(c, had)| g.state().obj(c.0).counters(c.1) > *had).map(|(c, _)| c.0).collect();
g.end_sentence();
 Ok(()) }
fn ability_3(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
planned!("keyword-action-choose-sector" => { let player = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; cantrip_mechanics::sectors::choose_sector(g, player, asker.source)?; });
g.end_sentence();
let objects_4 = { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| (q.chars(id).is(CardType::Creature) && planned_value!("object filter InSector", false => q.in_sector(asker, id, None)))).collect::<Vec<ObjectId>>() }; let destroyed_4 = g.destroy_all(&objects_4);
g.end_sentence();
 Ok(()) }
planned_item!("engine-no-space-sculptor" => abilities: cantrip_mechanics::space_sculptor::ABILITY);

pub static CARD: CardDef = CardDef { name: "Space Beleren", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [1, 1, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Planeswalker]), subtypes: SubtypeSet::of(&[Subtype::Jace]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: None, toughness: None, loyalty: Some(3), defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(loyalty(1)), ability_1, "+1: Creatures in each sector can be blocked this turn only by creatures in the same sector.").targets(&[]).sorcery_speed(),ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(loyalty(-1)), ability_2, "−1: Put a +1/+1 counter on each creature in the sector of your choice.").targets(&[]).sorcery_speed(),ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(loyalty(-5)), ability_3, "−5: Destroy all creatures in the sector of your choice.").targets(&[]).sorcery_speed()], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, lasting_block_rules: &[LastingBlockRule { text: "BeBlocked", allows: block_rule_0 }], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
