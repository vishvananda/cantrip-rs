// Phyrexian Censor
// status: ready
// Each player can't cast more than one non-Phyrexian spell each turn.
// Non-Phyrexian creatures enter tapped.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_0_rule(q: &dyn Query, asker: &Asker, player: PlayerId, spell: ObjectId) -> bool { !(true && !(q.chars(spell).has_subtype(q.words(asker.source).subtype(Subtype::Phyrexian))) && q.history().casts_by(player).filter(|c| c.spell != spell && !(c.chars(q).has_subtype(q.words(asker.source).subtype(Subtype::Phyrexian)))).count() >= 1) }
#[allow(unused_variables)] fn ability_1_applies(q: &dyn Query, asker: &Asker, proposal: &Proposal) -> bool { if let Proposal::ZoneChange(z) = proposal { let id = z.object; let subject = Target::Object(id); let data = [u32::MAX; 2]; if z.to.is(ZoneKind::Battlefield) && (!(q.chars(id).has_subtype(q.words(asker.source).subtype(Subtype::Phyrexian))) && q.chars(id).is(CardType::Creature)) { return true; } }
 false }
#[allow(unused_variables, unused_mut)] fn ability_1_apply(q: &dyn Query, asker: &Asker, proposal: &mut Proposal) { let seen = *proposal; if let Proposal::ZoneChange(seen) = &seen { let z = seen; let id = z.object; let subject = Target::Object(id); let data = [u32::MAX; 2]; if let Proposal::ZoneChange(z) = proposal { z.tapped = true; } } }

pub static CARD: CardDef = CardDef { name: "Phyrexian Censor", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [1, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Phyrexian,Subtype::Wizard]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(3), toughness: Some(3), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[replacement(ReplacementKind::ZoneChange, ability_1_applies, ability_1_apply, "Non-Phyrexian creatures enter tapped.")], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, spell_rules: &[ability_0_0_rule], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
