// Kruphix, God of Horizons
// status: planned
// requires: numeric expression CountManaSymbols
// requires: player rule UnspentManaBecomes
// Indestructible
// As long as your devotion to green and blue is less than seven, Kruphix isn't a creature.
// You have no maximum hand size.
// If you would lose unspent mana, that mana becomes colorless instead.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_0_affects(q: &dyn LayerQuery, source: ObjectId, affected: ObjectId, so_far: &Chars) -> bool { let asker = &Asker::new(source, q.controller(source)); ((planned_value!("numeric expression CountManaSymbols", 0i64 => i64::from(cantrip_mechanics::devotion::symbols_among(q, &objects, 2)))) < (7i64)) && (source == affected) }
#[allow(unused_variables)] fn ability_1_0_apply_0(q: &dyn LayerQuery, source: ObjectId, chars: &mut Chars) { let asker = &Asker::new(source, q.controller(source)); chars.types = TypeSet(chars.types.0 & !(1 << CardType::Creature as u8)); }
#[allow(unused_variables)] fn ability_3_0_applies(q: &dyn Query, asker: &Asker, player: PlayerId) -> bool { player == asker.controller }
planned_item!("player-rule-unspent-mana-becomes" => statics: player_rule(ability_3_0_applies, PlayerRule::UnspentManaBecomes(None)));

pub static CARD: CardDef = CardDef { name: "Kruphix, God of Horizons", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [0, 1, 0, 0, 1, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Enchantment,CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::God]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(4), toughness: Some(7), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Indestructible]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[static_ability(Layer::Type, ability_1_0_affects, ability_1_0_apply_0, "As long as your devotion to green and blue is less than seven, Kruphix isn't a creature.")], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, play_rules: &[PlayRule::MaximumHandSize(None)], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
