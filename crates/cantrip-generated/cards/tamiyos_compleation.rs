// Tamiyo's Compleation
// status: ready
// Flash
// Enchant artifact, creature, or planeswalker
// When this Aura enters, tap enchanted permanent. If it's an Equipment, unattach it.
// Enchanted permanent loses all abilities and doesn't untap during its controller's untap step.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_enchant(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if cantrip_mechanics::targets::permanent(q, id) && ((q.chars(id).is(CardType::Artifact) || q.chars(id).is(CardType::Creature) || q.chars(id).is(CardType::Planeswalker)))) }
#[allow(unused_variables)] fn ability_2_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ZoneChange { old, new, from, to, .. } = event { let id = *new; let subject = Target::Object(id); let data = [0u32, u32::MAX]; if to.is(ZoneKind::Battlefield) && Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
fn ability_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let objects_0 = { let q: &dyn Query = g; (Some(asker.source).and_then(|o: ObjectId| q.state().obj(o).attached_to).into_iter().collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let turned_0: Vec<ObjectId> = objects_0.iter().copied().filter(|&o| g.state().obj(o).tapped != true).collect(); g.tap_together(&turned_0);
g.end_sentence();
if { let q: &dyn Query = g; Some(asker.source).and_then(|o: ObjectId| q.state().obj(o).attached_to).is_some_and(|candidate: ObjectId| q.chars(candidate).has_subtype(q.words(asker.source).subtype(Subtype::Equipment))) } {
if let Some(object) = ({ let q: &dyn Query = g; (Some(asker.source).and_then(|o: ObjectId| q.state().obj(o).attached_to).into_iter().collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }).first().copied() { g.unattach(object); }
g.end_sentence();
}
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_4_0_rule(q: &dyn Query, asker: &Asker, id: ObjectId) -> bool { q.state().obj(asker.source).attached_to == Some(id) }

pub static CARD: CardDef = CardDef { name: "Tamiyo's Compleation", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [0, 1, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Enchantment]), subtypes: SubtypeSet::of(&[Subtype::Aura]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Flash]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ZONE_CHANGE.0), ability_2_event), ability_2, "When this Aura enters, tap enchanted permanent. If it's an Equipment, unattach it.").targets(&[])], statics: &[loses_all_abilities(cantrip_mechanics::attach::attached_object, "Enchanted permanent loses all abilities and doesn't untap during its controller's untap step.")], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[ability_4_0_rule], delayed: &[], enters: None, modal: None, enchant: Some(target(ability_1_enchant)), ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
