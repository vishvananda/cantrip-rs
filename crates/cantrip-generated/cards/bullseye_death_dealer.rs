// Bullseye, Death Dealer
// status: planned
// requires: engine: no choice between the costs of an ability
// When Bullseye enters, you may sacrifice an artifact or discard a nonland card. When you do, Bullseye deals 2 damage to any target.
// {3}, {T}, Sacrifice an artifact or discard a nonland card: Bullseye deals 2 damage to any target.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ZoneChange { old, new, from, to, .. } = event { let id = *new; let subject = Target::Object(id); let data = [0u32, u32::MAX]; if to.is(ZoneKind::Battlefield) && Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
static REFLEXIVE_3_TARGETS: &[cantrip_api::TargetSpec] = &[cantrip_mechanics::targets::ANY];
fn reflexive_3(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let source_4 = { let q: &dyn Query = g; Some(asker.source) }; let amount_4 = { let q: &dyn Query = g; (2i64).max(0) as u16 }; let recipients_4 = { let q: &dyn Query = g; t_0.clone() }; if let Some(source) = source_4 { let batch: Vec<(ObjectId, Target, u16)> = recipients_4.iter().map(|t| (source, *t, amount_4)).collect(); g.damage_together(&batch); }
g.end_sentence();
 Ok(()) }
static REFLEXIVE_10_TARGETS: &[cantrip_api::TargetSpec] = &[cantrip_mechanics::targets::ANY];
fn reflexive_10(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let source_11 = { let q: &dyn Query = g; Some(asker.source) }; let amount_11 = { let q: &dyn Query = g; (2i64).max(0) as u16 }; let recipients_11 = { let q: &dyn Query = g; t_0.clone() }; if let Some(source) = source_11 { let batch: Vec<(ObjectId, Target, u16)> = recipients_11.iter().map(|t| (source, *t, amount_11)).collect(); g.damage_together(&batch); }
g.end_sentence();
 Ok(()) }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_14: Vec<ObjectId> = Vec::new();
