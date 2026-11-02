// Waste Management
// status: ready
// Kicker {3}{B} (You may pay an additional {3}{B} as you cast this spell.)
// Exile up to two target cards from a single graveyard. If this spell was kicked, instead exile target player's graveyard. Create a 2/2 black Rogue creature token for each creature card exiled this way.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_paid_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Player(p) if true) }
static ABILITY_0_PAID_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_0_paid_target_0)];
#[allow(unused_variables)] fn ability_0_unpaid_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if (q.state().obj(id).zone.kind() == ZoneKind::Graveyard) && !q.state().obj(id).token && (true)) }
#[allow(unused_variables)] fn ability_0_unpaid_target_0_same_graveyard(q: &dyn Query, asker: &Asker, t: Target) -> bool { (matches!(t, Target::Object(id) if (q.state().obj(id).zone.kind() == ZoneKind::Graveyard) && !q.state().obj(id).token && (true))) && match (t, q.chosen_targets().first()) { (Target::Object(id), Some(&Target::Object(first))) => first != id && q.state().obj(first).zone == q.state().obj(id).zone, _ => true } }
static ABILITY_0_UNPAID_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_0_unpaid_target_0).optional(),target(ability_0_unpaid_target_0_same_graveyard).optional()];
#[allow(unused_variables)] fn ability_0_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { if q.obj(asker.source).kicked > 0 { (ability_0_paid_target_0)(q, asker, t) } else { (ability_0_unpaid_target_0)(q, asker, t) } }
static ABILITY_0_TARGETS: &[cantrip_api::TargetSpec] = &[TargetSpec { min: 0, max: 2, filter: ability_0_target_0, label: "", if_kicked: Some((1, 1)), together: None }];
pub mod token_2 {
#[allow(unused_imports)] use cantrip_api::*;

pub static CARD: CardDef = CardDef { name: token!(Rogue).name, token_words: token!(Rogue).token_words, oracle: false, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Rogue]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: Some(ColorSet(4)), power: Some(2), toughness: Some(2), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];

}
fn ability_0_paid(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw: Vec<Target> = [announced(g, item, 0, false), announced(g, item, 1, false)].concat(); let t_0: Vec<Target> = [announced(g, item, 0, true), announced(g, item, 1, true)].concat();
let objects_0 = { let q: &dyn Query = g; ((q.state().apnap().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::graveyard(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && (q.state().obj(id).zone.kind() == ZoneKind::Graveyard && t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.player()) == Some(q.state().obj(id).owner))).collect::<Vec<ObjectId>>() }; let moved_0 = g.exile_all(&objects_0);
g.end_sentence();
let players_1 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }; let mut tokens_1 = Vec::new();
let count_1_0 = { let q: &dyn Query = g; (((moved_0.clone()).into_iter().filter(|&id| q.chars(id).is(CardType::Creature)).collect::<Vec<ObjectId>>()).len() as i64) as u32 }; let def_1_0 = g.card_id(&token_2::CARD);
for player in players_1 {
tokens_1.extend(std::iter::repeat((def_1_0,player)).take(count_1_0 as usize));
} let words_1 = g.words(asker.source);
let created_1 = g.create_tokens_worded(&tokens_1, false, None, words_1)?;
g.end_sentence();
 Ok(()) }
pub mod token_5 {
#[allow(unused_imports)] use cantrip_api::*;

pub static CARD: CardDef = CardDef { name: token!(Rogue).name, token_words: token!(Rogue).token_words, oracle: false, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Rogue]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: Some(ColorSet(4)), power: Some(2), toughness: Some(2), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];

}
fn ability_0_unpaid(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw: Vec<Target> = [announced(g, item, 0, false), announced(g, item, 1, false)].concat(); let t_0: Vec<Target> = [announced(g, item, 0, true), announced(g, item, 1, true)].concat();
let objects_3 = { let q: &dyn Query = g; t_0.iter().filter_map(|t| t.object()).collect::<Vec<ObjectId>>() }; let moved_3 = g.exile_all(&objects_3);
g.end_sentence();
let players_4 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }; let mut tokens_4 = Vec::new();
let count_4_0 = { let q: &dyn Query = g; (((moved_3.clone()).into_iter().filter(|&id| q.chars(id).is(CardType::Creature)).collect::<Vec<ObjectId>>()).len() as i64) as u32 }; let def_4_0 = g.card_id(&token_5::CARD);
for player in players_4 {
tokens_4.extend(std::iter::repeat((def_4_0,player)).take(count_4_0 as usize));
} let words_4 = g.words(asker.source);
let created_4 = g.create_tokens_worded(&tokens_4, false, None, words_4)?;
g.end_sentence();
 Ok(()) }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { if item.kicked > 0 { ability_0_paid(g, item) } else { ability_0_unpaid(g, item) } }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }

pub static CARD: CardDef = CardDef { name: "Waste Management", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 0, 1, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Instant]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(ABILITY_0_TARGETS, ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, kicker: kicker(ManaCost { generic: 3, x: 0, symbols: [0, 0, 1, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }), late_targets: true, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[&token_2::CARD,&token_5::CARD];
