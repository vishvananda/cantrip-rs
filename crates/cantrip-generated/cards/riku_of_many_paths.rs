// Riku of Many Paths
// status: planned
// requires: event value outside its ability
// Whenever you cast a modal spell, choose up to X, where X is the number of times you chose a mode for that spell —
// • Exile the top card of your library. Until the end of your next turn, you may play it.
// • Put a +1/+1 counter on Riku. It gains trample until end of turn.
// • Create a 1/1 blue Bird creature token with flying.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::SpellCast { spell, player, .. } = event { let id = *spell; let subject = Target::Object(id); let data = [u32::from(player.0), u32::MAX]; if Some(asker.controller) == Some(*player) && q.card(q.chars(id).card).modal.is_some() { return Some(Fired { subject, data }); } }
 None }
fn ability_0_mode_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let objects_0 = { let q: &dyn Query = g; (((Some(asker.controller).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::library(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && true).collect::<Vec<ObjectId>>()).into_iter().take((1i64).max(0) as usize).collect::<Vec<ObjectId>>() }; let moved_0 = g.exile_all(&objects_0);
g.end_sentence();
let cards_1_0 = { let q: &dyn Query = g; (moved_0.clone()).get(0 as usize).copied().into_iter().collect::<Vec<ObjectId>>() }; let players_1_0 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }; cantrip_api::instruction(g, |g| { for &card in &cards_1_0 { if g.obj(card).zone.is(ZoneKind::Exile) { for &player in &players_1_0 { g.allow_play(card, player, PlayGrant { lands: true, expiry: Expiry::EndOfYourNextTurn, ..PlayGrant::CAST }); } } } });
g.end_sentence();
 Ok(()) }
fn ability_0_mode_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let mut counters_2: Vec<(ObjectId, CounterKind, i16)> = Vec::new();
let amount = { let q: &dyn Query = g; (1i64).max(0) as i16 }; for t in { let q: &dyn Query = g; (Some(asker.source).into_iter().collect::<Vec<ObjectId>>()).into_iter().map(Target::Object).collect::<Vec<Target>>() } { if let Target::Object(object) = t { counters_2.push((object, CounterKind::PlusOne, amount)); } }
let had_2: Vec<_> = counters_2.iter().map(|c| g.state().obj(c.0).counters(c.1)).collect(); g.add_counters_together(&counters_2); let applied_2: Vec<ObjectId> = counters_2.iter().zip(had_2).filter(|(c, had)| g.state().obj(c.0).counters(c.1) > *had).map(|(c, _)| c.0).collect();
g.end_sentence();
let objects_3 = { let q: &dyn Query = g; (Some(asker.source).into_iter().collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let source_3 = { let q: &dyn Query = g; Some(asker.source) }; let duration_3 = Duration::EndOfTurn;
let kind_3_0_0 = { let q: &dyn Query = g; EffectKind::AddKeywords(Keywords::of(&[Keyword::Trample])) };
if let Some(source) = source_3 { cantrip_api::instruction(g, |g| { for &object in &objects_3 { g.add_effect(source, object, kind_3_0_0, duration_3);  } }); }
g.end_sentence();
 Ok(()) }
pub mod token_5 {
#[allow(unused_imports)] use cantrip_api::*;

pub static CARD: CardDef = CardDef { name: token!(Bird).name, token_words: token!(Bird).token_words, oracle: false, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Bird]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: Some(ColorSet(2)), power: Some(1), toughness: Some(1), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Flying]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];

}
fn ability_0_mode_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let players_4 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }; let mut tokens_4 = Vec::new();
let count_4_0 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let def_4_0 = g.card_id(&token_5::CARD);
for player in players_4 {
tokens_4.extend(std::iter::repeat((def_4_0,player)).take(count_4_0 as usize));
} let words_4 = g.words(asker.source);
let created_4 = g.create_tokens_worded(&tokens_4, false, None, words_4)?;
g.end_sentence();
 Ok(()) }
planned_item!("event-value-outside-its-ability", "(kind:Triggered,args:(event:Spell(event:Cast,players:(kind:One,args:(kind:AbilityController)),spells:(kind:State,args:Modal)),intervening_if:None,limits:(per_turn:None,total:None),announcement:(targets:[],modes:Some((minimum:(kind:Constant,args:0),maximum:(kind:ModesChosen,args:(kind:EventSubject)),repeat:false,modes:[(announcement:(targets:[],modes:None),body:(instructions:[(kind:Effect,args:(result:(0),operation:(kind:Move,args:(objects:(kind:Take,args:(objects:(kind:Zone,args:(scope:Players(zone:Library,players:(kind:One,args:(kind:AbilityController))),kind:Card,filter:(kind:Any))),count:(kind:Constant,args:1),from:Top)),destination:Exile,entry:(tapped:false,counters:[],states:[]),face_down:false)))),(kind:Effect,args:(result:(1),operation:(kind:CreateContinuousEffect,args:(source:(kind:Source),affected:Players((kind:One,args:(kind:AbilityController))),changes:[(kind:PlayerRule,args:PlayPermission(cards:(kind:One,args:(kind:Result,args:(result:(0),phase:After,index:0))),from:Exile,action:Either,filter:(kind:Any)))],duration:UntilEndOfNextTurn((kind:AbilityController)),evaluation:LiveObjects))))]),weight:1,requires:None),(announcement:(targets:[],modes:None),body:(instructions:[(kind:Effect,args:(result:(0),operation:(kind:PutCounters,args:(placements:[(recipients:(kind:Objects,args:(kind:One,args:(kind:Source))),kind:PowerToughness(1,1),amount:(kind:Constant,args:1),selection:None)])))),(kind:Effect,args:(result:(1),operation:(kind:CreateContinuousEffect,args:(source:(kind:Source),affected:Objects((kind:One,args:(kind:Source))),changes:[(kind:GrantKeyword,args:(kind:Trample))],duration:EndOfTurn,evaluation:Snapshot))))]),weight:1,requires:None),(announcement:(targets:[],modes:None),body:(instructions:[(kind:Effect,args:(result:(0),operation:(kind:CreateTokens,args:(players:(kind:One,args:(kind:AbilityController)),tokens:[(amount:(kind:Constant,args:1),words:CreatingEffect,recipe:Defined((name:FromSubtypes,colors:[Blue],chosen_color:false,chosen_creature_type:false,supertypes:[],card_types:[Creature],subtypes:[Bird],power_toughness:Some(((kind:Constant,args:1),(kind:Constant,args:1))),abilities:[(kind:Keyword,args:(kind:Flying))],abilities_of:None)))],entry:(tapped:false,counters:[],states:[])))))]),weight:1,requires:None)],implied:false,unchosen:None,surcharge:None,distinct_targets:false,chooser:None,random:false,budget:None,may_decline:false))),body:(instructions:[])))");

pub static CARD: CardDef = CardDef { name: "Riku of Many Paths", token_words: &[], oracle: true, cost: ManaCost { generic: 0, x: 0, symbols: [0, 1, 0, 1, 1, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Human,Subtype::Wizard]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(3), toughness: Some(3), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[&token_5::CARD];
