// Ozai, the Phoenix King
// status: planned
// requires: player rule UnspentManaBecomes
// Trample, firebending 4, haste
// If you would lose unspent mana, that mana becomes red instead.
// Ozai has flying and indestructible as long as you have six or more unspent mana.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_3_0_applies(q: &dyn Query, asker: &Asker, player: PlayerId) -> bool { player == asker.controller }
#[allow(unused_variables)] fn ability_4_0_affects(q: &dyn LayerQuery, source: ObjectId, affected: ObjectId, so_far: &Chars) -> bool { let asker = &Asker::new(source, q.controller(source)); ((Some(asker.controller).map_or(0i64, |p: PlayerId| i64::from(q.state().player(p).pool.total() + q.state().player(p).lasting.total()))) >= (6i64)) && (source == affected) }
#[allow(unused_variables)] fn ability_4_0_apply_0(q: &dyn LayerQuery, source: ObjectId, chars: &mut Chars) { let asker = &Asker::new(source, q.controller(source)); chars.keywords = chars.keywords.with(Keyword::Flying); }
#[allow(unused_variables)] fn ability_4_0_apply_1(q: &dyn LayerQuery, source: ObjectId, chars: &mut Chars) { let asker = &Asker::new(source, q.controller(source)); chars.keywords = chars.keywords.with(Keyword::Indestructible); }
planned_item!("player-rule-unspent-mana-becomes" => statics: player_rule(ability_3_0_applies, PlayerRule::UnspentManaBecomes(Some(Mana::Red))));

pub static CARD: CardDef = CardDef { name: "Ozai, the Phoenix King", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 0, 2, 2, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Human,Subtype::Noble]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(7), toughness: Some(7), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Trample,Keyword::Haste]), protection: ColorSet(0), spell: None, abilities: &[cantrip_mechanics::firebending::trigger::<4>("Firebending")], statics: &[static_ability(Layer::Ability, ability_4_0_affects, ability_4_0_apply_0, "Ozai has flying and indestructible as long as you have six or more unspent mana."),static_ability(Layer::Ability, ability_4_0_affects, ability_4_0_apply_1, "Ozai has flying and indestructible as long as you have six or more unspent mana.")], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
