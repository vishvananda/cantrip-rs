// Chaplain of Alms // Chapel Shieldgeist
// status: planned
// requires: engine: no Disturb
// Flying, first strike
// Each creature you control has ward {1}. (Whenever it becomes the target of a spell or ability an opponent controls, counter it unless that player pays {1}.)
// If Chapel Shieldgeist would be put into a graveyard from anywhere, exile it instead.
// First strike
// Ward {1} (Whenever this creature becomes the target of a spell or ability an opponent controls, counter it unless that player pays {1}.)
// Disturb {3}{W} (You may cast this card from your graveyard transformed for its disturb cost.)
#[allow(unused_imports)] use cantrip_api::*;
planned_item!("engine-no-disturb" => ways: cantrip_mechanics::disturb::way(ManaCost { generic: 3, x: 0, symbols: [1, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, "Disturb {3}{W}"));

pub static CARD: CardDef = CardDef { name: "Chaplain of Alms", token_words: &[], oracle: true, cost: ManaCost { generic: 0, x: 0, symbols: [1, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Human,Subtype::Cleric]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(1), toughness: Some(1), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::FirstStrike]), protection: ColorSet(0), spell: None, abilities: &[cantrip_mechanics::triggers::ward::<1>("Ward {1}")], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, back: Some(&back::CARD), ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
pub mod back {
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_2_affects(q: &dyn LayerQuery, source: ObjectId, affected: ObjectId, so_far: &Chars) -> bool { let asker = &Asker::new(source, q.controller(source)); ((*so_far).is(CardType::Creature) && Some(asker.controller) == Some(q.controller(affected))) }
#[allow(unused_variables)] fn ability_2_apply_0(q: &dyn LayerQuery, source: ObjectId, chars: &mut Chars) { let asker = &Asker::new(source, q.controller(source)); let card = q.cards().current(q.state().obj(asker.source)); let _ = card; if let Some(slot) = chars.abilities.iter().position(|a| a.is_none()) { chars.abilities[slot] = Some(AbilityRef::delayed(q.cards().current(q.state().obj(asker.source)), 0)); } }
#[allow(unused_variables)] fn ability_3_applies(q: &dyn Query, asker: &Asker, proposal: &Proposal) -> bool { if let Proposal::ZoneChange(z) = proposal { let id = z.object; let subject = Target::Object(id); let data = [u32::MAX; 2]; if z.to.is(ZoneKind::Graveyard) && Some(asker.source) == Some(id) { return true; } }
 false }
#[allow(unused_variables, unused_mut)] fn ability_3_apply(q: &dyn Query, asker: &Asker, proposal: &mut Proposal) { let seen = *proposal; if let Proposal::ZoneChange(seen) = &seen { let z = seen; let id = z.object; let subject = Target::Object(id); let data = [u32::MAX; 2]; if let Proposal::ZoneChange(z) = proposal { z.to = Zone::exile(q.state().obj(z.object).owner); } } }

pub static CARD: CardDef = CardDef { name: "Chapel Shieldgeist", token_words: &[], oracle: true, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Spirit,Subtype::Cleric]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: Some(ColorSet(1)), power: Some(2), toughness: Some(1), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Flying,Keyword::FirstStrike]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[static_ability(Layer::Ability, ability_2_affects, ability_2_apply_0, "Each creature you control has ward {1}.")], replacements: &[replacement(ReplacementKind::ZoneChange, ability_3_applies, ability_3_apply, "If Chapel Shieldgeist would be put into a graveyard from anywhere, exile it instead.").own_everywhere()], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[cantrip_mechanics::triggers::ward::<1>("Ward {1}")], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
}
