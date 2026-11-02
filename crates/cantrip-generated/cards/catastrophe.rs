// Catastrophe
// status: planned
// requires: destroyed together, some that can't be regenerated
// Destroy all lands or all creatures. Creatures destroyed this way can't be regenerated.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let player_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let chosen_0 = cantrip_mechanics::choose::option(g, player_0, asker.source, item.ability.card, 0..2)? - 0;
g.end_sentence();
planned!("destroyed-together-some-that-can-t-be-regenerated", Together, "value" = "[{\"args\":{\"objects\":{\"args\":{\"filter\":{\"args\":\"Creature\",\"kind\":\"CardType\"},\"objects\":{\"args\":{\"kind\":\"ChosenFilter\"},\"kind\":\"Battlefield\"}},\"kind\":\"Matching\"},\"regeneration\":false},\"kind\":\"Destroy\"},{\"args\":{\"objects\":{\"args\":{\"excluding\":{\"args\":{\"args\":\"Creature\",\"kind\":\"CardType\"},\"kind\":\"Battlefield\"},\"objects\":{\"args\":{\"kind\":\"ChosenFilter\"},\"kind\":\"Battlefield\"}},\"kind\":\"Difference\"},\"regeneration\":true},\"kind\":\"Destroy\"}]");
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Catastrophe", token_words: &[], oracle: true, cost: ManaCost { generic: 4, x: 0, symbols: [2, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Sorcery]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, labels: &["land", "creature"], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
