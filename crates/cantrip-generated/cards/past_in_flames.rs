// Past in Flames
// status: ready
// Each instant and sorcery card in your graveyard gains flashback until end of turn. The flashback cost is equal to its mana cost.
// Flashback {4}{R} (You may cast this card from your graveyard for its flashback cost. Then exile it.)
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let cards_0: Vec<ObjectId> = { let q: &dyn Query = g; ((Some(asker.controller).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::graveyard(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && (q.chars(id).is(CardType::Instant) || q.chars(id).is(CardType::Sorcery))).collect::<Vec<ObjectId>>() }; for card in cards_0 { let owner = g.obj(card).owner; g.allow_play(card, owner, PlayGrant { expiry: Expiry::EndOfTurn, flashback: true, ..PlayGrant::CAST }); }
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_1_require(q: &dyn Query, asker: &Asker) -> bool { (q.chars(asker.source).is(CardType::Instant) || q.chars(asker.source).is(CardType::Sorcery)) }

pub static CARD: CardDef = CardDef { name: "Past in Flames", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [0, 0, 0, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Sorcery]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ways: &[way(Cost::NONE.and(Cost::mana(ManaCost { generic: 4, x: 0, symbols: [0, 0, 0, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })), ZoneKind::Graveyard, "Flashback").only_from().exile_after().condition(ability_1_require)], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
