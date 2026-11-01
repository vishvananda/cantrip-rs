// Vision, Spectral Synthezoid
// status: planned
// requires: play permission from a hand once each turn
// Flying
// Once during each of your turns, you may cast a noncreature or Robot spell from your hand without paying its mana cost.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_0_applies(q: &dyn Query, asker: &Asker, player: PlayerId) -> bool { player == asker.controller }
planned_item!("play-permission-from-a-hand-once-each-turn" => statics: player_rule(ability_1_0_applies, PlayerRule::Play { zone: ZoneKind::Hand, cards: CardsOfRule::Asked(|q: &dyn Query, asker: &Asker, player: PlayerId, card: ObjectId| ((((Some(player).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::hand(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && true).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| (!(q.chars(id).is(CardType::Creature)) || q.chars(id).has_subtype(q.words(asker.source).subtype(Subtype::Robot)))).collect::<Vec<ObjectId>>()).contains(&card)), grant: PlayGrant { limit: PlayLimit::EachTurn(1), instead: Some(ManaCost::ZERO), ..PlayGrant::CAST } }));

pub static CARD: CardDef = CardDef { name: "Vision, Spectral Synthezoid", token_words: &[], oracle: true, cost: ManaCost { generic: 6, x: 0, symbols: [0, 2, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Artifact,CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Robot,Subtype::Hero]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(2), toughness: Some(5), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Flying]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
