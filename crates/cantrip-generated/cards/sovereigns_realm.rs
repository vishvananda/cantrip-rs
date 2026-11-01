// Sovereign's Realm
// status: planned
// requires: ability DraftRule
// requires: ability StartingHandSize
// requires: card type Conspiracy
// requires: layout Conspiracy
// requires: play permission to PlayLand
// (Start the game with this conspiracy face up in the command zone.)
// Your starting deck can't have basic land cards and your starting hand size is five.
// Exile a card from your hand: This turn, you may play basic lands from outside the game.
// Basic lands you control have "{T}: Add one mana of any color."
#[allow(unused_imports)] use cantrip_api::*;
fn ability_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
planned!("play-permission-to-play-land" => if let Some(source) = { let q: &dyn Query = g; Some(asker.source) } { for player in { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() } { g.add_player_effect(player, source, EffectKind::PlayerRule(PlayerRule::Play { zone: ZoneKind::Outside, cards: CardsOfRule::kept(move |q: &dyn Query, asker: &Asker, card: ObjectId| ((q.outside_the_game(asker.controller)).into_iter().filter(|&id| (q.chars(id).supertypes.basic && q.chars(id).is(CardType::Land))).collect::<Vec<ObjectId>>()).contains(&card)), grant: PlayGrant { lands: true, spells: false, ..PlayGrant::CAST } }), Duration::EndOfTurn); } });
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_2_2(q: &dyn Query, asker: &Asker, id: ObjectId) -> bool { true }
#[allow(unused_variables)] fn ability_3_affects(q: &dyn LayerQuery, source: ObjectId, affected: ObjectId, so_far: &Chars) -> bool { let asker = &Asker::new(source, q.controller(source)); ((*so_far).supertypes.basic && (*so_far).is(CardType::Land) && Some(asker.controller) == Some(q.controller(affected))) }
fn granted_3_mana(g: &mut dyn Game, asker: &Asker, chosen: Option<Color>) -> Result<(),Suspend> { let mana_players_4 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
let mana_4 = { let q: &dyn Query = g; (1i64).max(0) as u32 };
for player in mana_players_4 {
let color = chosen.expect("mana activation selected a color");
g.add_mana(player, color.mana(), mana_4);}
 Ok(()) }
#[allow(unused_variables)] fn ability_3_apply_0(q: &dyn LayerQuery, source: ObjectId, chars: &mut Chars) { let asker = &Asker::new(source, q.controller(source)); let card = { let o = q.state().obj(asker.source); o.copied_card().unwrap_or(o.card) }; let _ = card; if let Some(slot) = chars.abilities.iter().position(|a| a.is_none()) { chars.abilities[slot] = Some(AbilityRef::delayed({ let o = q.state().obj(asker.source); o.copied_card().unwrap_or(o.card) }, 0)); } }
planned_item!("ability-draft-rule" => draft_rules: cantrip_mechanics::draft::rule(&[ConspiracyDeck::NoCardPassesFilter(|q: &dyn Query, asker: &Asker, id: ObjectId| (q.chars(id).supertypes.basic && q.chars(id).is(CardType::Land)))]));
planned_item!("ability-starting-hand-size" => statics: starting_hand_size(|q: &dyn Query, asker: &Asker| 5i64));
planned_item!("layout-conspiracy" => variant: Some(Variant::ConspiracyDraft));

pub static CARD: CardDef = CardDef { name: "Sovereign's Realm", token_words: &[], oracle: true, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[planned_value!("card type Conspiracy", CardType::Land => CardType::Conspiracy)]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(exile_from_hand(1, ability_2_2)), ability_2, "Exile a card from your hand: This turn, you may play basic lands from outside the game.").targets(&[])], statics: &[static_ability(Layer::Ability, ability_3_affects, ability_3_apply_0, "Basic lands you control have \"{T}: Add one mana of any color.\"")], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[mana_ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(Cost { tap: true, ..Cost::NONE }), add_by(granted_3_mana).choose_color().colors(ColorSet(31)).yields_any(1), "{T}: Add one mana of any color.")], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
