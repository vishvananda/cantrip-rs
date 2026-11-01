// Empyrial Archangel
// status: ready
// Flying
// Shroud (This creature can't be the target of spells or abilities.)
// All damage that would be dealt to you is dealt to this creature instead.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_2_applies(q: &dyn Query, asker: &Asker, proposal: &Proposal) -> bool { if let Proposal::Damage(e) = proposal { let (source, target, amount, combat) = (e.source, &e.target, e.amount, e.combat); let subject = *target; let data = [u32::from(amount), source.0]; if true && true && matches!(*target, Target::Player(p) if Some(asker.controller) == Some(p)) { return true; } }
 false }
#[allow(unused_variables, unused_mut)] fn ability_2_apply(q: &dyn Query, asker: &Asker, proposal: &mut Proposal) { let seen = *proposal; if let Proposal::Damage(seen) = &seen { let e = seen; let (source, target, amount, combat) = (e.source, &e.target, e.amount, e.combat); let subject = *target; let data = [u32::from(amount), source.0]; if let Proposal::Damage(e) = proposal { if let Some(recipient) = ((Some(asker.source).into_iter().collect::<Vec<ObjectId>>()).into_iter().map(Target::Object).collect::<Vec<Target>>()).first().copied() { e.target = recipient; } } } }

pub static CARD: CardDef = CardDef { name: "Empyrial Archangel", token_words: &[], oracle: true, cost: ManaCost { generic: 4, x: 0, symbols: [2, 1, 0, 0, 1, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Angel]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(5), toughness: Some(8), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Flying,Keyword::Shroud]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[replacement(ReplacementKind::Damage, ability_2_applies, ability_2_apply, "All damage that would be dealt to you is dealt to this creature instead.")], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
