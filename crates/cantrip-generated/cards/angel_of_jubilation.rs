// Angel of Jubilation
// status: planned
// requires: prohibited action PayLife
// requires: prohibited action Sacrifice
// Flying
// Other nonblack creatures you control get +1/+1.
// Players can't pay life or sacrifice creatures to cast spells or activate abilities.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_affects(q: &dyn LayerQuery, source: ObjectId, affected: ObjectId, so_far: &Chars) -> bool { let asker = &Asker::new(source, q.controller(source)); (!(Some(asker.source) == Some(affected)) && (!((*so_far).colors.has(q.words(asker.source).color(Color::Black))) && (*so_far).is(CardType::Creature) && Some(asker.controller) == Some(q.controller(affected)))) }
#[allow(unused_variables)] fn ability_1_apply_0(q: &dyn LayerQuery, source: ObjectId, chars: &mut Chars) { let asker = &Asker::new(source, q.controller(source)); let (p, t) = ((1i64) as i16, (1i64) as i16); chars.power = chars.power.map(|v| v.saturating_add(p)); chars.toughness = chars.toughness.map(|v| v.saturating_add(t)); }
#[allow(unused_variables)] fn ability_2_0_applies(q: &dyn Query, asker: &Asker, player: PlayerId) -> bool { true }
#[allow(unused_variables)] fn ability_2_1_applies(q: &dyn Query, asker: &Asker, player: PlayerId) -> bool { true }
#[allow(unused_variables)] fn ability_2_2_applies(q: &dyn Query, asker: &Asker, player: PlayerId) -> bool { true }
#[allow(unused_variables)] fn ability_2_3_applies(q: &dyn Query, asker: &Asker, player: PlayerId) -> bool { true }
planned_item!("prohibited-action-pay-life" => statics: player_rule(ability_2_0_applies, PlayerRule::Cant(Prohibition::PayLife(Payment::Spells(|q: &dyn Query, asker: &Asker, player: PlayerId, spell: ObjectId| true)))));
planned_item!("prohibited-action-pay-life" => statics: player_rule(ability_2_1_applies, PlayerRule::Cant(Prohibition::PayLife(Payment::Abilities(|q: &dyn Query, asker: &Asker, permanent: ObjectId, def: Option<&ActivatedDef>| true)))));
planned_item!("prohibited-action-sacrifice" => statics: player_rule(ability_2_2_applies, PlayerRule::Cant(Prohibition::Sacrifice { objects: |q: &dyn Query, asker: &Asker, id: ObjectId| q.chars(id).is(CardType::Creature), paying: Some(Payment::Spells(|q: &dyn Query, asker: &Asker, player: PlayerId, spell: ObjectId| true)) })));
planned_item!("prohibited-action-sacrifice" => statics: player_rule(ability_2_3_applies, PlayerRule::Cant(Prohibition::Sacrifice { objects: |q: &dyn Query, asker: &Asker, id: ObjectId| q.chars(id).is(CardType::Creature), paying: Some(Payment::Abilities(|q: &dyn Query, asker: &Asker, permanent: ObjectId, def: Option<&ActivatedDef>| true)) })));

pub static CARD: CardDef = CardDef { name: "Angel of Jubilation", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [3, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Angel]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(3), toughness: Some(3), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Flying]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[static_ability(Layer::PtModify, ability_1_affects, ability_1_apply_0, "Other nonblack creatures you control get +1/+1.")], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
