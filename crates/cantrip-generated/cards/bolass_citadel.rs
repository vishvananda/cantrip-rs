// Bolas's Citadel
// status: planned
// requires: cost part LifeEqualToOwnManaValue
// requires: play permission for another cost
// You may look at the top card of your library any time.
// You may play lands and cast spells from the top of your library. If you cast a spell this way, pay life equal to its mana value rather than pay its mana cost.
// {T}, Sacrifice ten nonland permanents: Each opponent loses 10 life.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_1_applies(q: &dyn Query, asker: &Asker, player: PlayerId) -> bool { player == asker.controller }
fn ability_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let amount_1 = { let q: &dyn Query = g; (10i64).max(0) as i32 }; let players_1 = { let q: &dyn Query = g; Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>() };
for player in players_1 { g.lose_life(player, amount_1); }
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_2_2(q: &dyn Query, asker: &Asker, id: ObjectId) -> bool { !(q.chars(id).is(CardType::Land)) }
planned_item!("play-permission-for-another-cost" => statics: player_rule(ability_1_1_applies, PlayerRule::Play { zone: ZoneKind::Library, cards: CardsOfRule::Asked(|q: &dyn Query, asker: &Asker, player: PlayerId, card: ObjectId| ((((Some(player).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::library(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && true).collect::<Vec<ObjectId>>()).into_iter().take((1i64).max(0) as usize).collect::<Vec<ObjectId>>()).contains(&card)), grant: PlayGrant { alternate: Some(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(planned_value!("cost part LifeEqualToOwnManaValue", Cost::NONE => Cost::life_equal_to_mana_value()))), ..PlayGrant::CAST } }));

pub static CARD: CardDef = CardDef { name: "Bolas's Citadel", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [0, 0, 3, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Artifact]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(Cost { tap: true, ..Cost::NONE }).and(sacrifice(10, ability_2_2)), ability_2, "{T}, Sacrifice ten nonland permanents: Each opponent loses 10 life.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, play_rules: &[PlayRule::LookAtLibraryTop,PlayRule::LibraryTopLands], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
