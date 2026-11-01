// Jihad
// status: ready
// As this enchantment enters, choose a color and an opponent.
// White creatures get +2/+1 as long as the chosen player controls a nontoken permanent of the chosen color.
// When the chosen player controls no nontoken permanents of the chosen color, sacrifice this enchantment.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables, unused_mut)] fn ability_0_enters(g: &mut dyn Game, entering: &mut Entering) -> Result<(), Suspend> { let asker = &Asker::new(entering.object, entering.controller); let player_0 = entering.controller; let Answer::Colors(colors_0) = g.ask(player_0, RequestKind::ChooseColors { source: entering.object, choices: ColorSet(31), count: 1, purpose: ColorUse::Any })? else { return Ok(()) }; let chosen_0 = colors_0[0];
entering.chosen_color = Some(chosen_0);
let player_1 = entering.controller; let candidates_1 = { let q: &dyn Query = g; Target::Object(asker.source).object().map(|o: ObjectId| q.controller(o)).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>() }; let Some(chosen_1) = cantrip_mechanics::choose::player(g, player_1, entering.object, candidates_1, "Choose", 0)? else { return Ok(()) };
entering.chosen_player = Some(chosen_1);
 Ok(()) }
#[allow(unused_variables)] fn ability_1_0_affects(q: &dyn LayerQuery, source: ObjectId, affected: ObjectId, so_far: &Chars) -> bool { let asker = &Asker::new(source, q.controller(source)); (q.state().battlefield().iter().copied().filter(|&id| q.controller(id) == q.state().chosen_player(q.linked_to(asker.source)).unwrap_or(PlayerId(u8::MAX)) && (!(q.state().obj(id).token) && q.state().chosen_color(q.linked_to(asker.source)).is_some_and(|chosen| q.chars(id).colors.has(chosen)))).count() >= 1) && (((*so_far).colors.has(q.words(asker.source).color(Color::White)) && (*so_far).is(CardType::Creature))) }
#[allow(unused_variables)] fn ability_1_0_apply_0(q: &dyn LayerQuery, source: ObjectId, chars: &mut Chars) { let asker = &Asker::new(source, q.controller(source)); let (p, t) = ((2i64) as i16, (1i64) as i16); chars.power = chars.power.map(|v| v.saturating_add(p)); chars.toughness = chars.toughness.map(|v| v.saturating_add(t)); }
fn ability_2_state(q: &dyn Query, asker: &Asker) -> bool { ((q.state().battlefield().iter().copied().filter(|&id| ((!(q.state().obj(id).token) && q.state().chosen_color(q.linked_to(asker)).is_some_and(|chosen| q.chars(id).colors.has(chosen))) && q.state().chosen_player(q.linked_to(asker)) == Some(q.controller(id)))).collect::<Vec<ObjectId>>()).len() as i64) == (0i64) }
fn ability_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let objects_2 = { let q: &dyn Query = g; Some(asker.source).into_iter().collect::<Vec<ObjectId>>() }; let players_2 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }; let mut picks_2: Vec<(PlayerId, ObjectId)> = Vec::new();
for &object in &objects_2 { let controller = g.controller(object); if players_2.contains(&controller) && g.can_sacrifice(controller, object) { picks_2.push((controller, object)); } }
let sacrificed_2 = g.sacrifice_each(&picks_2);
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Jihad", token_words: &[], oracle: true, cost: ManaCost { generic: 0, x: 0, symbols: [3, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Enchantment]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[state_triggered(ability_2_state, ability_2, "When the chosen player controls no nontoken permanents of the chosen color, sacrifice this enchantment.")], statics: &[static_ability(Layer::PtModify, ability_1_0_affects, ability_1_0_apply_0, "White creatures get +2/+1 as long as the chosen player controls a nontoken permanent of the chosen color.")], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: Some(ability_0_enters), modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
