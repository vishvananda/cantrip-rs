// Kresh the Bloodbraided Avatar
// status: planned
// requires: card type Vanguard
// requires: earlier objects Devoured
// requires: layout Vanguard
// requires: trigger event Object Devoured
// Whenever a creature you control is devoured, create an X/X green Ooze creature token, where X is the devoured creature's power.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { None }
pub mod token_1 {
#[allow(unused_imports)] use cantrip_api::*;

pub static CARD: CardDef = CardDef { name: token!(Ooze).name, token_words: token!(Ooze).token_words, oracle: false, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Ooze]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: Some(ColorSet(16)), power: Some(0), toughness: Some(0), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];

}
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let count_0 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let power_0 = { let q: &dyn Query = g; ((planned_value!("earlier objects Devoured", Vec::<ObjectId>::new() => q.acted_on(asker, Acted::Devoured))).into_iter().next().map_or(0i64, |id: ObjectId| i64::from(q.chars(id).power.unwrap_or(0)))).clamp(0, 32767) as i16 }; let toughness_0 = { let q: &dyn Query = g; ((planned_value!("earlier objects Devoured", Vec::<ObjectId>::new() => q.acted_on(asker, Acted::Devoured))).into_iter().next().map_or(0i64, |id: ObjectId| i64::from(q.chars(id).power.unwrap_or(0)))).clamp(0, 32767) as i16 }; let players_0 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }; let mut values_0 = Copiable::of(g.card_id(&token_1::CARD)).sized(power_0, toughness_0); values_0.words = g.words(asker.source); let mut created_0: Vec<ObjectId> = Vec::new(); for player in players_0 { created_0.extend(g.create_token_copies_tapped(values_0, player, count_0, false)?); }
g.end_sentence();
 Ok(()) }
planned_item!("trigger-event-object-devoured" => matcher: #[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::Devoured { creature, by } = event { let id = *creature; let subject = Target::Object(id); let data = [by.0, u32::MAX]; if (q.chars(id).is(CardType::Creature) && Some(asker.controller) == Some(q.controller(id))) { return Some(Fired { subject, data }); } }
 None });
planned_item!("layout-vanguard" => variant: Some(Variant::Vanguard { hand: 1, life: -3 }));

pub static CARD: CardDef = CardDef { name: "Kresh the Bloodbraided Avatar", token_words: &[], oracle: true, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[planned_value!("card type Vanguard", CardType::Land => CardType::Vanguard)]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(planned_value!("trigger event Object Devoured", EventMask(0) => EventMask(EventMask::DEVOURED.0)), ability_0_event), ability_0, "Whenever a creature you control is devoured, create an X/X green Ooze creature token, where X is the devoured creature's power.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[&token_1::CARD];
