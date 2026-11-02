// Xander's Pact
// status: ready
// Casualty 2 (As you cast this spell, you may sacrifice a creature with power 2 or greater. When you do, copy this spell.)
// Each opponent exiles the top card of their library. You may cast spells from among those cards this turn. If you cast a spell this way, pay life equal to that spell's mana value rather than pay its mana cost.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let mut players_0: Vec<PlayerId> = { let q: &dyn Query = g; Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>() };
{ let order: Vec<PlayerId> = g.state().apnap().collect(); players_0.sort_by_key(|p| order.iter().position(|o| o == p)); }
let mut gathered_1: Vec<ObjectId> = Vec::new();
 for (index_0, bound_0) in players_0.iter().copied().enumerate() { gathered_1.extend({ let q: &dyn Query = g; (((Some(bound_0).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::library(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && true).collect::<Vec<ObjectId>>()).into_iter().take((1i64).max(0) as usize).collect::<Vec<ObjectId>>() });
    } 
let objects_3 = { let q: &dyn Query = g; gathered_1.clone() }; let moved_3 = g.exile_all(&objects_3);
g.end_sentence();
let cards_4_0 = { let q: &dyn Query = g; moved_3.clone() }; let players_4_0 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }; cantrip_api::instruction(g, |g| { for &card in &cards_4_0 { if g.obj(card).zone.is(ZoneKind::Exile) { for &player in &players_4_0 { g.allow_play(card, player, PlayGrant { lands: false, expiry: Expiry::EndOfTurn, life_for_mana_value: true, ..PlayGrant::CAST }); } } } });
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Xander's Pact", token_words: &[], oracle: true, cost: ManaCost { generic: 4, x: 0, symbols: [0, 0, 2, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Sorcery]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_1), abilities: &[cantrip_mechanics::casualty::trigger("Casualty")], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, labels: &["Cast without casualty", "Casualty 2"], optional_cost: Some(cantrip_mechanics::casualty::optional::<2>()), ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
