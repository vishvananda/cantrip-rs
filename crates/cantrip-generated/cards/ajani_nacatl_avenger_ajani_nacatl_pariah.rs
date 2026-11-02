// Ajani, Nacatl Avenger // Ajani, Nacatl Pariah
// status: planned
// requires: one or more that die read as each in the data
// [+2]: Put a +1/+1 counter on each Cat you control.
// [0]: Create a 2/1 white Cat Warrior creature token. When you do, if you control a red permanent other than Ajani, he deals damage equal to the number of creatures you control to any target.
// [−4]: Each opponent chooses an artifact, a creature, an enchantment, and a planeswalker from among the nonland permanents they control, then sacrifices the rest.
// When Ajani enters, create a 2/1 white Cat Warrior creature token.
// Whenever one or more other Cats you control die, you may exile Ajani, then return him to the battlefield transformed under his owner's control.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ZoneChange { old, new, from, to, .. } = event { let id = *new; let subject = Target::Object(id); let data = [0u32, u32::MAX]; if to.is(ZoneKind::Battlefield) && Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
pub mod token_1 {
#[allow(unused_imports)] use cantrip_api::*;

pub static CARD: CardDef = CardDef { name: token!(Cat Warrior).name, token_words: token!(Cat Warrior).token_words, oracle: false, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Cat,Subtype::Warrior]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: Some(ColorSet(1)), power: Some(2), toughness: Some(1), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];

}
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let players_0 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }; let mut tokens_0 = Vec::new();
let count_0_0 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let def_0_0 = g.card_id(&token_1::CARD);
for player in players_0 {
tokens_0.extend(std::iter::repeat((def_0_0,player)).take(count_0_0 as usize));
} let words_0 = g.words(asker.source);
let created_0 = g.create_tokens_worded(&tokens_0, false, None, words_0)?;
g.end_sentence();
 Ok(()) }
planned_item!("one-or-more-that-die-read-as-each-in-the-data", "(kind:Triggered,args:(event:ZoneChange(objects:(kind:All,args:[(kind:Not,args:(kind:SameObject,args:(kind:Source))),(kind:All,args:[(kind:Subtype,args:Cat),(kind:ControlledBy,args:(kind:One,args:(kind:AbilityController)))])]),from:Some(Battlefield),to:Graveyard),intervening_if:None,limits:(per_turn:None,total:None),announcement:(targets:[],modes:None),body:(instructions:[(kind:Optional,args:(decider:(kind:AbilityController),body:(instructions:[(kind:Effect,args:(result:(0),operation:(kind:Move,args:(objects:(kind:One,args:(kind:Source)),destination:Exile,entry:(tapped:false,counters:[],states:[]),face_down:false)))),(kind:Effect,args:(result:(1),operation:(kind:Move,args:(objects:(kind:InZone,args:(objects:(kind:One,args:(kind:Result,args:(result:(0),phase:After,index:0))),zone:Exile)),destination:OwnersBattlefield,entry:(tapped:false,counters:[],states:[Transformed]),face_down:false))))])))])))");

pub static CARD: CardDef = CardDef { name: "Ajani, Nacatl Pariah", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [1, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Cat,Subtype::Warrior]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(1), toughness: Some(2), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ZONE_CHANGE.0), ability_0_event), ability_0, "When Ajani enters, create a 2/1 white Cat Warrior creature token.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, back: Some(&back::CARD), ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[&token_1::CARD,&back::token_2::CARD];
pub mod back {
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let mut counters_0: Vec<(ObjectId, CounterKind, i16)> = Vec::new();
let amount = { let q: &dyn Query = g; (1i64).max(0) as i16 }; for t in { let q: &dyn Query = g; (q.state().battlefield().iter().copied().filter(|&id| (q.chars(id).has_subtype(q.words(asker.source).subtype(Subtype::Cat)) && Some(asker.controller) == Some(q.controller(id)))).collect::<Vec<ObjectId>>()).into_iter().map(Target::Object).collect::<Vec<Target>>() } { if let Target::Object(object) = t { counters_0.push((object, CounterKind::PlusOne, amount)); } }
let had_0: Vec<_> = counters_0.iter().map(|c| g.state().obj(c.0).counters(c.1)).collect(); g.add_counters_together(&counters_0); let applied_0: Vec<ObjectId> = counters_0.iter().zip(had_0).filter(|(c, had)| g.state().obj(c.0).counters(c.1) > *had).map(|(c, _)| c.0).collect();
g.end_sentence();
 Ok(()) }
