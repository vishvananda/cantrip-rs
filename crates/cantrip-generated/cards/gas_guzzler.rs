// Gas Guzzler
// status: ready
// Start your engines! (If you have no speed, it starts at 1. It increases once on each of your turns when an opponent loses life. Max speed is 4.)
// This creature enters tapped.
// Max speed — {B}, Sacrifice another creature or Vehicle: Draw a card.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables, unused_mut)] fn ability_1_enters(g: &mut dyn Game, entering: &mut Entering) -> Result<(), Suspend> { let asker = &Asker::new(entering.object, entering.controller); entering.tapped = true;
 Ok(()) }
fn ability_2_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let amount_1 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let players_1 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
for player in players_1 { g.draw(player, amount_1); }
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_2_0_2(q: &dyn Query, asker: &Asker, id: ObjectId) -> bool { (!(Some(asker.source) == Some(id)) && (q.chars(id).is(CardType::Creature) || q.chars(id).has_subtype(q.words(asker.source).subtype(Subtype::Vehicle)))) }
fn ability_2_0_if(q: &dyn Query, asker: &Asker) -> bool { Some(asker.controller).is_some_and(|player: PlayerId| q.state().player(player).speed == Some(4)) }

pub static CARD: CardDef = CardDef { name: "Gas Guzzler", token_words: &[], oracle: true, cost: ManaCost { generic: 0, x: 0, symbols: [0, 0, 1, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Vampire,Subtype::Rogue]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(2), toughness: Some(1), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 1, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(sacrifice(1, ability_2_0_2)), ability_2_0, "Max speed — {B}, Sacrifice another creature or Vehicle: Draw a card.").targets(&[]).condition(ability_2_0_if)], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: Some(ability_1_enters), modal: None, enchant: None, start_your_engines: true, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
