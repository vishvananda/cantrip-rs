// Lumbering Laundry
// status: planned
// requires: a card a player may look at for a while
// {2}: Until end of turn, you may look at face-down creatures you don't control any time.
// Disguise {5} (You may cast this card face down for {3} as a 2/2 creature with ward {2}. Turn it face up any time for its disguise cost.)
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
planned!("a-card-a-player-may-look-at-for-a-while" => let cards_0_0 = { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| (q.state().obj(id).face_down && q.chars(id).is(CardType::Creature) && ({ let other = Some(asker.controller); q.state().apnap().filter(|p| Some(*p) != other).collect::<Vec<PlayerId>>() }).contains(&q.controller(id)))).collect::<Vec<ObjectId>>() }; let players_0_0 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }; for &card in &cards_0_0 { for &player in &players_0_0 { g.allow_look(card, player, Expiry::EndOfTurn); } });
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Lumbering Laundry", token_words: &[], oracle: true, cost: ManaCost { generic: 5, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Artifact,CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Golem]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(4), toughness: Some(5), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 2, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })), ability_0, "{2}: Until end of turn, you may look at face-down creatures you don't control any time.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ways: &[cantrip_mechanics::disguise::way(Cost::NONE.and(Cost::mana(ManaCost { generic: 5, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })), "Disguise {5}")], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
