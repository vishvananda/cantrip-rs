// Rise of the Deathbringer
// status: ready
// Choose one —
// • Draw cards equal to the greatest power among creatures you control. You lose life equal to the number of cards drawn this way.
// • All creatures get -3/-3 until end of turn.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0_mode_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let amount_0 = { let q: &dyn Query = g; ((q.state().battlefield().iter().copied().filter(|&id| (q.chars(id).is(CardType::Creature) && Some(asker.controller) == Some(q.controller(id)))).collect::<Vec<ObjectId>>()).into_iter().filter_map(|o: ObjectId| q.chars(o).power).map(i64::from).max().unwrap_or(0)).clamp(0, 4294967295) as u32 }; let players_0 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
let mut done_0: i64 = 0; let mut drawn_0: Vec<ObjectId> = Vec::new(); for player in players_0 { let before = g.history().drawn(player) as i64; let held = g.state().zone(Zone::hand(player)).to_vec(); g.draw(player, amount_0); done_0 += g.history().drawn(player) as i64 - before; drawn_0.extend(g.state().zone(Zone::hand(player)).iter().copied().filter(|c| !held.contains(c))); }
g.end_sentence();
let amount_1 = { let q: &dyn Query = g; ((done_0 as i64)).min(i32::MAX.into()) as i32 }; let players_1 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
for player in players_1 { g.lose_life(player, amount_1); }
g.end_sentence();
 Ok(()) }
fn ability_0_mode_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let objects_2 = { let q: &dyn Query = g; (q.state().battlefield().iter().copied().filter(|&id| q.chars(id).is(CardType::Creature)).collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let source_2 = { let q: &dyn Query = g; Some(asker.source) }; let duration_2 = Duration::EndOfTurn;
let kind_2_0_0 = { let q: &dyn Query = g; EffectKind::Pump((-3i64) as i16, (-3i64) as i16) };
if let Some(source) = source_2 { cantrip_api::instruction(g, |g| { for &object in &objects_2 { g.add_effect(source, object, kind_2_0_0, duration_2);  } }); }
g.end_sentence();
 Ok(()) }
static ABILITY_0_MODES: ModalDef = modes(&[mode(ability_0_mode_0, "Draw cards equal to the greatest power among creatures you control. You lose life equal to the number of cards drawn this way.").targets(&[]),mode(ability_0_mode_1, "All creatures get -3/-3 until end of turn.").targets(&[])]).choose(1, 1);

pub static CARD: CardDef = CardDef { name: "Rise of the Deathbringer", token_words: &[], oracle: true, cost: ManaCost { generic: 4, x: 0, symbols: [0, 0, 1, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Instant]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: Some(SpellDef::MODAL), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: Some(&ABILITY_0_MODES), enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
