// Talon Gates
// status: planned
// requires: card type Plane
// requires: layout Planar
// requires: player rule KeywordAccess
// requires: subtype Dominaria
// requires: trigger event Player ChaosEnsues
// Any time you could cast a sorcery, you may exile a nonland card from your hand with X time counters on it, where X is its mana value. If the exiled card doesn't have suspend, it gains suspend. (At the beginning of its owner's upkeep, they remove a time counter. When the last is removed, the player may cast it without paying its mana cost. If it's a creature, it has haste.)
// Whenever chaos ensues, remove two time counters from each suspended card you own.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_0_applies(q: &dyn Query, asker: &Asker, player: PlayerId) -> bool { player == asker.controller }
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { None }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let mut removed_0: i64 = 0;
let objects_0_0 = { let q: &dyn Query = g; ((q.state().seats().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::exile(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && ((q.state().obj(id).zone.is(ZoneKind::Exile) && q.state().obj(id).counters(CounterKind::Time) > 0 && q.card(q.state().obj(id).card).suspend.is_some()) && Some(asker.controller) == Some(q.state().obj(id).owner))).collect::<Vec<ObjectId>>() };
let amount_0_0 = { let q: &dyn Query = g; (2i64).max(0) as i16 }; cantrip_api::instruction(g, |g| { for object in objects_0_0 { removed_0 += i64::from(g.remove_counters(object, CounterKind::Time, amount_0_0)); } });
g.end_sentence();
 Ok(()) }
planned_item!("player-rule-keyword-access" => statics: player_rule(ability_0_0_applies, PlayerRule::KeywordAccess { keyword: CostedKeyword::Suspend, cards: |q: &dyn Query, asker: &Asker, player: PlayerId, card: ObjectId| !(q.chars(card).is(CardType::Land)), from: ZoneKind::Hand, during: None }));
planned_item!("trigger-event-player-chaos-ensues" => matcher: #[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ChaosEnsued { player } = event { let subject = Target::Player(*player); let data = [0u32; 2]; if true { return Some(Fired { subject, data }); } }
 None });
planned_item!("layout-planar" => variant: Some(Variant::Planechase));

pub static CARD: CardDef = CardDef { name: "Talon Gates", token_words: &[], oracle: true, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[planned_value!("card type Plane", CardType::Land => CardType::Plane)]), subtypes: SubtypeSet::of(&[planned_value!("subtype Dominaria", Subtype::Plains => Subtype::Dominaria)]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(planned_value!("trigger event Player ChaosEnsues", EventMask(0) => EventMask(EventMask::CHAOS_ENSUED.0)), ability_1_event), ability_1, "Whenever chaos ensues, remove two time counters from each suspended card you own.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
