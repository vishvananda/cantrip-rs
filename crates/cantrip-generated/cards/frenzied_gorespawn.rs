// Frenzied Gorespawn
// status: planned
// requires: target cardinality PerPlayer
// When this creature enters, for each opponent, goad target creature that player controls. (Until your next turn, those creatures attack each combat if able and attack a player other than you if able.)
// Whenever one or more creatures attack one of your opponents, those creatures gain menace until end of turn.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::Attacks { attacker, defender, defending, .. } = event { let id = *attacker; let subject = Target::Object(id); let data = [u32::from(q.controller(id).0), u32::from(defending.0)]; if q.chars(id).is(CardType::Creature) && match *defender { Target::Player(p) => (Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>()).contains(&p), _ => false } { return Some(Fired { subject, data }); } }
 None }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let objects_0 = { let q: &dyn Query = g; (q.event_objects(item).to_vec()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let source_0 = { let q: &dyn Query = g; Some(asker.source) }; let duration_0 = Duration::EndOfTurn;
let kind_0_0_0 = { let q: &dyn Query = g; EffectKind::AddKeywords(Keywords::of(&[Keyword::Menace])) };
if let Some(source) = source_0 { cantrip_api::instruction(g, |g| { for &object in &objects_0 { g.add_effect(source, object, kind_0_0_0, duration_0);  } }); }
g.end_sentence();
 Ok(()) }
planned_item!("target-cardinality-per-player", "(kind:Triggered,args:(event:ZoneChange(objects:(kind:SameObject,args:(kind:Source)),from:None,to:Battlefield),intervening_if:None,limits:(per_turn:None,total:None),announcement:(targets:[(slot:(0),kind:Object((kind:Permanent,zones:[Battlefield])),count:UpTo((kind:CountPlayers,args:(kind:OpponentsOf,args:(kind:AbilityController)))),filter:(kind:All,args:[(kind:CardType,args:Creature),(kind:ControlledBy,args:(kind:OpponentsOf,args:(kind:AbilityController)))]),players:None,distribute:None,constraints:[DifferentControllers],chooser:None)],modes:None),body:(instructions:[(kind:Effect,args:(result:(0),operation:(kind:KeywordEach,args:(actor:(kind:One,args:(kind:AbilityController)),action:Goad,objects:(kind:Targets,args:(0)),until:None))))])))");

pub static CARD: CardDef = CardDef { name: "Frenzied Gorespawn", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [0, 0, 1, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Horror]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(4), toughness: Some(4), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ATTACKS.0), ability_1_event), ability_1, "Whenever one or more creatures attack one of your opponents, those creatures gain menace until end of turn.").targets(&[]).once_per_batch().batch_by_data()], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