let mut hoisted_13: Vec<ObjectId> = Vec::new();
let mut hoisted_12: Vec<ObjectId> = Vec::new();
let mut hoisted_6: Vec<ObjectId> = Vec::new();
let mut hoisted_5: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
let decider_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") };
let offered_0: Vec<u8> = [(0u8, { let q: &dyn Query = g; { let players: Vec<PlayerId> = Some(asker.controller).into_iter().collect::<Vec<PlayerId>>(); (q.state().battlefield().iter().copied().filter(|&id| q.chars(id).is(CardType::Artifact)).collect::<Vec<ObjectId>>()).into_iter().any(|o: ObjectId| players.contains(&q.obj(o).controller)) } }), (1u8, !{ let q: &dyn Query = g; ((Some(asker.controller).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::hand(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && !(q.chars(id).is(CardType::Land))).collect::<Vec<ObjectId>>() }.is_empty()), (2u8, true)].into_iter().filter(|option| option.1).map(|option| option.0).collect(); let card_0 = item.ability.card; let picked_0 = if offered_0.len() > 1 { offered_0[cantrip_mechanics::choose::one_of(g, decider_0, asker.source, offered_0.iter().map(|&option| Label::Card(card_0, option)).collect())? as usize] } else { offered_0.first().copied().unwrap_or(u8::MAX) }; match picked_0 {
0 => {
let objects_1 = { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| q.chars(id).is(CardType::Artifact)).collect::<Vec<ObjectId>>() }; let players_1 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }; let mut picks_1: Vec<(PlayerId, ObjectId)> = Vec::new();
let count_1 = { let q: &dyn Query = g; cantrip_mechanics::select::Count::AsManyAsCan((1i64).max(0) as u32) }; for player in players_1 { let eligible: Vec<ObjectId> = objects_1.iter().copied().filter(|&o| g.controller(o) == player && g.can_sacrifice(player, o)).collect(); let taken = match count_1.clone() { cantrip_mechanics::select::Count::Exactly(n) | cantrip_mechanics::select::Count::AsManyAsCan(n) if eligible.len() <= n as usize => eligible, _ => cantrip_mechanics::choose::select(g, player, asker.source, Purpose::Sacrifice, eligible, cantrip_mechanics::select::Selection::of(count_1.clone(), &[]))? }; for object in taken { picks_1.push((player, object)); } }
let sacrificed_1 = g.sacrifice_each(&picks_1);
if !sacrificed_1.clone().is_empty() && true { g.reflexive_trigger(item.object, item.controller, AbilityRef::delayed(item.ability.card, 0), Target::None); }
g.end_sentence();
hoisted_5.extend(sacrificed_1.clone());
hoisted_6.extend({ let q: &dyn Query = g; objects_1.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>() });
}
1 => {
let selected_7_player = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let selected_7_candidates = { let q: &dyn Query = g; ((Some(asker.controller).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::hand(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && !(q.chars(id).is(CardType::Land))).collect::<Vec<ObjectId>>() }; let selected_7_count = { let q: &dyn Query = g; cantrip_mechanics::select::Count::AsManyAsCan((1i64).max(0) as u32) }; let selected_7 = { cantrip_mechanics::choose::select(g, selected_7_player, asker.source, Purpose::Discard, selected_7_candidates, cantrip_mechanics::select::Selection::of(selected_7_count, &[]))? };
let cards_8 = { let q: &dyn Query = g; selected_7.clone() }; let discarded_8 = g.discard_together(&cards_8);
if !discarded_8.clone().is_empty() && true { g.reflexive_trigger(item.object, item.controller, AbilityRef::delayed(item.ability.card, 1), Target::None); }
g.end_sentence();
hoisted_12.extend(discarded_8.clone());
hoisted_13.extend(selected_7.clone());
hoisted_14.extend({ let q: &dyn Query = g; discarded_8.clone() });
}
2 => {
g.end_sentence();
}
_ => {}
}
g.end_sentence();
 Ok(()) }
static ABILITY_1_TARGETS: &[cantrip_api::TargetSpec] = &[cantrip_mechanics::targets::ANY];
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let source_15 = { let q: &dyn Query = g; Some(asker.source) }; let amount_15 = { let q: &dyn Query = g; (2i64).max(0) as u16 }; let recipients_15 = { let q: &dyn Query = g; t_0.clone() }; if let Some(source) = source_15 { let batch: Vec<(ObjectId, Target, u16)> = recipients_15.iter().map(|t| (source, *t, amount_15)).collect(); g.damage_together(&batch); }
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_1_16(q: &dyn Query, asker: &Asker, id: ObjectId) -> bool { q.chars(id).is(CardType::Artifact) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }
planned_item!("engine-no-choice-between-the-costs-of-an-ability", "(kind:Activated,args:(origin:Ordinary,cost:(parts:[Mana([Generic(3)]),TapSymbol,Choice([(parts:[Sacrifice(objects:(kind:Battlefield,args:(kind:CardType,args:Artifact)),choose:Some(Exactly((kind:Constant,args:1))))]),(parts:[Discard(count:(kind:Constant,args:1),filter:(kind:Not,args:(kind:CardType,args:Land)),random:false)])])]),limits:(once:false,once_each_turn:false,sorcery:false,condition:None,activators:None,keyword:None,modifiers:[]),announcement:(targets:[(slot:(0),kind:DamageRecipient,count:Exactly((kind:Constant,args:1)),filter:(kind:Any),players:None,distribute:None,constraints:[],chooser:None)],modes:None),body:(instructions:[(kind:Effect,args:(result:(0),operation:(kind:DealDamage,args:(source:(kind:Source),recipients:(kind:Targets,args:(0)),amount:(kind:Constant,args:2),preventable:true))))])))");

pub static CARD: CardDef = CardDef { name: "Bullseye, Death Dealer", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [Some((Mana::B, Mana::R)),None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Human,Subtype::Assassin,Subtype::Villain]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(2), toughness: Some(3), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ZONE_CHANGE.0), ability_0_event), ability_0, "When Bullseye enters, you may sacrifice an artifact or discard a nonland card. When you do, Bullseye deals 2 damage to any target.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[delayed_trigger(reflexive_3, "When you do, Bullseye deals 2 damage to any target.").targets(REFLEXIVE_3_TARGETS),delayed_trigger(reflexive_10, "When you do, Bullseye deals 2 damage to any target.").targets(REFLEXIVE_10_TARGETS)], enters: None, modal: None, enchant: None, labels: &["Sacrifice an artifact", "Discard a nonland card", "Neither"], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
