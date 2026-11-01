// Soulless Jailer
// status: planned
// requires: ability CantEnter
// requires: prohibited action CastSpell
// Permanent cards in graveyards can't enter the battlefield.
// Players can't cast noncreature spells from graveyards or exile.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_0_applies(q: &dyn Query, asker: &Asker, player: PlayerId) -> bool { true }
#[allow(unused_variables)] fn ability_1_1_applies(q: &dyn Query, asker: &Asker, player: PlayerId) -> bool { true }
planned_item!("ability-cant-enter" => cant_enter: CantEnter { cards: |q: &dyn Query, asker: &Asker, id: ObjectId| (q.chars(id).is(CardType::Artifact) || q.chars(id).is(CardType::Battle) || q.chars(id).is(CardType::Creature) || q.chars(id).is(CardType::Enchantment) || q.chars(id).is(CardType::Land) || q.chars(id).is(CardType::Planeswalker)), from: Some(Scope::Of(ZoneKind::Graveyard, |q: &dyn Query, asker: &Asker, player: PlayerId| true)) });
planned_item!("prohibited-action-cast-spell" => statics: player_rule(ability_1_0_applies, PlayerRule::Cant(Prohibition::CastSpell { from: Some(Scope::Of(ZoneKind::Graveyard, |q: &dyn Query, asker: &Asker, player: PlayerId| true)), spells: |q: &dyn Query, asker: &Asker, player: PlayerId, spell: ObjectId| !(q.chars(spell).is(CardType::Creature)) })));
planned_item!("prohibited-action-cast-spell" => statics: player_rule(ability_1_1_applies, PlayerRule::Cant(Prohibition::CastSpell { from: Some(Scope::Shared(ZoneKind::Exile)), spells: |q: &dyn Query, asker: &Asker, player: PlayerId, spell: ObjectId| !(q.chars(spell).is(CardType::Creature)) })));

pub static CARD: CardDef = CardDef { name: "Soulless Jailer", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Artifact,CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Phyrexian,Subtype::Golem]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(0), toughness: Some(4), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
