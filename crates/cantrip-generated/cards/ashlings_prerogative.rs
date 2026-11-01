// Ashling's Prerogative
// status: ready
// As this enchantment enters, choose odd or even. (Zero is even.)
// Each creature with mana value of the chosen quality has haste.
// Each creature without mana value of the chosen quality enters tapped.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables, unused_mut)] fn ability_0_enters(g: &mut dyn Game, entering: &mut Entering) -> Result<(), Suspend> { let asker = &Asker::new(entering.object, entering.controller); cantrip_mechanics::choose::option_as_enters(g, entering, &CARD, 0..2)?; match entering.chosen_option.map(|chosen| chosen.index) {
Some(0) => {  }
Some(1) => {  }
_ => {} }
 Ok(()) }
#[allow(unused_variables)] fn ability_1_affects(q: &dyn LayerQuery, source: ObjectId, affected: ObjectId, so_far: &Chars) -> bool { let asker = &Asker::new(source, q.controller(source)); ((*so_far).is(CardType::Creature) && ((i64::from(q.mana_value(affected, &(*so_far)))) - (q.state().chosen_option(q.linked_to(asker.source)).filter(|c| std::ptr::eq(q.cards().get(c.card), &CARD)).map_or(-1i64, |c| i64::from(c.index == 0)))) % 2 == 0) }
#[allow(unused_variables)] fn ability_1_apply_0(q: &dyn LayerQuery, source: ObjectId, chars: &mut Chars) { let asker = &Asker::new(source, q.controller(source)); chars.keywords = chars.keywords.with(Keyword::Haste); }
#[allow(unused_variables)] fn ability_2_applies(q: &dyn Query, asker: &Asker, proposal: &Proposal) -> bool { if let Proposal::ZoneChange(z) = proposal { let id = z.object; let subject = Target::Object(id); let data = [u32::MAX; 2]; if z.to.is(ZoneKind::Battlefield) && (q.chars(id).is(CardType::Creature) && ((i64::from(q.mana_value(id))) - (cantrip_mechanics::choose::chosen_option(q, asker, &CARD).map_or(-1i64, |index| i64::from(index == 0)))) % 2 != 0) { return true; } }
 false }
#[allow(unused_variables, unused_mut)] fn ability_2_apply(q: &dyn Query, asker: &Asker, proposal: &mut Proposal) { let seen = *proposal; if let Proposal::ZoneChange(seen) = &seen { let z = seen; let id = z.object; let subject = Target::Object(id); let data = [u32::MAX; 2]; if let Proposal::ZoneChange(z) = proposal { z.tapped = true; } } }

pub static CARD: CardDef = CardDef { name: "Ashling's Prerogative", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [0, 0, 0, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Enchantment]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[static_ability(Layer::Ability, ability_1_affects, ability_1_apply_0, "Each creature with mana value of the chosen quality has haste.")], replacements: &[replacement(ReplacementKind::ZoneChange, ability_2_applies, ability_2_apply, "Each creature without mana value of the chosen quality enters tapped.")], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: Some(ability_0_enters), modal: None, enchant: None, labels: &["odd", "even"], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
