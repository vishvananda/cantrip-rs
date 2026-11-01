// Graxiplon
// status: planned
// requires: static rule BlockedByDefender
// This creature can't be blocked unless defending player controls three or more creatures that share a creature type.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_0_rule(q: &dyn Query, asker: &Asker, id: ObjectId) -> bool { let source = asker.source; id == asker.source }
planned_item!("static-rule-blocked-by-defender" => statics: object_rule(ability_0_0_rule, Rule::BlockedByDefender { players: |q: &dyn Query, asker: &Asker, player: PlayerId| (q.state().apnap().filter(|&bound_0: &PlayerId| ({ let those: Vec<ObjectId> = q.state().battlefield().iter().copied().filter(|&id| (q.chars(id).is(CardType::Creature) && Some(bound_0) == Some(q.controller(id)))).collect::<Vec<ObjectId>>(); those.iter().flat_map(|&a| q.chars(a).subtypes.iter().filter(|s| s.card_types().has(CardType::Creature)).collect::<Vec<Subtype>>()).map(|s| those.iter().filter(|&&b| q.chars(b).has_subtype(s)).count()).max().unwrap_or(0) as i64 }) >= (3i64)).collect::<Vec<PlayerId>>()).contains(&player), unless: true }));

pub static CARD: CardDef = CardDef { name: "Graxiplon", token_words: &[], oracle: true, cost: ManaCost { generic: 5, x: 0, symbols: [0, 1, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Beast]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(3), toughness: Some(4), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
