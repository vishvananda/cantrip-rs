// Colfenor's Plans
// status: planned
// requires: cards a player may look at
// requires: skipping a step of the turn
// When this enchantment enters, exile the top seven cards of your library face down.
// You may look at the cards exiled with this enchantment, and you may play lands and cast spells from among those cards.
// Skip your draw step.
// You can't cast more than one spell each turn.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ZoneChange { old, new, from, to, .. } = event { let id = *new; let subject = Target::Object(id); let data = [0u32, u32::MAX]; if to.is(ZoneKind::Battlefield) && Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let objects_0 = { let q: &dyn Query = g; (((Some(asker.controller).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::library(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && true).collect::<Vec<ObjectId>>()).into_iter().take((7i64).max(0) as usize).collect::<Vec<ObjectId>>() }; let by_0 = Some(g.linked(item)); let moved_0 = g.exile_face_down(&objects_0, by_0);
g.end_sentence();
 Ok(()) }
planned_item!("cards-a-player-may-look-at" => #[allow(unused_variables)] fn may_look_at_0(q: &dyn Query, asker: &Asker, player: PlayerId, card: ObjectId) -> bool { player == asker.controller && (((q.state().seats().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::exile(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && q.exiled_with(q.linked_to(asker)).contains(&id)).collect::<Vec<ObjectId>>()).contains(&card) });
#[allow(unused_variables)] fn ability_1_1_allows(q: &dyn Query, asker: &Asker, card: ObjectId, player: PlayerId) -> bool { player == asker.controller && q.exiled_with(q.linked_to(asker)).contains(&card) && q.chars(card).is(CardType::Land) }
#[allow(unused_variables)] fn ability_1_2_allows(q: &dyn Query, asker: &Asker, card: ObjectId, player: PlayerId) -> bool { player == asker.controller && q.exiled_with(q.linked_to(asker)).contains(&card) }
planned_item!("skipping-a-step-of-the-turn" => #[allow(unused_variables)] fn ability_2_0_skips_0(q: &dyn Query, asker: &Asker, player: PlayerId) -> bool { player == asker.controller });
#[allow(unused_variables)] fn ability_3_0_rule(q: &dyn Query, asker: &Asker, player: PlayerId, spell: ObjectId) -> bool { !(player == asker.controller && true && q.history().casts_by(player).filter(|c| c.spell != spell && true).count() >= 1) }
planned_item!("cards-a-player-may-look-at" => play_rules: PlayRule::MayLookAt(may_look_at_0));
planned_item!("skipping-a-step-of-the-turn" => skip_steps: SkipStep { applies: ability_2_0_skips_0, step: Step::Draw });

pub static CARD: CardDef = CardDef { name: "Colfenor's Plans", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 0, 2, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Enchantment]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ZONE_CHANGE.0), ability_0_event), ability_0, "When this enchantment enters, exile the top seven cards of your library face down.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, exile_casts: &[ExileCast { text: "Play from exile", allows: ability_1_1_allows, additional: Cost::NONE, any_type: false, lands: true, life_for_mana_value: false },ExileCast { text: "Cast from exile", allows: ability_1_2_allows, additional: Cost::NONE, any_type: false, lands: false, life_for_mana_value: false }], spell_rules: &[ability_3_0_rule], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
