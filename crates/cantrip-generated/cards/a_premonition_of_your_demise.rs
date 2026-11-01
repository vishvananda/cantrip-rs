// A Premonition of Your Demise
// status: planned
// requires: card type Scheme
// requires: layout Scheme
// requires: operation WithReflexive
// requires: trigger event Player SetsSchemeInMotion
// When you set this scheme in motion, reveal the top two cards of your library and put them into your hand. When you reveal one or more nonland cards this way, this scheme deals damage equal to their total mana value to any target.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { None }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let objects_2 = { let q: &dyn Query = g; (((Some(asker.controller).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::library(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && true).collect::<Vec<ObjectId>>()).into_iter().take((2i64).max(0) as usize).collect::<Vec<ObjectId>>() }; let mut owners_2: Vec<PlayerId> = objects_2.iter().map(|o| g.state().obj(*o).owner).collect(); owners_2.dedup(); for owner in owners_2 { let cards: Vec<ObjectId> = objects_2.iter().copied().filter(|o| g.state().obj(*o).owner == owner).collect(); g.reveal(owner, &cards); }
planned!("operation-with-reflexive" => cantrip_mechanics::delayed::reflexive_on(g, item, ReflexiveWhen::Revealed(|q: &dyn Query, asker: &Asker, id: ObjectId| !(q.chars(id).is(CardType::Land))), "Ability(Spell { announcement: Announcement { targets: [TargetSpec { slot: Slot(0), kind: DamageRecipient, count: Exactly(Constant(1)), filter: Any, players: None, distribute: None, constraints: [], chooser: None }], modes: None }, body: Program { instructions: [Effect { result: ResultId(1), operation: DealDamage { source: Source, recipients: Targets(Slot(0)), amount: Earlier { what: Revealed, measure: Total(ManaValue), result: None, filter: None, by: None }, preventable: true } }] } })"));
g.end_sentence();
let objects_4 = { let q: &dyn Query = g; ({ let mut all: Vec<ObjectId> = q.state().battlefield().to_vec(); all.extend_from_slice(q.state().zone(Zone::STACK)); all.extend_from_slice(q.state().zone(Zone::COMMAND)); for p in q.state().seats() { for zone in [Zone::library(p), Zone::hand(p), Zone::graveyard(p), Zone::exile(p)] { all.extend_from_slice(q.state().zone(zone)); } } all }).into_iter().filter(|&id| (planned_value!("depends on an instruction with a gap", Vec::<ObjectId>::new() => result_0)).contains(&id)).collect::<Vec<ObjectId>>() }; let moved_4 = g.return_to_hand_revealed(&objects_4);
g.end_sentence();
 Ok(()) }
planned_item!("trigger-event-player-sets-scheme-in-motion" => matcher: #[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::SchemeSetInMotion { player, object, .. } = event { let id = *object; let subject = Target::Object(id); let data = [u32::from(player.0), 0]; if Some(asker.controller) == Some(*player) && Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None });
planned_item!("layout-scheme" => variant: Some(Variant::Archenemy));

pub static CARD: CardDef = CardDef { name: "A Premonition of Your Demise", token_words: &[], oracle: true, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[planned_value!("card type Scheme", CardType::Land => CardType::Scheme)]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(planned_value!("trigger event Player SetsSchemeInMotion", EventMask(0) => EventMask(EventMask::SCHEME_SET.0)), ability_0_event), ability_0, "When you set this scheme in motion, reveal the top two cards of your library and put them into your hand. When you reveal one or more nonland cards this way, this scheme deals damage equal to their total mana value to any target.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
