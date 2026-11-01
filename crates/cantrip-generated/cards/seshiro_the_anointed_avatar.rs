// Seshiro the Anointed Avatar
// status: planned
// requires: a choice of CreatureType outside its body
// requires: card type Vanguard
// requires: layout Vanguard
// requires: trigger event Player GameBegins
// At the beginning of the game, choose a creature type. Creatures you control, creature spells you control, and creature cards you own in any zone other than the battlefield or the stack have the chosen type in addition to their other types.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { None }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let player_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let chosen_0 = cantrip_mechanics::chosen_type::creature_type_now(g, player_0, asker.source)?;
planned!("a-choice-of-creature-type-outside-its-body" => let by = { let q: &dyn Query = g; q.linked(item) }; g.remember_chosen(by, ChosenValue::Subtype(chosen_0)););
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_1_affects(q: &dyn LayerQuery, source: ObjectId, affected: ObjectId, so_far: &Chars) -> bool { let asker = &Asker::new(source, q.controller(source)); ((*so_far).is(CardType::Creature) && Some(asker.controller) == Some(q.controller(affected))) }
#[allow(unused_variables)] fn ability_1_apply_0(q: &dyn LayerQuery, source: ObjectId, chars: &mut Chars) { let asker = &Asker::new(source, q.controller(source)); if let Some(chosen) = planned_value!("a choice of CreatureType outside its body", None::<Subtype> => q.state().chosen_subtype(q.linked_to(asker.source))) { chars.subtypes = chars.subtypes.with(chosen); } }
#[allow(unused_variables)] fn ability_2_affects(q: &dyn LayerQuery, source: ObjectId, affected: ObjectId, so_far: &Chars) -> bool { let asker = &Asker::new(source, q.controller(source)); (q.state().obj(affected).became.is_none() && q.state().obj(affected).zone.kind() == ZoneKind::Stack) && ((*so_far).is(CardType::Creature) && Some(asker.controller) == Some(q.controller(affected))) }
#[allow(unused_variables)] fn ability_2_apply_0(q: &dyn LayerQuery, source: ObjectId, chars: &mut Chars) { let asker = &Asker::new(source, q.controller(source)); if let Some(chosen) = planned_value!("a choice of CreatureType outside its body", None::<Subtype> => q.state().chosen_subtype(q.linked_to(asker.source))) { chars.subtypes = chars.subtypes.with(chosen); } }
#[allow(unused_variables)] fn ability_3_affects(q: &dyn LayerQuery, source: ObjectId, affected: ObjectId, so_far: &Chars) -> bool { let asker = &Asker::new(source, q.controller(source)); ((*so_far).is(CardType::Creature) && Some(asker.controller) == Some(q.state().obj(affected).owner)) && { let o = q.state().obj(affected); !o.zone.is(ZoneKind::Battlefield) && !o.token && !o.card_copy } }
#[allow(unused_variables)] fn ability_3_apply_0(q: &dyn LayerQuery, source: ObjectId, chars: &mut Chars) { let asker = &Asker::new(source, q.controller(source)); if let Some(chosen) = planned_value!("a choice of CreatureType outside its body", None::<Subtype> => q.state().chosen_subtype(q.linked_to(asker.source))) { chars.subtypes = chars.subtypes.with(chosen); } }
planned_item!("trigger-event-player-game-begins" => matcher: #[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::GameBegan { player } = event { let subject = Target::Player(*player); let data = [0u32; 2]; if true { return Some(Fired { subject, data }); } }
 None });
planned_item!("layout-vanguard" => variant: Some(Variant::Vanguard { hand: 0, life: -1 }));

pub static CARD: CardDef = CardDef { name: "Seshiro the Anointed Avatar", token_words: &[], oracle: true, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[planned_value!("card type Vanguard", CardType::Land => CardType::Vanguard)]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(planned_value!("trigger event Player GameBegins", EventMask(0) => EventMask(EventMask::GAME_BEGAN.0)), ability_0_event), ability_0, "At the beginning of the game, choose a creature type. Creatures you control, creature spells you control, and creature cards you own in any zone other than the battlefield or the stack have the chosen type in addition to their other types.").targets(&[])], statics: &[static_ability(Layer::Type, ability_1_affects, ability_1_apply_0, "Static: ChosenType."),static_ability(Layer::Type, ability_2_affects, ability_2_apply_0, "Static: ChosenType.").everywhere(),static_ability(Layer::Type, ability_3_affects, ability_3_apply_0, "Static: ChosenType.").everywhere()], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
