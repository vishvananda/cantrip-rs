// Haldan, Avid Arcanist
// status: planned
// requires: counter kind Fetch
// requires: object filter ExiledBy
// requires: play permission with mana as though any color
// Partner with Pako, Arcane Retriever (When this creature enters, target player may put Pako into their hand from their library, then shuffle.)
// You may play lands and cast noncreature spells from among cards you exiled that have fetch counters on them, and you may spend mana as though it were mana of any color to cast those spells.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_0_allows(q: &dyn Query, asker: &Asker, card: ObjectId, player: PlayerId) -> bool { player == asker.controller && (planned_value!("object filter ExiledBy", false => q.exiled_by(card, Some(asker.controller), None, false)) && (i64::from(q.state().obj(card).counters(planned_value!("counter kind Fetch", CounterKind::Charge => CounterKind::Fetch)))) > (0i64)) && q.chars(card).is(CardType::Land) }
planned_item!("play-permission-with-mana-as-though-any-color" => exile_casts: ExileCast { text: "Cast from exile", allows: |q, asker, card, player| { player == asker.controller && (planned_value!("object filter ExiledBy", false => q.exiled_by(card, Some(asker.controller), None, false)) && (i64::from(q.state().obj(card).counters(planned_value!("counter kind Fetch", CounterKind::Charge => CounterKind::Fetch)))) > (0i64)) && !(q.chars(card).is(CardType::Creature)) }, additional: Cost::NONE, any_type: false, lands: false, life_for_mana_value: false, any_color: true });

pub static CARD: CardDef = CardDef { name: "Haldan, Avid Arcanist", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 1, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Human,Subtype::Wizard]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(1), toughness: Some(4), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, exile_casts: &[ExileCast { text: "Play from exile", allows: ability_1_0_allows, additional: Cost::NONE, any_type: false, lands: true, life_for_mana_value: false }], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
