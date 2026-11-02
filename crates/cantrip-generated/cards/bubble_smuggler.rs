// Bubble Smuggler
// status: planned
// requires: ability AsEvent
// Disguise {5}{U} (You may cast this card face down for {3} as a 2/2 creature with ward {2}. Turn it face up any time for its disguise cost.)
// As this creature is turned face up, put four +1/+1 counters on it.
#[allow(unused_imports)] use cantrip_api::*;
planned_item!("ability-as-event" => #[allow(unused_variables)] fn ability_1_as_object(g: &mut dyn Game, asker: &Asker) -> Option<ObjectId> { { let q: &dyn Query = g; Some(asker.source) } });
planned_item!("ability-as-event" => #[allow(unused_variables)] fn ability_1_as_actions(g: &mut dyn Game, asker: &Asker) -> Result<(), Suspend> { cantrip_mechanics::as_events::perform(g, asker, &[FaceUpAction::PutCounters(&[FaceUpActionPutCounters::NumberCountersOfTypeOnPermanent({ let q: &dyn Query = g; 4i64 }, CounterType::PTCounter(1, 1), { let q: &dyn Query = g; Some(asker.source) })])]) });
planned_item!("ability-as-event" => as_events: AsEventDef { event: AsEvent::TurnsFaceUp, object: ability_1_as_object, actions: ability_1_as_actions });

pub static CARD: CardDef = CardDef { name: "Bubble Smuggler", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [0, 1, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Octopus,Subtype::Fish]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(2), toughness: Some(1), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ways: &[cantrip_mechanics::disguise::way(Cost::NONE.and(Cost::mana(ManaCost { generic: 5, x: 0, symbols: [0, 1, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })), "Disguise {5}{U}")], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
