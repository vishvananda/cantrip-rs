// Lavabrink Venturer
// status: planned
// requires: engine: no protection from this quality
// As this creature enters, choose odd or even. (Zero is even.)
// This creature has protection from each mana value of the chosen quality.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables, unused_mut)] fn ability_0_enters(g: &mut dyn Game, entering: &mut Entering) -> Result<(), Suspend> { let asker = &Asker::new(entering.object, entering.controller); cantrip_mechanics::choose::option_as_enters(g, entering, &CARD, 0..2)?; match entering.chosen_option.map(|chosen| chosen.index) {
Some(0) => {  }
Some(1) => {  }
_ => {} }
 Ok(()) }
#[allow(unused_variables)] fn ability_1_apply_0(q: &dyn LayerQuery, source: ObjectId, chars: &mut Chars) { let asker = &Asker::new(source, q.controller(source)); planned_value!("engine: no protection from this quality", () => chars.protection_from.push(ProtectionQuality::Matching(|q, asker, id| ((i64::from(q.mana_value(id, &q.chars(id)))) - (q.state().chosen_option(q.linked_to(asker.source)).filter(|c| std::ptr::eq(q.cards().get(c.card), &CARD)).map_or(-1i64, |c| i64::from(c.index == 0)))) % 2 == 0))); }

pub static CARD: CardDef = CardDef { name: "Lavabrink Venturer", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [1, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Human,Subtype::Soldier]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(3), toughness: Some(3), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[static_ability(Layer::Ability, cantrip_mechanics::statics::itself, ability_1_apply_0, "This creature has protection from each mana value of the chosen quality.")], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: Some(ability_0_enters), modal: None, enchant: None, labels: &["odd", "even"], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
