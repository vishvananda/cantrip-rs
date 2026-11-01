// Ranger's Hawk
// status: planned
// requires: engine: venture into the dungeon reaches only Undercity
// Flying
// {3}, {T}, Tap another untapped creature you control: Venture into the dungeon. Activate only as a sorcery. (Enter the first room or advance to the next room.)
#[allow(unused_imports)] use cantrip_api::*;
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
planned!("engine-venture-into-the-dungeon-reaches-only-undercity" => { let player = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; g.venture(player)?; });
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_1_1(q: &dyn Query, asker: &Asker, id: ObjectId) -> bool { (!(Some(asker.source) == Some(id)) && (!(q.state().obj(id).tapped) && q.chars(id).is(CardType::Creature) && Some(asker.controller) == Some(q.controller(id)))) }

pub static CARD: CardDef = CardDef { name: "Ranger's Hawk", token_words: &[], oracle: true, cost: ManaCost { generic: 0, x: 0, symbols: [1, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Bird]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(1), toughness: Some(1), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Flying]), protection: ColorSet(0), spell: None, abilities: &[ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 3, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(Cost { tap: true, ..Cost::NONE }).and(tap_untapped(1, ability_1_1)), ability_1, "{3}, {T}, Tap another untapped creature you control: Venture into the dungeon. Activate only as a sorcery.").targets(&[]).sorcery_speed()], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
