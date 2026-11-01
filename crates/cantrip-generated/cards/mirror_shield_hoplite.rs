// Mirror-Shield Hoplite
// status: planned
// requires: targeted by BackupAbility
// Vigilance
// Whenever a creature you control becomes the target of a backup ability, copy that ability. You may choose new targets for the copy. This ability triggers only once each turn.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::BecameTarget { target: Target::Object(target), by, controller, .. } = event { let id = *target; let by = *by; let controller = *controller; let subject = Target::Object(id); let data = [by.0, u32::from(controller.0) | (q.state().stack.len().saturating_sub(1) as u32) << 8]; if (q.chars(id).is(CardType::Creature) && Some(asker.controller) == Some(q.controller(id))) && q.is_on_battlefield(id) && !q.state().stack.iter().any(|i| i.kind == StackKind::Spell && i.object == by) && planned_value!("targeted by BackupAbility", false => cantrip_mechanics::targets::targeted_by(q, by, "BackupAbility", &[])) { return Some(Fired { subject, data }); } }
 None }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let copier_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let t = { let q: &dyn Query = g; Target::Ability(q.state().stack.get((item.data[1] >> 8) as usize).map_or(u32::MAX, |e| e.serial)) }; g.copy_ability(t, copier_0, true)?;
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Mirror-Shield Hoplite", token_words: &[], oracle: true, cost: ManaCost { generic: 0, x: 0, symbols: [1, 0, 0, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Human,Subtype::Soldier]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(2), toughness: Some(2), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Vigilance]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::BECAME_TARGET.0), ability_1_event), ability_1, "Whenever a creature you control becomes the target of a backup ability, copy that ability. You may choose new targets for the copy. This ability triggers only once each turn.").targets(&[]).triggers_each_turn(1)], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
