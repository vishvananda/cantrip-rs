// Wayward Swordtooth
// status: ready
// Ascend (If you control ten or more permanents, you get the city's blessing for the rest of the game.)
// You may play an additional land on each of your turns.
// This creature can't attack or block unless you have the city's blessing.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_2_0_affects(q: &dyn LayerQuery, source: ObjectId, affected: ObjectId, so_far: &Chars) -> bool { let asker = &Asker::new(source, q.controller(source)); (!(Some(asker.controller).is_some_and(|player: PlayerId| q.state().player(player).citys_blessing))) && (source == affected) }
#[allow(unused_variables)] fn ability_2_0_0_rule(q: &dyn Query, asker: &Asker, attacker: ObjectId, defender: Target) -> bool { let source = asker.source; let asker = &Asker::new(source, q.controller(source)); !((!(Some(asker.controller).is_some_and(|player: PlayerId| q.state().player(player).citys_blessing))) && attacker == asker.source) }
#[allow(unused_variables)] fn ability_2_0_1_keyword(q: &dyn LayerQuery, source: ObjectId, chars: &mut Chars) { let asker = &Asker::new(source, q.controller(source)); if !(Some(asker.controller).is_some_and(|player: PlayerId| q.state().player(player).citys_blessing)) { chars.keywords = chars.keywords.with(Keyword::CantBlock); } }

pub static CARD: CardDef = CardDef { name: "Wayward Swordtooth", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 0, 0, 0, 1, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Dinosaur]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(5), toughness: Some(5), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[static_ability(Layer::Ability, cantrip_mechanics::statics::itself, ability_2_0_1_keyword, "This creature can't attack or block unless you have the city's blessing.")], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[ability_2_0_0_rule], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, play_rules: &[PlayRule::ExtraLandPlays(1)], ascend: true, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
