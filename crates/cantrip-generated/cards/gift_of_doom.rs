// Gift of Doom
// status: planned
// requires: ability AsEvent
// Enchant creature
// Enchanted creature has deathtouch and indestructible.
// Morph—Sacrifice another creature. (You may cast this card face down as a 2/2 creature for {3}. Turn it face up any time for its morph cost.)
// As this Aura is turned face up, you may attach it to a creature.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_enchant(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if cantrip_mechanics::targets::permanent(q, id) && (q.chars(id).is(CardType::Creature))) }
#[allow(unused_variables)] fn ability_1_apply_0(q: &dyn LayerQuery, source: ObjectId, chars: &mut Chars) { let asker = &Asker::new(source, q.controller(source)); chars.keywords = chars.keywords.with(Keyword::Deathtouch); }
#[allow(unused_variables)] fn ability_1_apply_1(q: &dyn LayerQuery, source: ObjectId, chars: &mut Chars) { let asker = &Asker::new(source, q.controller(source)); chars.keywords = chars.keywords.with(Keyword::Indestructible); }
#[allow(unused_variables)] fn ability_2_0(q: &dyn Query, asker: &Asker, id: ObjectId) -> bool { (!(Some(asker.source) == Some(id)) && q.chars(id).is(CardType::Creature)) }
planned_item!("ability-as-event" => as_events: AsEventDef { event: AsEvent::TurnsFaceUp, object: |g: &mut dyn Game, asker: &Asker| { let q: &dyn Query = g; Some(asker.source) }, actions: &[FaceUpAction::MayActions(FaceUpAction::AttachPermanentToAPermanent({ let q: &dyn Query = g; Some(asker.source) }, { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| q.chars(id).is(CardType::Creature)).collect::<Vec<ObjectId>>() }))] });

pub static CARD: CardDef = CardDef { name: "Gift of Doom", token_words: &[], oracle: true, cost: ManaCost { generic: 4, x: 0, symbols: [0, 0, 1, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Enchantment]), subtypes: SubtypeSet::of(&[Subtype::Aura]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[static_ability(Layer::Ability, cantrip_mechanics::attach::attached_object, ability_1_apply_0, "Enchanted creature has deathtouch and indestructible."),static_ability(Layer::Ability, cantrip_mechanics::attach::attached_object, ability_1_apply_1, "Enchanted creature has deathtouch and indestructible.")], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: Some(target(ability_0_enchant)), ways: &[cantrip_mechanics::morph::way(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(sacrifice(1, ability_2_0)), "Morph—Sacrifice another creature.")], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
