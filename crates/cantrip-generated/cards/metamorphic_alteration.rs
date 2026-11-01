// Metamorphic Alteration
// status: planned
// requires: earlier objects Selected
// requires: static change CopyOf
// Enchant creature
// As this Aura enters, choose a creature.
// Enchanted creature is a copy of the chosen creature.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_enchant(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if cantrip_mechanics::targets::permanent(q, id) && (q.chars(id).is(CardType::Creature))) }
#[allow(unused_variables, unused_mut)] fn ability_1_enters(g: &mut dyn Game, entering: &mut Entering) -> Result<(), Suspend> { let asker = &Asker::new(entering.object, entering.controller); let permanents = { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| q.chars(id).is(CardType::Creature)).collect::<Vec<ObjectId>>() }; let count = { let q: &dyn Query = g; cantrip_mechanics::select::Count::AsManyAsCan((1i64).max(0) as u32) }; cantrip_mechanics::choose::object_as_enters(g, entering, permanents, cantrip_mechanics::select::Selection::of(count, &[]))?;
 Ok(()) }
#[allow(unused_variables)] fn ability_2_copies(q: &dyn Query, asker: &Asker, id: ObjectId) -> bool { let source = asker.source; q.state().obj(asker.source).attached_to == Some(id) }
planned_item!("static-change-copy-of" => statics: copy_of(ability_2_copies, |q: &dyn Query, asker: &Asker| (planned_value!("earlier objects Selected", Vec::<ObjectId>::new() => q.acted_on(asker, Acted::Selected))).first().copied(), StaticCopyEffects::NoStaticCopyEffects));

pub static CARD: CardDef = CardDef { name: "Metamorphic Alteration", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [0, 1, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Enchantment]), subtypes: SubtypeSet::of(&[Subtype::Aura]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: Some(ability_1_enters), modal: None, enchant: Some(target(ability_0_enchant)), ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
