// The Horus Heresy
// status: ready
// (As this Saga enters and after your draw step, add a lore counter. Sacrifice after III.)
// I — For each opponent, gain control of up to one target nonlegendary creature that player controls for as long as this Saga remains on the battlefield.
// II — Draw a card for each creature you control but don't own.
// III — Starting with you, each player chooses a creature. Destroy each creature chosen this way.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_0_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if cantrip_mechanics::targets::controlled_by_slot_opponent(q, id)) && (matches!(t, Target::Object(id) if cantrip_mechanics::targets::permanent(q, id) && ((!(q.chars(id).supertypes.legendary) && q.chars(id).is(CardType::Creature))))) }
static ABILITY_0_0_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_0_0_target_0).between(0, TargetSpec::EACH_OPPONENT)];
fn ability_0_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let objects_0 = { let q: &dyn Query = g; (t_0.iter().filter_map(|t| t.object()).collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let source_0 = { let q: &dyn Query = g; Some(asker.source) }; let duration_0 = Duration::WhileSourceOnBattlefield;
let kind_0_0_0 = { let q: &dyn Query = g; EffectKind::Control(Some(asker.controller).expect("a reference that always names a player")) };
if let Some(source) = source_0 { cantrip_api::instruction(g, |g| { for &object in &objects_0 { g.add_effect(source, object, kind_0_0_0, duration_0);  } }); }
g.end_sentence();
 Ok(()) }
fn ability_0_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let amount_1 = { let q: &dyn Query = g; ((q.state().battlefield().iter().copied().filter(|&id| (q.chars(id).is(CardType::Creature) && Some(asker.controller) == Some(q.controller(id)) && ({ let other = Some(asker.controller); q.state().apnap().filter(|p| Some(*p) != other).collect::<Vec<PlayerId>>() }).contains(&q.state().obj(id).owner))).collect::<Vec<ObjectId>>()).len() as i64) as u32 }; let players_1 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
for player in players_1 { g.draw(player, amount_1); }
g.end_sentence();
 Ok(()) }
fn ability_0_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let mut players_2: Vec<PlayerId> = { let q: &dyn Query = g; q.state().apnap().collect::<Vec<PlayerId>>() };
let first_2 = { let q: &dyn Query = g; Some(asker.controller) }; if let Some(at) = players_2.iter().position(|&p| Some(p) == first_2) { players_2.rotate_left(at); }
let mut each_4: Vec<Vec<ObjectId>> = vec![Vec::new(); players_2.len()]; let mut all_4: Vec<ObjectId> = Vec::new();
 for (index_2, bound_0) in players_2.iter().copied().enumerate() { let selected_3_player = { let q: &dyn Query = g; Some(bound_0).expect("a reference that always names a player") }; let selected_3_candidates = { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| q.chars(id).is(CardType::Creature)).collect::<Vec<ObjectId>>() }; let selected_3_count = { let q: &dyn Query = g; cantrip_mechanics::select::Count::AsManyAsCan((1i64).max(0) as u32) }; let selected_3 = { cantrip_mechanics::choose::select(g, selected_3_player, asker.source, Purpose::ChooseAsEnters, selected_3_candidates, cantrip_mechanics::select::Selection::of(selected_3_count, &[]))? };
 each_4[index_2] = selected_3.clone(); all_4.extend(each_4[index_2].iter().copied());
   } 
g.end_sentence();
let objects_5 = { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| (q.chars(id).is(CardType::Creature) && (all_4.clone()).contains(&id))).collect::<Vec<ObjectId>>() }; let destroyed_5 = g.destroy_all(&objects_5);
g.end_sentence();
 Ok(()) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }

pub static CARD: CardDef = CardDef { name: "The Horus Heresy", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [0, 1, 1, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Enchantment]), subtypes: SubtypeSet::of(&[Subtype::Saga]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[chapter(1, ability_0_0, "I — For each opponent, gain control of up to one target nonlegendary creature that player controls for as long as this Saga remains on the battlefield.").targets(ABILITY_0_0_TARGETS),chapter(2, ability_0_1, "II — Draw a card for each creature you control but don't own.").targets(&[]),chapter(3, ability_0_2, "III — Starting with you, each player chooses a creature. Destroy each creature chosen this way.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
