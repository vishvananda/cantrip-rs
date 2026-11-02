// Retriever Phoenix
// status: planned
// requires: source-zone scope on Replacement
// Flying, haste
// When this creature enters, if you cast it, learn. (You may reveal a Lesson card you own from outside the game and put it into your hand, or discard a card to draw a card.)
// As long as this card is in your graveyard, if you would learn, you may instead return this card to the battlefield.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_2_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ZoneChange { old, new, from, to, .. } = event { let id = *new; let subject = Target::Object(id); let data = [0u32, u32::MAX]; if to.is(ZoneKind::Battlefield) && Some(asker.source) == Some(id) && subject.object().is_some_and(|candidate: ObjectId| cast_at(q, event, candidate).is_some_and(|cast| Some(asker.controller).into_iter().collect::<Vec<PlayerId>>().contains(&cast.player))) { return Some(Fired { subject, data }); } }
 None }
fn ability_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller); if !{ let q: &dyn Query = g; item.subject.object().is_some_and(|candidate: ObjectId| cast_of(q, candidate).is_some_and(|cast| Some(asker.controller).into_iter().collect::<Vec<PlayerId>>().contains(&cast.player))) } { return Ok(()); } let asker = &Asker::new(item.object, item.controller);
cantrip_mechanics::learn::learn(g, item)?;
g.end_sentence();
 Ok(()) }
#[allow(unused_variables, unused_mut)] fn ability_3_stated_then_steps(g: &mut dyn Game, asker: &Asker, subject: Target, data: [u32; 2]) -> Result<(), Suspend> { let decider_2 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") };
match planned_value!("depends on an instruction with a gap", 0u8 => { let card_2 = g.card_id(&CARD); cantrip_mechanics::choose::one_of(g, decider_2, asker.source, vec![Label::Card(card_2, 0), Label::Card(card_2, 1)])? }) {
0 => {
let player_4 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") };
let objects_4 = ({ let q: &dyn Query = g; Some(asker.source).into_iter().collect::<Vec<ObjectId>>() }).into_iter().filter(|&o: &ObjectId| !g.state().obj(o).token).collect::<Vec<ObjectId>>(); let moved_4 = { let rows_4: Vec<(ObjectId, PlayerId, bool)> = objects_4.iter().map(|&o| (o, player_4, false)).collect(); let came_4 = g.put_all_onto_battlefield_as(&rows_4)?; came_4.into_iter().filter(|&o| g.state().obj(o).zone.is(ZoneKind::Battlefield)).collect::<Vec<ObjectId>>() };
}
_ => {
planned!("keyword-action-learn-by-another-than-its-controller", Keyword, "action" = "\"Learn\"", "actor" = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }, "amount" = "()", "object" = "()", "other" = "()", "subtype" = "()");
}
}
 Ok(()) }
fn cast_of<'a>(q: &'a dyn Query, id: ObjectId) -> Option<&'a Cast> { let records = q.history().all(); let spell = records.iter().rev().find_map(|r| match r.event { Event::ZoneChange { old, new, reason: ZoneReason::Resolve, .. } if new == id => Some(old), _ => None }).unwrap_or(id); records.iter().rev().find_map(|r| match &r.event { Event::Cast(cast) if cast.spell == spell => Some(cast), _ => None }) }
#[allow(dead_code)] fn cast_at<'a>(q: &'a dyn Query, event: &Event, id: ObjectId) -> Option<&'a Cast> { match event { Event::ZoneChange { old, new, reason: ZoneReason::Resolve, .. } if *new == id => cast_of(q, *old), _ => cast_of(q, id) } }
planned_item!("replacement-of-learn" => callbacks: #[allow(unused_variables)] fn ability_3_stated_applies(q: &dyn Query, asker: &Asker, proposal: &Proposal) -> bool { if let Proposal::Learn(p) = proposal { let subject = Target::Player(p.player); let data = [u32::MAX; 2]; if Some(asker.controller) == Some(p.player) { return true; } }
 false }
#[allow(unused_variables, unused_mut)] fn ability_3_stated_apply(q: &dyn Query, asker: &Asker, proposal: &mut Proposal) { let seen = *proposal; if let Proposal::Learn(seen) = &seen { let p = seen; let subject = Target::Player(p.player); let data = [u32::MAX; 2]; if let Proposal::Learn(p) = proposal { p.replaced = true; } } }
#[allow(unused_variables)] fn ability_3_stated_then(g: &mut dyn Game, asker: &Asker, before: &Proposal, _: &Proposal) { if let Proposal::Learn(p) = before { let q: &dyn Query = g; let subject = Target::Player(p.player); let data = [u32::MAX; 2]; let _ = ability_3_stated_then_steps(g, asker, subject, data); } }
);
planned_item!("replacement-of-learn" => replacements: replacement(ReplacementKind::Learn, ability_3_stated_applies, ability_3_stated_apply, "As long as this card is in your graveyard, if you would learn, you may instead return this card to the battlefield.").then(ability_3_stated_then));
planned_item!("source-zone-scope-on-replacement", "(kind:Scoped,args:(zones:[Graveyard],condition:None,ability:(kind:Replacement,args:(active:Battlefield,event:Player(event:Learn,players:(kind:One,args:(kind:AbilityController))),body:(instructions:[(kind:Effect,args:(result:(0),operation:(kind:ModifyEvent,args:(change:Cancel)))),(kind:Effect,args:(result:(5),operation:(kind:ChooseAction,args:(actor:(kind:One,args:(kind:AbilityController)),options:[(kind:Program,args:(instructions:[(kind:Effect,args:(result:(1),operation:(kind:ModifyEvent,args:(change:Cancel)))),(kind:Effect,args:(result:(2),operation:(kind:Move,args:(objects:(kind:One,args:(kind:Source)),destination:Battlefield((kind:AbilityController)),entry:(tapped:false,counters:[],states:[]),face_down:false))))])),(kind:Program,args:(instructions:[(kind:Effect,args:(result:(3),operation:(kind:ModifyEvent,args:(change:Cancel)))),(kind:Effect,args:(result:(4),operation:(kind:Keyword,args:(actor:(kind:One,args:(kind:AbilityController)),action:Learn,object:None,other:None,amount:None,subtype:None))))]))],villainous:false))))])))))");

pub static CARD: CardDef = CardDef { name: "Retriever Phoenix", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [0, 0, 0, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Phoenix]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(2), toughness: Some(2), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Flying,Keyword::Haste]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ZONE_CHANGE.0), ability_2_event), ability_2, "When this creature enters, if you cast it, learn.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
