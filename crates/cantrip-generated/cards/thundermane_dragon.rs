// Thundermane Dragon
// status: planned
// requires: static play permission with an effect on what is played
// Flying
// You may look at the top card of your library any time.
// You may cast creature spells with power 4 or greater from the top of your library. If you cast a creature spell this way, it gains haste until end of turn.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_2_0_applies(q: &dyn Query, asker: &Asker, player: PlayerId) -> bool { player == asker.controller }
planned_item!("static-play-permission-with-an-effect-on-what-is-played" => statics: player_rule(ability_2_0_applies, PlayerRule::Play { zone: ZoneKind::Library, cards: CardsOfRule::Asked(|q: &dyn Query, asker: &Asker, player: PlayerId, card: ObjectId| (((((Some(player).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::library(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && true).collect::<Vec<ObjectId>>()).into_iter().take((1i64).max(0) as usize).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| (q.chars(id).is(CardType::Creature) && q.chars(id).power.is_some_and(|v| (i64::from(v)) >= (4i64)))).collect::<Vec<ObjectId>>()).contains(&card)), grant: PlayGrant { rider: Some(&[&[SpellEffect::AddAbilityUntil(Keywords::of(&[Keyword::Haste]), Duration::EndOfTurn)]]), ..PlayGrant::CAST } }));

pub static CARD: CardDef = CardDef { name: "Thundermane Dragon", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [0, 0, 0, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Dragon]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(4), toughness: Some(4), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Flying]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, play_rules: &[PlayRule::LookAtLibraryTop], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
