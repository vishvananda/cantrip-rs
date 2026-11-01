// Streaking Oilgorger
// status: ready
// Flying, haste
// Start your engines! (If you have no speed, it starts at 1. It increases once on each of your turns when an opponent loses life. Max speed is 4.)
// Max speed — This creature has lifelink.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_3_0_affects(q: &dyn LayerQuery, source: ObjectId, affected: ObjectId, so_far: &Chars) -> bool { let asker = &Asker::new(source, q.controller(source)); (Some(asker.controller).is_some_and(|player: PlayerId| q.state().player(player).speed == Some(4))) && (source == affected) }
#[allow(unused_variables)] fn ability_3_0_apply_0(q: &dyn LayerQuery, source: ObjectId, chars: &mut Chars) { let asker = &Asker::new(source, q.controller(source)); chars.keywords = chars.keywords.with(Keyword::Lifelink); }

pub static CARD: CardDef = CardDef { name: "Streaking Oilgorger", token_words: &[], oracle: true, cost: ManaCost { generic: 4, x: 0, symbols: [0, 0, 1, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Vampire]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(3), toughness: Some(3), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Flying,Keyword::Haste]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[static_ability(Layer::Ability, ability_3_0_affects, ability_3_0_apply_0, "Max speed — This creature has lifelink.")], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, start_your_engines: true, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
