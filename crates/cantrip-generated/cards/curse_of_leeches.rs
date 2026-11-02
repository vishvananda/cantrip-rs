// Curse of Leeches // Leeching Lurker
// status: planned
// requires: ability AsEvent
// Lifelink
// Nightbound (If a player casts at least two spells during their own turn, it becomes day next turn.)
// Enchant player
// As this permanent transforms into Curse of Leeches, attach it to a player.
// At the beginning of enchanted player's upkeep, they lose 1 life and you gain 1 life.
// Daybound (If a player casts no spells during their own turn, it becomes night next turn.)
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_enchant(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Player(p) if true) }
planned_item!("ability-as-event" => #[allow(unused_variables)] fn ability_1_as_object(g: &mut dyn Game, asker: &Asker) -> Option<ObjectId> { { let q: &dyn Query = g; Some(asker.source) } });
planned_item!("ability-as-event" => #[allow(unused_variables)] fn ability_1_as_actions(g: &mut dyn Game, asker: &Asker) -> Result<(), Suspend> { cantrip_mechanics::as_events::perform(g, asker, &[TransformAction::AttachPermanentToAPlayer({ let q: &dyn Query = g; Some(asker.source) }, { let q: &dyn Query = g; q.state().apnap().collect::<Vec<PlayerId>>() })]) });
#[allow(unused_variables)] fn ability_2_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::StepBegan { step } = event { let subject = Target::Player(q.state().turn.active); let data = [0u32; 2]; if matches!(*step, Step::Upkeep) && Some(asker.source).and_then(|o: ObjectId| q.state().obj(o).attached_player) == Some(q.state().turn.active) { return Some(Fired { subject, data }); } }
 None }
fn ability_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let amount_0 = { let q: &dyn Query = g; (1i64).max(0) as i32 }; let players_0 = { let q: &dyn Query = g; item.subject.player().into_iter().collect::<Vec<PlayerId>>() };
for player in players_0 { g.lose_life(player, amount_0); }
g.end_sentence();
let amount_1 = { let q: &dyn Query = g; (1i64).max(0) as i32 }; let players_1 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
for player in players_1 { g.gain_life(player, amount_1); }
g.end_sentence();
 Ok(()) }
planned_item!("ability-as-event" => as_events: AsEventDef { event: AsEvent::Transforms, object: ability_1_as_object, actions: ability_1_as_actions });

pub static CARD: CardDef = CardDef { name: "Curse of Leeches", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 0, 1, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Enchantment]), subtypes: SubtypeSet::of(&[Subtype::Aura,Subtype::Curse]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Daybound]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::STEP_BEGAN.0), ability_2_event), ability_2, "At the beginning of enchanted player's upkeep, they lose 1 life and you gain 1 life.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: Some(target(ability_0_enchant)), back: Some(&back::CARD), ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
pub mod back {
#[allow(unused_imports)] use cantrip_api::*;

pub static CARD: CardDef = CardDef { name: "Leeching Lurker", token_words: &[], oracle: true, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Leech,Subtype::Horror]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: Some(ColorSet(4)), power: Some(4), toughness: Some(4), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Lifelink,Keyword::Nightbound]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
}
