// Pramikon, Sky Rampart
// status: planned
// requires: a choice of Direction
// requires: player reference InChosenDirection
// requires: player rule AttackOnly
// Flying, defender
// As Pramikon enters, choose left or right.
// Each player may attack only the nearest opponent in the chosen direction and planeswalkers controlled by that opponent.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables, unused_mut)] fn ability_2_enters(g: &mut dyn Game, entering: &mut Entering) -> Result<(), Suspend> { let asker = &Asker::new(entering.object, entering.controller); let chosen_0 = planned_value!("a choice of Direction", 0u8 => cantrip_mechanics::choose::direction(g, entering.controller, entering.object)?);
 Ok(()) }
#[allow(unused_variables)] fn ability_3_0_applies(q: &dyn Query, asker: &Asker, player: PlayerId) -> bool { true }
planned_item!("player-rule-attack-only" => statics: player_rule(ability_3_0_applies, PlayerRule::AttackOnly(|q: &dyn Query, asker: &Asker, player: PlayerId, defender: Target| (match defender { Target::Player(p) => (planned_value!("player reference InChosenDirection", None::<PlayerId> => q.nearest_in_chosen_direction(asker, Some(player), Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>())).into_iter().collect::<Vec<PlayerId>>()).contains(&p), Target::Object(id) => true && q.chars(id).is(CardType::Planeswalker) && (planned_value!("player reference InChosenDirection", None::<PlayerId> => q.nearest_in_chosen_direction(asker, Some(player), Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>())).into_iter().collect::<Vec<PlayerId>>()).contains(&q.controller(id)), _ => false }))));

pub static CARD: CardDef = CardDef { name: "Pramikon, Sky Rampart", token_words: &[], oracle: true, cost: ManaCost { generic: 0, x: 0, symbols: [1, 1, 0, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Wall]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(1), toughness: Some(5), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Flying,Keyword::Defender]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: Some(ability_2_enters), modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
