// Sower of Discord
// status: ready
// Flying
// As this creature enters, choose two players.
// Whenever damage is dealt to one of the chosen players, the other chosen player also loses that much life.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables, unused_mut)] fn ability_1_enters(g: &mut dyn Game, entering: &mut Entering) -> Result<(), Suspend> { let asker = &Asker::new(entering.object, entering.controller); let candidates_0: Vec<PlayerId> = { let q: &dyn Query = g; q.state().apnap().collect::<Vec<PlayerId>>() }; let Some(first_0) = cantrip_mechanics::choose::player(g, entering.controller, entering.object, candidates_0.clone(), "Choose", 0)? else { return Ok(()) }; let left_0: Vec<PlayerId> = candidates_0.into_iter().filter(|&p| p != first_0).collect(); let Some(second_0) = cantrip_mechanics::choose::player(g, entering.controller, entering.object, left_0, "Choose", 0)? else { return Ok(()) }; entering.chosen_player = Some(first_0); entering.chosen_number = Some(u32::from(second_0.0));
 Ok(()) }
#[allow(unused_variables)] fn ability_2_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::Damage { source, target, amount, kind, .. } = event { let (source, amount, combat) = (*source, *amount, kind.is_combat()); let subject = *target; let data = [u32::from(amount), match target { Target::Player(p) => u32::from(p.0), _ => u32::MAX }]; if true && true && matches!(*target, Target::Player(p) if (q.state().apnap().filter(|&bound_0: &PlayerId| Some(bound_0).is_some_and(|p: PlayerId| q.state().chosen_player(q.linked_to(asker)) == Some(p) || q.state().chosen_number(q.linked_to(asker)) == Some(u32::from(p.0)))).collect::<Vec<PlayerId>>()).contains(&p)) { return Some(Fired { subject, data }); } }
 None }
fn ability_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let amount_1 = { let q: &dyn Query = g; (i64::from(item.data[0])).min(i32::MAX.into()) as i32 }; let players_1 = { let q: &dyn Query = g; item.subject.player().and_then(|p: PlayerId| { let (first, second) = (q.state().chosen_player(q.linked(item)), q.state().chosen_number(q.linked(item)).map(|n| PlayerId(n as u8))); if first == Some(p) { second } else if second == Some(p) { first } else { None } }).into_iter().collect::<Vec<PlayerId>>() };
for player in players_1 { g.lose_life(player, amount_1); }
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Sower of Discord", token_words: &[], oracle: true, cost: ManaCost { generic: 4, x: 0, symbols: [0, 0, 2, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Demon]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(6), toughness: Some(6), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Flying]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::DAMAGE.0), ability_2_event), ability_2, "Whenever damage is dealt to one of the chosen players, the other chosen player also loses that much life.").targets(&[]).sums_per_subject()], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: Some(ability_1_enters), modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