pub mod token_2 {
#[allow(unused_imports)] use cantrip_api::*;

pub static CARD: CardDef = CardDef { name: token!(Cat Warrior).name, token_words: token!(Cat Warrior).token_words, oracle: false, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Cat,Subtype::Warrior]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: Some(ColorSet(1)), power: Some(2), toughness: Some(1), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];

}
static REFLEXIVE_4_TARGETS: &[cantrip_api::TargetSpec] = &[cantrip_mechanics::targets::ANY];
fn reflexive_4(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller); if !{ let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| q.controller(id) == Some(asker.controller).unwrap_or(PlayerId(u8::MAX)) && (q.chars(id).colors.has(q.words(asker.source).color(Color::Red)) && (q.chars(id).is(CardType::Artifact) || q.chars(id).is(CardType::Battle) || q.chars(id).is(CardType::Creature) || q.chars(id).is(CardType::Enchantment) || q.chars(id).is(CardType::Land) || q.chars(id).is(CardType::Planeswalker)) && !(Some(asker.source) == Some(id)))).count() >= 1 } { return Ok(()); } let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let source_5 = { let q: &dyn Query = g; Some(asker.source) }; let amount_5 = { let q: &dyn Query = g; ((q.state().battlefield().iter().copied().filter(|&id| (q.chars(id).is(CardType::Creature) && Some(asker.controller) == Some(q.controller(id)))).collect::<Vec<ObjectId>>()).len() as i64).clamp(0, 65535) as u16 }; let recipients_5 = { let q: &dyn Query = g; t_0.clone() }; if let Some(source) = source_5 { let batch: Vec<(ObjectId, Target, u16)> = recipients_5.iter().map(|t| (source, *t, amount_5)).collect(); g.damage_together(&batch); }
g.end_sentence();
 Ok(()) }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let players_1 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }; let mut tokens_1 = Vec::new();
let count_1_0 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let def_1_0 = g.card_id(&token_2::CARD);
for player in players_1 {
tokens_1.extend(std::iter::repeat((def_1_0,player)).take(count_1_0 as usize));
} let words_1 = g.words(asker.source);
let created_1 = g.create_tokens_worded(&tokens_1, false, None, words_1)?;
if !created_1.clone().is_empty() && { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| q.controller(id) == Some(asker.controller).unwrap_or(PlayerId(u8::MAX)) && (q.chars(id).colors.has(q.words(asker.source).color(Color::Red)) && (q.chars(id).is(CardType::Artifact) || q.chars(id).is(CardType::Battle) || q.chars(id).is(CardType::Creature) || q.chars(id).is(CardType::Enchantment) || q.chars(id).is(CardType::Land) || q.chars(id).is(CardType::Planeswalker)) && !(Some(asker.source) == Some(id)))).count() >= 1 } { g.reflexive_trigger(item.object, item.controller, AbilityRef::delayed(item.ability.card, 0), Target::None); }
g.end_sentence();
 Ok(()) }
