// Archangel of Strife
// status: planned
// requires: condition VotedFor
// Flying
// As this creature enters, each player chooses war or peace.
// Creatures controlled by players who chose war get +3/+0.
// Creatures controlled by players who chose peace get +0/+3.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables, unused_mut)] fn ability_1_enters(g: &mut dyn Game, entering: &mut Entering) -> Result<(), Suspend> { let asker = &Asker::new(entering.object, entering.controller); let mut players_0: Vec<PlayerId> = { let q: &dyn Query = g; q.state().apnap().collect::<Vec<PlayerId>>() };
let mut each_chosen_1 = Vec::new();
let mut each_chosen_1_who: Vec<PlayerId> = Vec::new();
 for (index_0, bound_0) in players_0.iter().copied().enumerate() { cantrip_mechanics::choose::option_as_enters(g, entering, &CARD, 0..2)?; let chosen_1: u8 = entering.chosen_option.map_or(0, |chosen| chosen.index);
 each_chosen_1.push(chosen_1);
each_chosen_1_who.push(bound_0);
   } 
 Ok(()) }
#[allow(unused_variables)] fn ability_2_affects(q: &dyn LayerQuery, source: ObjectId, affected: ObjectId, so_far: &Chars) -> bool { let asker = &Asker::new(source, q.controller(source)); ((*so_far).is(CardType::Creature) && (q.state().apnap().filter(|&bound_0: &PlayerId| planned_value!("condition VotedFor", false => Some(bound_0).is_some_and(|p: PlayerId| q.votes().voted(p, cantrip_mechanics::vote::Vote::Word("War"))))).collect::<Vec<PlayerId>>()).contains(&q.controller(affected))) }
#[allow(unused_variables)] fn ability_2_apply_0(q: &dyn LayerQuery, source: ObjectId, chars: &mut Chars) { let asker = &Asker::new(source, q.controller(source)); let (p, t) = ((3i64) as i16, (0i64) as i16); chars.power = chars.power.map(|v| v.saturating_add(p)); chars.toughness = chars.toughness.map(|v| v.saturating_add(t)); }
#[allow(unused_variables)] fn ability_3_affects(q: &dyn LayerQuery, source: ObjectId, affected: ObjectId, so_far: &Chars) -> bool { let asker = &Asker::new(source, q.controller(source)); ((*so_far).is(CardType::Creature) && (q.state().apnap().filter(|&bound_0: &PlayerId| planned_value!("condition VotedFor", false => Some(bound_0).is_some_and(|p: PlayerId| q.votes().voted(p, cantrip_mechanics::vote::Vote::Word("Peace"))))).collect::<Vec<PlayerId>>()).contains(&q.controller(affected))) }
#[allow(unused_variables)] fn ability_3_apply_0(q: &dyn LayerQuery, source: ObjectId, chars: &mut Chars) { let asker = &Asker::new(source, q.controller(source)); let (p, t) = ((0i64) as i16, (3i64) as i16); chars.power = chars.power.map(|v| v.saturating_add(p)); chars.toughness = chars.toughness.map(|v| v.saturating_add(t)); }

pub static CARD: CardDef = CardDef { name: "Archangel of Strife", token_words: &[], oracle: true, cost: ManaCost { generic: 5, x: 0, symbols: [2, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Angel]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(6), toughness: Some(6), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Flying]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[static_ability(Layer::PtModify, ability_2_affects, ability_2_apply_0, "Creatures controlled by players who chose war get +3/+0."),static_ability(Layer::PtModify, ability_3_affects, ability_3_apply_0, "Creatures controlled by players who chose peace get +0/+3.")], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: Some(ability_1_enters), modal: None, enchant: None, labels: &["War", "Peace"], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