fn ability_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let mut players_6: Vec<PlayerId> = { let q: &dyn Query = g; Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>() };
let mut each_8: Vec<Vec<ObjectId>> = vec![Vec::new(); players_6.len()]; let mut all_8: Vec<ObjectId> = Vec::new();
 for (index_6, bound_0) in players_6.iter().copied().enumerate() { let player_7 = { let q: &dyn Query = g; Some(bound_0).expect("a reference that always names a player") }; let pool_7 = { let q: &dyn Query = g; (q.state().battlefield().iter().copied().filter(|&id| !(q.chars(id).is(CardType::Land))).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| Some(bound_0) == Some(q.controller(id))).collect::<Vec<ObjectId>>() }; let mut picked_7: Vec<ObjectId> = Vec::new();
{ let open = { let q: &dyn Query = g; pool_7.iter().copied().filter(|id| !picked_7.contains(id)).filter(|&id| q.chars(id).is(CardType::Artifact)).collect::<Vec<ObjectId>>() }; let one = cantrip_mechanics::choose::cards(g, player_7, asker.source, Purpose::Keep, open, 1, false)?; for o in one { if !picked_7.contains(&o) { picked_7.push(o); } } }
{ let open = { let q: &dyn Query = g; pool_7.iter().copied().filter(|id| !picked_7.contains(id)).filter(|&id| q.chars(id).is(CardType::Creature)).collect::<Vec<ObjectId>>() }; let one = cantrip_mechanics::choose::cards(g, player_7, asker.source, Purpose::Keep, open, 1, false)?; for o in one { if !picked_7.contains(&o) { picked_7.push(o); } } }
{ let open = { let q: &dyn Query = g; pool_7.iter().copied().filter(|id| !picked_7.contains(id)).filter(|&id| q.chars(id).is(CardType::Enchantment)).collect::<Vec<ObjectId>>() }; let one = cantrip_mechanics::choose::cards(g, player_7, asker.source, Purpose::Keep, open, 1, false)?; for o in one { if !picked_7.contains(&o) { picked_7.push(o); } } }
{ let open = { let q: &dyn Query = g; pool_7.iter().copied().filter(|id| !picked_7.contains(id)).filter(|&id| q.chars(id).is(CardType::Planeswalker)).collect::<Vec<ObjectId>>() }; let one = cantrip_mechanics::choose::cards(g, player_7, asker.source, Purpose::Keep, open, 1, false)?; for o in one { if !picked_7.contains(&o) { picked_7.push(o); } } }
 each_8[index_6] = picked_7.clone(); all_8.extend(each_8[index_6].iter().copied());
   } 
let objects_9 = { let q: &dyn Query = g; { let chosen: Vec<ObjectId> = all_8.clone(); (players_6.iter().copied().flat_map(|bound_0: PlayerId| (q.state().battlefield().iter().copied().filter(|&id| !(q.chars(id).is(CardType::Land))).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| Some(bound_0) == Some(q.controller(id))).collect::<Vec<ObjectId>>()).collect::<Vec<ObjectId>>()).into_iter().filter(|o| !chosen.contains(o)).collect::<Vec<ObjectId>>() } }; let players_9 = { let q: &dyn Query = g; Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>() }; let mut picks_9: Vec<(PlayerId, ObjectId)> = Vec::new();
for &object in &objects_9 { let controller = g.controller(object); if players_9.contains(&controller) && g.can_sacrifice(controller, object) { picks_9.push((controller, object)); } }
let sacrificed_9 = g.sacrifice_each(&picks_9);
g.end_sentence();
 Ok(()) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }

pub static CARD: CardDef = CardDef { name: "Ajani, Nacatl Avenger", token_words: &[], oracle: true, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[CardType::Planeswalker]), subtypes: SubtypeSet::of(&[Subtype::Ajani]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: Some(ColorSet(9)), power: None, toughness: None, loyalty: Some(3), defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(loyalty(2)), ability_0, "+2: Put a +1/+1 counter on each Cat you control.").targets(&[]).sorcery_speed(),ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(loyalty(0)), ability_1, "0: Create a 2/1 white Cat Warrior creature token. When you do, if you control a red permanent other than Ajani, he deals damage equal to the number of creatures you control to any target.").targets(&[]).sorcery_speed(),ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(loyalty(-4)), ability_2, "−4: Each opponent chooses an artifact, a creature, an enchantment, and a planeswalker from among the nonland permanents they control, then sacrifices the rest.").targets(&[]).sorcery_speed()], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[delayed_trigger(reflexive_4, "When you do, if you control a red permanent other than Ajani, he deals damage equal to the number of creatures you control to any target.").targets(REFLEXIVE_4_TARGETS)], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
}
